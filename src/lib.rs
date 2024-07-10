#![allow(non_snake_case, non_camel_case_types, unused)]
extern crate jni;
use jni::sys::{jint, JNI_OK};

#[no_mangle]
pub extern "system" fn Agent_OnLoad(
    vm: jni::JavaVM,
    options: *mut std::os::raw::c_void,
    reserved: *mut std::os::raw::c_void,
) -> jint {
    let env = vm.get_env().expect("Failed to get JNIEnv");
    let jvmti = vm.get_env();
    println!("Agent_OnLoad called!");
    JNI_OK
}

#[no_mangle]
pub extern "system" fn Agent_OnAttach(vm: jni::JavaVM, options: *mut std::os::raw::c_void, reserved: *mut std::os::raw::c_void) -> jint {
    let env = vm.get_env().expect("Failed to get JNIEnv");
    let jvmti = vm.get_env();
    println!("Agent_OnAttach called!");
    JNI_OK
}

#[no_mangle]
pub extern "system" fn Agent_OnUnload(vm: jni::JavaVM) {
    println!("Agent_OnUnload called!");
}
