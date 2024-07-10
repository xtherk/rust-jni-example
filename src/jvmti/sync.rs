#![allow(non_snake_case, non_camel_case_types, dead_code)]

use crate::jvmti::{jvmti_sys::jvmtiEnv, jvmti_wrapper::JvmtiEnv};
use jni::JavaVM;
use std::os::raw::{c_int, c_void};
use std::ptr;

pub trait JvmtiSupplier {
    fn get_jvmti_env(&self, jvmti_version: c_int) -> JvmtiEnv;
}

impl JvmtiSupplier for JavaVM {
    fn get_jvmti_env(&self, jvmti_version: c_int) -> JvmtiEnv {
        let vm = self.get_java_vm_pointer();

        let mut jvmti: *mut c_void = ptr::null_mut();

        unsafe {
            (**vm).GetEnv.unwrap()(vm, &mut jvmti, jvmti_version);

            JvmtiEnv::from(jvmti as *mut jvmtiEnv)
        }
    }
}
