#![allow(non_snake_case, non_camel_case_types, dead_code)]
use jni::{
    objects::JClass,
    strings::JNIString,
    sys::{self, jclass, JNIEnv},
};
use libloading::Library;
use std::{ffi::CStr, os::raw::c_char};

pub type FindClassFromBootLoaderFn = unsafe extern "system" fn(*mut JNIEnv, *mut c_char) -> jclass;
pub type GetClassNameUTFFn = unsafe extern "system" fn(*mut JNIEnv, *mut jclass) -> *mut c_char;

pub struct JvmInternal {
    find_class_from_bootloader: FindClassFromBootLoaderFn,
    get_class_name_utf: GetClassNameUTFFn,
}

impl JvmInternal {
    pub fn new(libjvm_path: &str) -> Result<JvmInternal, libloading::Error> {
        let libjvm = match unsafe { Library::new(libjvm_path) } {
            Ok(lib) => lib,
            Err(e) => return Err(e),
        };

        let internal = unsafe {
            let function_ptr = libjvm.get(b"JVM_FindClassFromBootLoader\0").unwrap();
            let find_class_from_bootloader: FindClassFromBootLoaderFn = *function_ptr;

            let function_ptr = libjvm.get(b"JVM_GetClassNameUTF\0").unwrap();
            let get_class_name_utf: GetClassNameUTFFn = *function_ptr;
            JvmInternal {
                find_class_from_bootloader,
                get_class_name_utf,
            }
        };
        Ok(internal)
    }

    /**
     * 获取name所对应的jclass
     */
    pub fn find_class_from_bootloader(
        self: &Self,
        penv: *mut sys::JNIEnv,
        name: &str,
    ) -> Option<JClass> {
        let name = JNIString::from(name);
        let clz = unsafe { (self.find_class_from_bootloader)(penv, name.as_ptr() as *mut c_char) };
        if clz.is_null() {
            None
        } else {
            let result = unsafe { JClass::from_raw(clz) };
            Some(result)
        }
    }

    /**
     * 获取jclass的类名
     */
    pub fn get_class_name_utf(self: &Self, penv: *mut sys::JNIEnv, cls: &jclass) -> Option<String> {
        let class_name = unsafe { (self.get_class_name_utf)(penv, *cls as *mut _) };
        if class_name.is_null() {
            None
        } else {
            let raw = unsafe { CStr::from_ptr(class_name) };
            Some(raw.to_str().unwrap().to_owned())
        }
    }
}
