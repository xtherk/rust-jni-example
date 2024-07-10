#![allow(non_snake_case, non_camel_case_types, dead_code)]
use jni::{
    objects::{JObject, JString},
    strings::JNIString,
    JNIEnv,
};
use log::debug;
use std::os::raw::c_char;

pub trait JNIEnvUtility {
    fn get_class_name(&mut self, class: &JObject) -> String;

    fn get_code_location(&mut self, protection_domain: &JObject) -> String;
}

impl JNIEnvUtility for JNIEnv<'_> {
    fn get_class_name(&mut self, class: &JObject) -> String {
        let name = self.call_method(&class, "getName", "()Ljava/lang/String;", &[]);
        let jname = name.unwrap().l().unwrap();
        let name = JString::from(jname);
        let java_name = self.get_string(&name).unwrap();
        return java_name.to_str().unwrap().to_string();
    }

    fn get_code_location(&mut self, protection_domain: &JObject) -> String {
        let result = self.call_method(
            protection_domain,
            "getCodeSource",
            "()Ljava/security/CodeSource;",
            &[],
        );

        let code_source = match result {
            Ok(v) => v,
            Err(e) => {
                debug!("Call getCodeSource Error. Cause({})", e.to_string());
                return String::new();
            }
        }
        .l()
        .unwrap();

        let url = self.call_method(code_source, "getLocation", "()Ljava/net/URL;", &[]);
        let url = match url {
            Ok(v) => v,
            Err(e) => {
                debug!("Call getLocation Error. Cause({})", e.to_string());
                return String::new();
            }
        }
        .l()
        .unwrap();

        let path = self.call_method(url, "getPath", "()Ljava/lang/String;", &[]);
        let path = match path {
            Ok(v) => v,
            Err(e) => {
                debug!("Call getPath Error. Cause({})", e.to_string());
                return String::new();
            }
        }
        .l()
        .unwrap();

        let jpath = JString::from(path);
        let result: String = self
            .get_string(&jpath)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        result
    }
}

pub unsafe fn to_mut_c_char(str: &str) -> *mut c_char {
    let name = JNIString::from(str);
    name.as_ptr() as _
}

pub fn read_to<T>(data: &[u8]) -> T
where
    T: std::ops::Add<Output = T> + From<u8> + Copy,
{
    let value = data.as_ptr() as *const T;
    unsafe { value.read_unaligned() }
}