use std::process;
use jni::objects::{JObjectArray, JString};
use jni::sys::jobjectArray;

use crate::jvmti::jvmti_wrapper::JvmtiEnv;
use log::{debug, error};

#[allow(unused)]
pub unsafe fn vm_param_check(jvmti: &JvmtiEnv, env: &mut jni::JNIEnv) {
    let array_args = {
        let management_factory = env
            .find_class("java/lang/management/ManagementFactory")
            .unwrap();

        let mxbean = env
            .call_static_method(
                management_factory,
                "getRuntimeMXBean",
                "()Ljava/lang/management/RuntimeMXBean;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        let list_args = env
            .call_method(mxbean, "getInputArguments", "()Ljava/util/List;", &[])
            .unwrap()
            .l()
            .unwrap();

        env.call_method(list_args, "toArray", "()[Ljava/lang/Object;", &[])
            .unwrap()
            .as_jni()
            .l as jobjectArray
    };

    let array_args = JObjectArray::from_raw(array_args);
    let array_size = env.get_array_length(&array_args).unwrap();

    let mut has_disable_attachmechanism = false;
    let illegal_params = vec!["-agentpath", "-agentlib", "-javaagent"];
    let disable_attachmechanism = "-XX:+DisableAttachMechanism";

    for i in 0..array_size {
        let arv = env.get_object_array_element(&array_args, i).unwrap();
        let arv = JString::from(arv);
        let option = env.get_string(&arv).unwrap();
        let opt_value = option.to_str().unwrap();
        debug!("VM option: {}", opt_value);

        for p in &illegal_params {
            if opt_value.contains(*p) {
                error!("Illegal param {}", opt_value);
                process::exit(1);
            }
        }

        if opt_value.eq(disable_attachmechanism) {
            has_disable_attachmechanism = true;
        }
    }

    if !has_disable_attachmechanism {
        error!("VM has no param -XX:+DisableAttachMechanism");
        process::exit(1);
    }
    debug!("Runtime vm param check pass!");
}

/**
 * We can check if some files have been tampered with
 */
#[inline(always)]
pub fn md5_check(java_home: &str) -> bool {
    true
}