use core::slice;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::io::{BufReader, Cursor, Read, Seek};
use std::os::raw::{c_char, c_uchar};
use std::path::Path;
use std::sync::Mutex;

use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;
use jni::{
    sys,
    sys::{jclass, jint, jobject},
};

use log::{debug, info, trace};
use once_cell::sync::Lazy;

use crate::jvmti::jvmti_sys::jvmtiEnv;
use crate::jvmti::jvmti_wrapper::JvmtiEnv;
use crate::utility::{self, JNIEnvUtility};

static LOADED_JAR_MAP: Lazy<Mutex<HashMap<String, bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static PRELOAD_CLASS_MAP: Lazy<Mutex<HashMap<String, Vec<u8>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));


const ENCRYPT_MAGIC_NUMBER : u32 = 0xDEADC0DE;

/**
 * We can perform some operations on the class in the class_hoke_event, 
 * such as bytecode enhancement or bytecode decryption.
 * 
 * This example uses class_hook_event to decrypt the class.
 * It supports two methods of bytecode decryption:
 *  1. Encrypting the class itself. 
 *      This method does not support annotation scanning by frameworks like Spring.
 * 
 *  2. Clearing the functions of the class and storing the original encrypted copy in META-INF/.classes/, 
 *      retaining fields, function signatures, and annotations. 
 *      This method allows annotation scanning by frameworks like Spring.
 */
#[allow(unused)]
pub unsafe extern "C" fn class_hook_event(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut sys::JNIEnv,
    class_being_redefined: jclass,
    loader: jobject,
    name: *const c_char,
    protection_domain: jobject,
    class_data_len: jint,
    class_data: *const c_uchar,
    new_class_data_len: *mut jint,
    new_class_data: *mut *mut c_uchar,
) {
    if name.is_null() {
        return;
    }

    let class_name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    let class_name = class_name.replace("/", ".");

    let mut env = JNIEnv::from_raw(jni_env).unwrap();
    let jvmti = JvmtiEnv::from(jvmti_env);

    let mut class_data = slice::from_raw_parts(class_data, class_data_len as usize).to_vec();
    let mut magic: [u8; 4] = [0; 4];
    let magic_number = &class_data[0..4];
    let magic_number = utility::read_to::<u32>(magic_number);

    let decrypter = |class_data: &mut [u8]| -> Vec<u8> {
        // Implement your own decryption logic here.
        class_data.to_owned()
    };

    if magic_number == ENCRYPT_MAGIC_NUMBER {
        // Encryption mode one, we directly decrypt it.
        let mut decrypt_class_data = decrypter(&mut class_data);
        let memory = jvmti.allocate(decrypt_class_data.len() as jlong);
        *new_class_data = memory.unwrap();
        *new_class_data_len = decrypt_class_data.len() as jint;
        std::ptr::copy_nonoverlapping(
            decrypt_class_data.as_ptr(),
            *new_class_data,
            decrypt_class_data.len(),
        );
        return;
    }

    if protection_domain.is_null() {
        return;
    }

    let protection_domain = unsafe { JObject::from_raw(protection_domain) };
    let class_location = env.get_code_location(&protection_domain);
    if class_location.is_empty() {
        return;
    }

    let path_decode = url_decode(class_location.as_str());
    let path = if path_decode.starts_with("file:/") {
        &path_decode[6..]
    } else if class_location.starts_with("/") {
        &path_decode[1..]
    } else {
        path_decode.as_str()
    };

    let path = match path.find(".jar") {
        Some(index) => &path[0..index + 4],
        None => {
            debug!("{} not belongs to jar", class_name);
            return;
        }
    };

    // If the JAR has not been parsed before, parse the JAR to find the encrypted class copies and decrypt them
    {
        let mut loaded_jar_map = LOADED_JAR_MAP.lock().unwrap();
        if !loaded_jar_map.contains_key(path) {
            loaded_jar_map.insert(path.to_owned(), true);

            let path = Path::new(path);
            let buffer = BufReader::new(fs::File::open(path).unwrap());
            debug!("Load jar {}", path.display());
            read_and_decrypt_jar_files(buffer);
        }
    }

    // The class files encrypted using encryption method two must have their decrypted data available
    {
        let class_map = PRELOAD_CLASS_MAP.lock().unwrap();
        if class_map.contains_key(&class_name) {
            let class_data = class_map.get(&class_name).unwrap();
            let memory = jvmti.allocate(class_data.len() as jlong);
            *new_class_data = memory.unwrap();
            *new_class_data_len = class_data.len() as jint;
            std::ptr::copy_nonoverlapping(class_data.as_ptr(), *new_class_data, class_data.len());
        }
    }

    // Unencrypted files do not require processing
    debug!("class_hook_event: {}", class_name);
}

fn url_decode(input: &str) -> String {
    let mut data = input.chars().peekable();
    let mut result = String::new();

    while let Some(c) = data.next() {
        if c == '+' {
            result.push(' ');
        } else if c == '%' {
            if let (Some(h1), Some(h2)) = (data.next(), data.next()) {
                if h1.is_ascii_hexdigit() && h2.is_ascii_hexdigit() {
                    let hex = format!("{}{}", h1, h2);
                    if let Ok(decoded_char) = u8::from_str_radix(&hex, 16) {
                        result.push(decoded_char as char);
                    } else {
                        result.push(c);
                        result.push(h1);
                        result.push(h2);
                    }
                } else {
                    result.push(c);
                    result.push(h1);
                    if let Some(h) = data.peek() {
                        result.push(*h);
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/**
 * some.jar
 * |__META-INF
 *    |__.classes
 * 
 * Read the original class file from the /META-INF/.classes/ folder and decrypt it
 */
#[allow(unused)]
pub fn read_and_decrypt_jar_files<R: Read + Seek>(reader: R) -> bool {
    const ZIP_CLASS_PATH: &str = "META-INF/.classes/";
    const ZIP_CLASS_PATH_LEN: usize = ZIP_CLASS_PATH.len();

    let mut archive = zip::ZipArchive::new(reader).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if file.is_file() {
            let file_name = file.name().to_owned();

            if file_name.ends_with(".jar") {
                debug!("Processing internal jar {}.", file_name);
                let mut data: Vec<u8> = Vec::new();
                let result = file.read_to_end(&mut data);
                if let Err(e) = result {
                    debug!("Reading jar file <{}> error.", &file_name);
                    continue;
                }

                let cursor = Cursor::new(data);
                let buffer = BufReader::new(cursor);
                read_and_decrypt_jar_files(buffer);
                continue;
            }

            if !file_name.starts_with(ZIP_CLASS_PATH) {
                continue;
            }

            let class_name = file_name[ZIP_CLASS_PATH_LEN..].replace("/", ".");
            let mut magic_number: [u8; 4] = [0; 4];
            match file.read(&mut magic_number) {
                Ok(_) => {
                    let magic_number = utility::read_to::<u32>(&magic_number);
                    if magic_number != ENCRYPT_MAGIC_NUMBER {
                        debug!("The file<{}> has been damaged.", &class_name);
                            continue;
                    }
                }
                Err(e) => {
                    debug!(
                        "Read file<{}> magic error. Cause: ({})",
                        &class_name,
                        e.to_string()
                    );
                    continue;
                }
            }

            let decrypter = |class_data: &mut [u8]| -> Vec<u8> {
                // Implement your own decryption logic here.
                class_data.to_owned()
            };

            let decrypt_class_data = {
                let mut data: Vec<u8> = Vec::new();
                let result = file.read_to_end(&mut data);
                if let Err(e) = result {
                    debug!("Reading class error {}", &class_name);
                    continue;
                }
                decrypter(&mut data)
            };

            // The purpose is to narrow the scope, allowing CLASS_MAP to automatically release the lock.
            {
                let mut class_map = PRELOAD_CLASS_MAP.lock().unwrap();
                if class_map.contains_key(&class_name) {
                    trace!("Overwrite class: {}", class_name);
                    class_map.remove(&class_name);
                }
                class_map.insert(class_name.clone(), decrypt_class_data);
                trace!("Preloaded class: {}", class_name);
            }
        }
    }
    true
}
