use std::os::raw::c_int;
use std::path::PathBuf;
use std::{env, process};

use env_logger::Builder;
use errors::GeneralError;
use jni::errors::StartJvmError;
use jni::objects::{AsJArrayRaw, JClass, JObject, JObjectArray};

use jni::sys::jsize;
use jni::{objects::JValue, InitArgsBuilder, JNIEnv};

use config::Config;
use log::{debug, error, info, LevelFilter};
use rustop::{opts, Error};

use crate::jvm::jvm_internal::JvmInternal;
use crate::jvmti::capabilities::{
    self, CAN_GENERATE_ALL_CLASS_HOOK_EVENTS, CAN_GENERATE_MONITOR_EVENTS,
};
use crate::jvmti::jvmti_sys::{jvmtiCapabilities, jvmtiEventMode, JVMTI_VERSION};
use crate::jvmti::jvmti_wrapper::JvmtiEnv;
use crate::jvmti::{jvmti_sys::jvmtiEventCallbacks, sync::JvmtiSupplier};

mod check;
mod config;
mod errors;
mod hook;
mod jvm;
mod jvmti;
mod utility;

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

#[derive(Default)]
struct Args {
    pub debug: bool,
    pub verbose: bool,
    pub args: Option<String>,
}

fn parse_params() -> Args {
    let result = opts! {
        help(false);
        auto_shorts(false);
        opt debug:bool=false, short: 'd';
        opt verbose:bool=false, short: 'v';
        opt args:Option<String>;
    }
    .parse();

    match result {
        Err(Error::Unknown(_)) => Default::default(),
        Err(_) => process::exit(1),
        Ok((p, _)) => Args {
            debug: p.debug,
            verbose: p.verbose,
            args: p.args,
        },
    }
}

fn main() {
    let params = parse_params();
    log_init(&params);

    let config_path = "config.ini";
    let config: Option<Config> = Config::load(config_path);
    let config = match config {
        Some(value) => value,
        None => {
            error!("Failed to load config file {}", config_path);
            process::exit(1);
        }
    };

    env::set_var("JAVA_HOME", &config.java_home);
    let jvm_path = {
        let jvmdyn = java_locator::get_jvm_dyn_lib_file_name();
        let libjvm_path = [
            java_locator::locate_jvm_dyn_library()
                .map_err(StartJvmError::NotFound).unwrap_or_default()
                .as_str(), jvmdyn
        ]
        .iter()
        .collect::<PathBuf>();

        if !libjvm_path.is_file() {
            error!("The {} file cannot be located", jvmdyn);
            process::exit(1);
        }
        libjvm_path.display().to_string()
    };

    let jvm = match create_javavm(&config) {
        Ok(vm) => vm,
        Err(e) => {
            error!("Failed to create java vm. Cause: ({})", e.to_string());
            process::exit(1);
        }
    };
    let mut env = jvm.attach_current_thread().unwrap();
    let jvmti = jvm.get_jvmti_env(JVMTI_VERSION as c_int);

    unsafe { check::vm_param_check(&jvmti, &mut env) }

    let java_home = match jvmti.get_system_property("java.home") {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e.to_string());
            process::exit(1);
        }
    };
    debug!("java_home: {}", java_home);

    let check = check::md5_check(java_home.as_str());
    if !check {
        error!("Environmental anomaly");
        if !DEBUG {
            process::exit(1);
        }
    }

    set_capabilities(&jvmti);
    set_event_callback(&jvmti);

    let internal = JvmInternal::new(jvm_path.as_str()).unwrap();
    let main_class = unsafe {
        let raw_class = load_main_class(&mut env, &internal, 2, &config.jar_path).as_raw();
        JObject::from_raw(raw_class)
    };

    // prepare main args
    let main_args_raw: *mut jni::sys::_jobject = {
        let cmd_args = params.args.unwrap_or_default();
        let main_args = load_main_args(&mut env, &internal, &config, cmd_args);
        main_args.as_jarray_raw()
    };

    // register native functions with the JVM for Java to call
    register_native_methods(&mut env);
    
    // call main method
    unsafe {
        let main_args = JObject::from_raw(main_args_raw);
        let main_args = JValue::Object(&main_args);
        let result = env.call_static_method(
            JClass::from(main_class),
            "main",
            "([Ljava/lang/String;)V",
            &[main_args],
        );
        if let Err(e) = result {
            debug!("error: {}", e.to_string());
        }
    }

    unsafe {
        jvm.detach_current_thread();
        let _ = jvm.destroy();
    };
}

fn create_javavm(config: &Config) -> Result<jni::JavaVM, GeneralError> {
    // I don't want others to use the agent to make modifications to my program
    let illegal_params = ["-agentpath", "-agentlib", "-javaagent"];
    let args = config.jvm_args.as_str();
    let options = args.split(' ');

    let mut class_path = String::new();
    class_path.push_str("-Djava.class.path=");
    class_path.push_str(config.jar_path.as_str());

    let mut jvm_args_builder = InitArgsBuilder::new()
        .option(class_path.as_str())
        .option("-Djava.library.path=.")
        .option("-Dsun.java.launcher=VM_STARTUP")
        .option("-XX:+DisableAttachMechanism");
    // .option("-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005");

    for option in options {
        if option.is_empty() {
            continue;
        }

        let skip = illegal_params.iter().any(|p| option.contains(p));
        if skip {
            debug!("skil option: {}", option);
            continue;
        }
        jvm_args_builder = jvm_args_builder.option(option);
    }

    let vm_result = jni::JavaVM::new(
        jvm_args_builder
            .version(jni::JNIVersion::V2)
            .ignore_unrecognized(false)
            .build()
            .unwrap(),
    );

    match vm_result {
        Ok(vm) => Ok(vm),
        Err(e) => Err(GeneralError::StartJvmError(e)),
    }
}

fn load_main_args<'a>(
    env: &'a mut JNIEnv,
    internal: &JvmInternal,
    config: &Config,
    cmd_args: String,
) -> JObjectArray<'a> {
    let mut items = Vec::new();
    if !cmd_args.is_empty() {
        let args = cmd_args.split(' ');
        args.for_each(|arg| items.push(arg));
    }

    let args = config.main_args.split(' ');
    args.for_each(|arg| items.push(arg));

    let string_class = internal
        .find_class_from_bootloader(env.get_raw(), "java/lang/String")
        .unwrap();

    let null_value = JObject::null();
    let init_element = JValue::Object(&null_value);
    let array = env.new_object_array(
        items.len() as jsize,
        string_class,
        init_element.l().unwrap(),
    );
    let array = array.unwrap();
    for (i, item) in items.iter().enumerate() {
        let item = env.new_string(item).unwrap();
        let _ = env.set_object_array_element(&array, i as i32, &item);
    }
    array
}

fn load_main_class<'a>(
    env: &'a mut JNIEnv,
    internal: &JvmInternal,
    mode: i32,
    name: &str,
) -> JObject<'a> {
    let launcher_helper =
        internal.find_class_from_bootloader(env.get_raw(), "sun/launcher/LauncherHelper");

    let launcher_helper = match launcher_helper {
        None => {
            error!("jvm error");
            process::exit(1);
        }
        Some(v) => v,
    };

    let main_class = unsafe {
        let jname = env.new_string(name);
        let name = JObject::from_raw(jname.unwrap().as_raw());

        let pname = JValue::Object(&name);
        let print_to_stderr = JValue::from(true);
        let mode = JValue::from(mode);

        env.call_static_method(
            launcher_helper,
            "checkAndLoadMain",
            "(ZILjava/lang/String;)Ljava/lang/Class;",
            &[print_to_stderr, mode, pname],
        )
        .unwrap()
        .l()
        .unwrap()
    };

    main_class
}

fn set_capabilities(jvmti: &JvmtiEnv) {
    let mut capabilities = jvmtiCapabilities::default();
    let _ = capabilities::set_capability(&mut capabilities, CAN_GENERATE_ALL_CLASS_HOOK_EVENTS);
    let _ = capabilities::set_capability(&mut capabilities, CAN_GENERATE_MONITOR_EVENTS);
    let result = jvmti.add_capabilities(capabilities);
    if let Err(e) = result {
        error!("Failed to add capabilities. Cause: ({})", e.to_string());
        process::exit(1);
    }
}

fn set_event_callback(jvmti: &JvmtiEnv) {
    let callbacks = jvmtiEventCallbacks {
        ClassFileLoadHook : Some(hook::class_hook_event),
        .. Default::default()
    };
    
    let result = jvmti.set_event_callbacks(callbacks);
    if let Err(e) = result {
        error!("Failed to set eventcallback. Cause: ({})", e.to_string());
        process::exit(1);
    }

    let result = jvmti.set_event_notification_mode(
        jvmtiEventMode::JVMTI_ENABLE,
        jvmti::jvmti_sys::jvmtiEvent::JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
        std::ptr::null_mut(),
    );
    if let Err(e) = result {
        error!(
            "Failed to set event_notification_mode. Cause: ({})",
            e.to_string()
        );
        process::exit(1);
    }
}

fn log_init(args: &Args) {
    use chrono::Local;
    use std::io::Write;

    let debug = args.debug;
    let verbose = args.verbose;

    let level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else if debug && verbose {
        LevelFilter::Trace
    } else if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    Builder::new()
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let level_name = record.level().as_str();
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            writeln!(
                buf,
                "[{timestamp}] {level_style}{level_name}{level_style:#} {}",
                record.args()
            )
        })
        .filter_level(level)
        .write_style(env_logger::WriteStyle::Always)
        .format_timestamp_millis()
        .init();
}


 /**
  * a jni example
  */
fn register_native_methods(env: &mut JNIEnv) {
    use jni::NativeMethod;
    use jni::strings::JNIString;
    use jni::objects::JString;

    #[no_mangle]
    #[allow(unused_mut)]
    pub extern "system" fn Main_test(mut env: JNIEnv, _class: JObject, text: JString) {
        let text: String = env.get_string(&text).unwrap().into();
        println!("hello, I am jni : {text}");
    }

    let fn_ptr = Main_test as *mut std::ffi::c_void;
    let methods = NativeMethod {
        name: JNIString::from("test"),
        sig: JNIString::from("(Ljava/lang/String;)V"),
        fn_ptr,
    };

    let clz = env.find_class("Main").unwrap();
    let result = env.register_native_methods(clz, &[methods]);

    match result {
        Ok(_) => info!("JNI function registered successfully"),
        Err(e) => error!("JNI function registered failed, reason: {}", e.to_string()),
    }
}