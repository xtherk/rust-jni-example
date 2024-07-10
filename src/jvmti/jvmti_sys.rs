// Modified from https://github.com/sfackler/rust-jni-sys to clean up naming, use jni-sys, and get rid of libc
// rust wrapper for jvmti.h
// copy from https://github.com/cretz/stackparam/blob/master/src/jvmti_sys/mod.rs
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use jni::sys::{
    jboolean, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jmethodID, jobject, jvalue,
    JNIEnv, JNINativeInterface_,
};
use std::{
    mem,
    os::raw::{c_char, c_int, c_uchar, c_uint, c_void},
};

pub const JVMTI_VERSION_1: c_uint = 805371904;
pub const JVMTI_VERSION_1_0: c_uint = 805371904;
pub const JVMTI_VERSION_1_1: c_uint = 805372160;
pub const JVMTI_VERSION_1_2: c_uint = 805372416;
pub const JVMTI_VERSION_9: c_uint = 805896192;
pub const JVMTI_VERSION_11: c_uint = 806027264;
pub const JVMTI_VERSION: c_int = 806420480;

pub type jvmtiEnv = *const jvmtiInterface_1;
pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;
pub enum _jrawMonitorID {}
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = JNINativeInterface_;

pub const JVMTI_THREAD_STATE_ALIVE: c_uint = 1;
pub const JVMTI_THREAD_STATE_TERMINATED: c_uint = 2;
pub const JVMTI_THREAD_STATE_RUNNABLE: c_uint = 4;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: c_uint = 1024;
pub const JVMTI_THREAD_STATE_WAITING: c_uint = 128;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: c_uint = 16;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: c_uint = 32;
pub const JVMTI_THREAD_STATE_SLEEPING: c_uint = 64;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: c_uint = 256;
pub const JVMTI_THREAD_STATE_PARKED: c_uint = 512;
pub const JVMTI_THREAD_STATE_SUSPENDED: c_uint = 1048576;
pub const JVMTI_THREAD_STATE_INTERRUPTED: c_uint = 2097152;
pub const JVMTI_THREAD_STATE_IN_NATIVE: c_uint = 4194304;
pub const JVMTI_THREAD_STATE_VENDOR_1: c_uint = 268435456;
pub const JVMTI_THREAD_STATE_VENDOR_2: c_uint = 536870912;
pub const JVMTI_THREAD_STATE_VENDOR_3: c_uint = 1073741824;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: c_uint = 1207;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: c_uint = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: c_uint = 2;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: c_uint = 5;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: c_uint = 1025;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: c_uint = 145;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: c_uint = 161;

pub const JVMTI_THREAD_MIN_PRIORITY: c_uint = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: c_uint = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: c_uint = 10;

pub const JVMTI_HEAP_FILTER_TAGGED: c_uint = 4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: c_uint = 8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: c_uint = 16;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: c_uint = 32;

pub const JVMTI_VISIT_OBJECTS: c_uint = 256;
pub const JVMTI_VISIT_ABORT: c_uint = 32768;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiHeapReferenceKind {
    JVMTI_HEAP_REFERENCE_CLASS = 1,
    JVMTI_HEAP_REFERENCE_FIELD = 2,
    JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_HEAP_REFERENCE_CLASS_LOADER = 4,
    JVMTI_HEAP_REFERENCE_SIGNERS = 5,
    JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_HEAP_REFERENCE_INTERFACE = 7,
    JVMTI_HEAP_REFERENCE_STATIC_FIELD = 8,
    JVMTI_HEAP_REFERENCE_CONSTANT_POOL = 9,
    JVMTI_HEAP_REFERENCE_SUPERCLASS = 10,
    JVMTI_HEAP_REFERENCE_JNI_GLOBAL = 21,
    JVMTI_HEAP_REFERENCE_SYSTEM_CLASS = 22,
    JVMTI_HEAP_REFERENCE_MONITOR = 23,
    JVMTI_HEAP_REFERENCE_STACK_LOCAL = 24,
    JVMTI_HEAP_REFERENCE_JNI_LOCAL = 25,
    JVMTI_HEAP_REFERENCE_THREAD = 26,
    JVMTI_HEAP_REFERENCE_OTHER = 27,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiPrimitiveType {
    JVMTI_PRIMITIVE_TYPE_BOOLEAN = 90,
    JVMTI_PRIMITIVE_TYPE_BYTE = 66,
    JVMTI_PRIMITIVE_TYPE_CHAR = 67,
    JVMTI_PRIMITIVE_TYPE_SHORT = 83,
    JVMTI_PRIMITIVE_TYPE_INT = 73,
    JVMTI_PRIMITIVE_TYPE_LONG = 74,
    JVMTI_PRIMITIVE_TYPE_FLOAT = 70,
    JVMTI_PRIMITIVE_TYPE_DOUBLE = 68,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiHeapObjectFilter {
    JVMTI_HEAP_OBJECT_TAGGED = 1,
    JVMTI_HEAP_OBJECT_UNTAGGED = 2,
    JVMTI_HEAP_OBJECT_EITHER = 3,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiHeapRootKind {
    JVMTI_HEAP_ROOT_JNI_GLOBAL = 1,
    JVMTI_HEAP_ROOT_SYSTEM_CLASS = 2,
    JVMTI_HEAP_ROOT_MONITOR = 3,
    JVMTI_HEAP_ROOT_STACK_LOCAL = 4,
    JVMTI_HEAP_ROOT_JNI_LOCAL = 5,
    JVMTI_HEAP_ROOT_THREAD = 6,
    JVMTI_HEAP_ROOT_OTHER = 7,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiObjectReferenceKind {
    JVMTI_REFERENCE_CLASS = 1,
    JVMTI_REFERENCE_FIELD = 2,
    JVMTI_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_REFERENCE_CLASS_LOADER = 4,
    JVMTI_REFERENCE_SIGNERS = 5,
    JVMTI_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_REFERENCE_INTERFACE = 7,
    JVMTI_REFERENCE_STATIC_FIELD = 8,
    JVMTI_REFERENCE_CONSTANT_POOL = 9,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiIterationControl {
    JVMTI_ITERATION_CONTINUE = 1,
    JVMTI_ITERATION_IGNORE = 2,
    JVMTI_ITERATION_ABORT = 0,
}

pub const JVMTI_CLASS_STATUS_VERIFIED: c_uint = 1;
pub const JVMTI_CLASS_STATUS_PREPARED: c_uint = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED: c_uint = 4;
pub const JVMTI_CLASS_STATUS_ERROR: c_uint = 8;
pub const JVMTI_CLASS_STATUS_ARRAY: c_uint = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE: c_uint = 32;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiEventMode {
    JVMTI_ENABLE = 1,
    JVMTI_DISABLE = 0,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiParamTypes {
    JVMTI_TYPE_JBYTE = 101,
    JVMTI_TYPE_JCHAR = 102,
    JVMTI_TYPE_JSHORT = 103,
    JVMTI_TYPE_JINT = 104,
    JVMTI_TYPE_JLONG = 105,
    JVMTI_TYPE_JFLOAT = 106,
    JVMTI_TYPE_JDOUBLE = 107,
    JVMTI_TYPE_JBOOLEAN = 108,
    JVMTI_TYPE_JOBJECT = 109,
    JVMTI_TYPE_JTHREAD = 110,
    JVMTI_TYPE_JCLASS = 111,
    JVMTI_TYPE_JVALUE = 112,
    JVMTI_TYPE_JFIELDID = 113,
    JVMTI_TYPE_JMETHODID = 114,
    JVMTI_TYPE_CCHAR = 115,
    JVMTI_TYPE_CVOID = 116,
    JVMTI_TYPE_JNIENV = 117,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiParamKind {
    JVMTI_KIND_IN = 91,
    JVMTI_KIND_IN_PTR = 92,
    JVMTI_KIND_IN_BUF = 93,
    JVMTI_KIND_ALLOC_BUF = 94,
    JVMTI_KIND_ALLOC_ALLOC_BUF = 95,
    JVMTI_KIND_OUT = 96,
    JVMTI_KIND_OUT_BUF = 97,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiTimerKind {
    JVMTI_TIMER_USER_CPU = 30,
    JVMTI_TIMER_TOTAL_CPU = 31,
    JVMTI_TIMER_ELAPSED = 32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiPhase {
    JVMTI_PHASE_ONLOAD = 1,
    JVMTI_PHASE_PRIMORDIAL = 2,
    JVMTI_PHASE_START = 6,
    JVMTI_PHASE_LIVE = 4,
    JVMTI_PHASE_DEAD = 8,
}

pub const JVMTI_VERSION_INTERFACE_JNI: c_uint = 0;
pub const JVMTI_VERSION_INTERFACE_JVMTI: c_uint = 805306368;

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: c_uint = 1879048192;
pub const JVMTI_VERSION_MASK_MAJOR: c_uint = 268369920;
pub const JVMTI_VERSION_MASK_MINOR: c_uint = 65280;
pub const JVMTI_VERSION_MASK_MICRO: c_uint = 255;

pub const JVMTI_VERSION_SHIFT_MAJOR: c_uint = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: c_uint = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: c_uint = 0;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiVerboseFlag {
    JVMTI_VERBOSE_OTHER = 0,
    JVMTI_VERBOSE_GC = 1,
    JVMTI_VERBOSE_CLASS = 2,
    JVMTI_VERBOSE_JNI = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiJlocationFormat {
    JVMTI_JLOCATION_JVMBCI = 1,
    JVMTI_JLOCATION_MACHINEPC = 2,
    JVMTI_JLOCATION_OTHER = 0,
}

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: c_uint = 1;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: c_uint = 2;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: c_uint = 4;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiError {
    JVMTI_ERROR_NONE = 0,
    JVMTI_ERROR_INVALID_THREAD = 10,
    JVMTI_ERROR_INVALID_THREAD_GROUP = 11,
    JVMTI_ERROR_INVALID_PRIORITY = 12,
    JVMTI_ERROR_THREAD_NOT_SUSPENDED = 13,
    JVMTI_ERROR_THREAD_SUSPENDED = 14,
    JVMTI_ERROR_THREAD_NOT_ALIVE = 15,
    JVMTI_ERROR_INVALID_OBJECT = 20,
    JVMTI_ERROR_INVALID_CLASS = 21,
    JVMTI_ERROR_CLASS_NOT_PREPARED = 22,
    JVMTI_ERROR_INVALID_METHODID = 23,
    JVMTI_ERROR_INVALID_LOCATION = 24,
    JVMTI_ERROR_INVALID_FIELDID = 25,
    JVMTI_ERROR_NO_MORE_FRAMES = 31,
    JVMTI_ERROR_OPAQUE_FRAME = 32,
    JVMTI_ERROR_TYPE_MISMATCH = 34,
    JVMTI_ERROR_INVALID_SLOT = 35,
    JVMTI_ERROR_DUPLICATE = 40,
    JVMTI_ERROR_NOT_FOUND = 41,
    JVMTI_ERROR_INVALID_MONITOR = 50,
    JVMTI_ERROR_NOT_MONITOR_OWNER = 51,
    JVMTI_ERROR_INTERRUPT = 52,
    JVMTI_ERROR_INVALID_CLASS_FORMAT = 60,
    JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION = 61,
    JVMTI_ERROR_FAILS_VERIFICATION = 62,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED = 63,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED = 64,
    JVMTI_ERROR_INVALID_TYPESTATE = 65,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED = 66,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED = 67,
    JVMTI_ERROR_UNSUPPORTED_VERSION = 68,
    JVMTI_ERROR_NAMES_DONT_MATCH = 69,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED = 70,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED = 71,
    JVMTI_ERROR_UNMODIFIABLE_CLASS = 79,
    JVMTI_ERROR_NOT_AVAILABLE = 98,
    JVMTI_ERROR_MUST_POSSESS_CAPABILITY = 99,
    JVMTI_ERROR_NULL_POINTER = 100,
    JVMTI_ERROR_ABSENT_INFORMATION = 101,
    JVMTI_ERROR_INVALID_EVENT_TYPE = 102,
    JVMTI_ERROR_ILLEGAL_ARGUMENT = 103,
    JVMTI_ERROR_NATIVE_METHOD = 104,
    JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED = 106,
    JVMTI_ERROR_OUT_OF_MEMORY = 110,
    JVMTI_ERROR_ACCESS_DENIED = 111,
    JVMTI_ERROR_WRONG_PHASE = 112,
    JVMTI_ERROR_INTERNAL = 113,
    JVMTI_ERROR_UNATTACHED_THREAD = 115,
    JVMTI_ERROR_INVALID_ENVIRONMENT = 116,
}
pub const JVMTI_ERROR_MAX: c_uint = 116;

pub const JVMTI_MIN_EVENT_TYPE_VAL: c_uint = 50;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum jvmtiEvent {
    JVMTI_EVENT_VM_INIT = 50,
    JVMTI_EVENT_VM_DEATH = 51,
    JVMTI_EVENT_THREAD_START = 52,
    JVMTI_EVENT_THREAD_END = 53,
    JVMTI_EVENT_CLASS_FILE_LOAD_HOOK = 54,
    JVMTI_EVENT_CLASS_LOAD = 55,
    JVMTI_EVENT_CLASS_PREPARE = 56,
    JVMTI_EVENT_VM_START = 57,
    JVMTI_EVENT_EXCEPTION = 58,
    JVMTI_EVENT_EXCEPTION_CATCH = 59,
    JVMTI_EVENT_SINGLE_STEP = 60,
    JVMTI_EVENT_FRAME_POP = 61,
    JVMTI_EVENT_BREAKPOINT = 62,
    JVMTI_EVENT_FIELD_ACCESS = 63,
    JVMTI_EVENT_FIELD_MODIFICATION = 64,
    JVMTI_EVENT_METHOD_ENTRY = 65,
    JVMTI_EVENT_METHOD_EXIT = 66,
    JVMTI_EVENT_NATIVE_METHOD_BIND = 67,
    JVMTI_EVENT_COMPILED_METHOD_LOAD = 68,
    JVMTI_EVENT_COMPILED_METHOD_UNLOAD = 69,
    JVMTI_EVENT_DYNAMIC_CODE_GENERATED = 70,
    JVMTI_EVENT_DATA_DUMP_REQUEST = 71,
    JVMTI_EVENT_MONITOR_WAIT = 73,
    JVMTI_EVENT_MONITOR_WAITED = 74,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTER = 75,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTERED = 76,
    JVMTI_EVENT_RESOURCE_EXHAUSTED = 80,
    JVMTI_EVENT_GARBAGE_COLLECTION_START = 81,
    JVMTI_EVENT_GARBAGE_COLLECTION_FINISH = 82,
    JVMTI_EVENT_OBJECT_FREE = 83,
    JVMTI_EVENT_VM_OBJECT_ALLOC = 84,
    JVMTI_EVENT_SAMPLED_OBJECT_ALLOC = 86,
}
pub const JVMTI_MAX_EVENT_TYPE_VAL: c_uint = 86;

//#[allow(non_camel_case_types)]
//pub type jvmtiThreadInfo = Struct__jvmtiThreadInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiMonitorStackDepthInfo = Struct__jvmtiMonitorStackDepthInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiThreadGroupInfo = Struct__jvmtiThreadGroupInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiFrameInfo = Struct__jvmtiFrameInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiStackInfo = Struct__jvmtiStackInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoField = Struct__jvmtiHeapReferenceInfoField;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoArray = Struct__jvmtiHeapReferenceInfoArray;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoConstantPool = Struct__jvmtiHeapReferenceInfoConstantPool;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoStackLocal = Struct__jvmtiHeapReferenceInfoStackLocal;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoJniLocal = Struct__jvmtiHeapReferenceInfoJniLocal;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfoReserved = Struct__jvmtiHeapReferenceInfoReserved;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapReferenceInfo = Union__jvmtiHeapReferenceInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiHeapCallbacks = Struct__jvmtiHeapCallbacks;
//#[allow(non_camel_case_types)]
//pub type jvmtiClassDefinition = Struct__jvmtiClassDefinition;
//#[allow(non_camel_case_types)]
//pub type jvmtiMonitorUsage = Struct__jvmtiMonitorUsage;
//#[allow(non_camel_case_types)]
//pub type jvmtiLineNumberEntry = Struct__jvmtiLineNumberEntry;
//#[allow(non_camel_case_types)]
//pub type jvmtiLocalVariableEntry = Struct__jvmtiLocalVariableEntry;
//#[allow(non_camel_case_types)]
//pub type jvmtiParamInfo = Struct__jvmtiParamInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiExtensionFunctionInfo = Struct__jvmtiExtensionFunctionInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiExtensionEventInfo = Struct__jvmtiExtensionEventInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiTimerInfo = Struct__jvmtiTimerInfo;
//#[allow(non_camel_case_types)]
//pub type jvmtiAddrLocationMap = Struct__jvmtiAddrLocationMap;
//#[allow(non_camel_case_types)]

pub type jvmtiStartFunction = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, arg: *mut c_void) -> (),
>;
pub type jvmtiHeapIterationCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;
pub type jvmtiHeapReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiHeapReferenceKind,
        reference_info: *const jvmtiHeapReferenceInfo,
        class_tag: jlong,
        referrer_class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;
pub type jvmtiPrimitiveFieldCallback = Option<
    unsafe extern "C" fn(
        kind: jvmtiHeapReferenceKind,
        info: *const jvmtiHeapReferenceInfo,
        object_class_tag: jlong,
        object_tag_ptr: *mut jlong,
        value: jvalue,
        value_type: jvmtiPrimitiveType,
        user_data: *mut c_void,
    ) -> jint,
>;
pub type jvmtiArrayPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        element_count: jint,
        element_type: jvmtiPrimitiveType,
        elements: *const c_void,
        user_data: *mut c_void,
    ) -> jint,
>;
pub type jvmtiStringPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        value: *const jchar,
        value_length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;
pub type jvmtiReservedCallback = Option<extern "C" fn() -> jint>;
pub type jvmtiHeapObjectCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;
pub type jvmtiHeapRootCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;
pub type jvmtiStackReferenceCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        thread_tag: jlong,
        depth: jint,
        method: jmethodID,
        slot: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;
pub type jvmtiObjectReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiObjectReferenceKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag: jlong,
        referrer_index: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;
pub type jvmtiExtensionFunction =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError>;
pub type jvmtiExtensionEvent = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiThreadInfo {
    pub name: *mut c_char,
    pub priority: jint,
    pub is_daemon: jboolean,
    pub thread_group: jthreadGroup,
    pub context_class_loader: jobject,
}
impl Clone for jvmtiThreadInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiThreadInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiMonitorStackDepthInfo {
    pub monitor: jobject,
    pub stack_depth: jint,
}
impl Clone for jvmtiMonitorStackDepthInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiMonitorStackDepthInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiThreadGroupInfo {
    pub parent: jthreadGroup,
    pub name: *mut c_char,
    pub max_priority: jint,
    pub is_daemon: jboolean,
}
impl Clone for jvmtiThreadGroupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiThreadGroupInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiFrameInfo {
    pub method: jmethodID,
    pub location: jlocation,
}
impl Clone for jvmtiFrameInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiFrameInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiStackInfo {
    pub thread: jthread,
    pub state: jint,
    pub frame_buffer: *mut jvmtiFrameInfo,
    pub frame_count: jint,
}
impl Clone for jvmtiStackInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiStackInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoField {
    pub index: jint,
}
impl Clone for jvmtiHeapReferenceInfoField {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoField {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoArray {
    pub index: jint,
}
impl Clone for jvmtiHeapReferenceInfoArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoArray {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoConstantPool {
    pub index: jint,
}
impl Clone for jvmtiHeapReferenceInfoConstantPool {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoConstantPool {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoStackLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
    pub location: jlocation,
    pub slot: jint,
}
impl Clone for jvmtiHeapReferenceInfoStackLocal {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoStackLocal {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoJniLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
}
impl Clone for jvmtiHeapReferenceInfoJniLocal {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoJniLocal {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfoReserved {
    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}
impl Clone for jvmtiHeapReferenceInfoReserved {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfoReserved {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapReferenceInfo {
    pub _bindgen_data_: [u64; 8usize],
}
impl jvmtiHeapReferenceInfo {
    pub unsafe fn field(&mut self) -> *mut jvmtiHeapReferenceInfoField {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn array(&mut self) -> *mut jvmtiHeapReferenceInfoArray {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn constant_pool(&mut self) -> *mut jvmtiHeapReferenceInfoConstantPool {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn stack_local(&mut self) -> *mut jvmtiHeapReferenceInfoStackLocal {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn jni_local(&mut self) -> *mut jvmtiHeapReferenceInfoJniLocal {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn other(&mut self) -> *mut jvmtiHeapReferenceInfoReserved {
        let raw: *mut u8 = &self._bindgen_data_ as *const [u64; 8] as *mut u8;
        mem::transmute(raw.offset(0))
    }
}
impl Clone for jvmtiHeapReferenceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapReferenceInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiHeapCallbacks {
    pub heap_iteration_callback: jvmtiHeapIterationCallback,
    pub heap_reference_callback: jvmtiHeapReferenceCallback,
    pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
    pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
    pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
    pub reserved5: jvmtiReservedCallback,
    pub reserved6: jvmtiReservedCallback,
    pub reserved7: jvmtiReservedCallback,
    pub reserved8: jvmtiReservedCallback,
    pub reserved9: jvmtiReservedCallback,
    pub reserved10: jvmtiReservedCallback,
    pub reserved11: jvmtiReservedCallback,
    pub reserved12: jvmtiReservedCallback,
    pub reserved13: jvmtiReservedCallback,
    pub reserved14: jvmtiReservedCallback,
    pub reserved15: jvmtiReservedCallback,
}
impl Clone for jvmtiHeapCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiHeapCallbacks {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiClassDefinition {
    pub klass: jclass,
    pub class_byte_count: jint,
    pub class_bytes: *const c_uchar,
}
impl Clone for jvmtiClassDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiClassDefinition {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiMonitorUsage {
    pub owner: jthread,
    pub entry_count: jint,
    pub waiter_count: jint,
    pub waiters: *mut jthread,
    pub notify_waiter_count: jint,
    pub notify_waiters: *mut jthread,
}
impl Clone for jvmtiMonitorUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiMonitorUsage {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}
impl Clone for jvmtiLineNumberEntry {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiLineNumberEntry {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiLocalVariableEntry {
    pub start_location: jlocation,
    pub length: jint,
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub generic_signature: *mut c_char,
    pub slot: jint,
}
impl Clone for jvmtiLocalVariableEntry {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiLocalVariableEntry {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiParamInfo {
    pub name: *mut c_char,
    pub kind: jvmtiParamKind,
    pub base_type: jvmtiParamTypes,
    pub null_ok: jboolean,
}
impl Clone for jvmtiParamInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiParamInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiExtensionFunctionInfo {
    pub func: jvmtiExtensionFunction,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
    pub error_count: jint,
    pub errors: *mut jvmtiError,
}
impl Clone for jvmtiExtensionFunctionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiExtensionFunctionInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiExtensionEventInfo {
    pub extension_event_index: jint,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
}
impl Clone for jvmtiExtensionEventInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiExtensionEventInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiTimerInfo {
    pub max_value: jlong,
    pub may_skip_forward: jboolean,
    pub may_skip_backward: jboolean,
    pub kind: jvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}
impl Clone for jvmtiTimerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiTimerInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}
impl Clone for jvmtiAddrLocationMap {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiAddrLocationMap {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiCapabilities {
    pub _bindgen_bitfield_1_: c_uint,
    pub _bindgen_bitfield_2_: c_uint,
    pub _bindgen_bitfield_3_: c_uint,
    pub _bindgen_bitfield_4_: c_uint,
}
impl Clone for jvmtiCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiCapabilities {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type jvmtiEventReserved = Option<extern "C" fn() -> ()>;
pub type jvmtiEventBreakpoint = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ) -> (),
>;
pub type jvmtiEventClassFileLoadHook = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        class_being_redefined: jclass,
        loader: jobject,
        name: *const c_char,
        protection_domain: jobject,
        class_data_len: jint,
        class_data: *const c_uchar,
        new_class_data_len: *mut jint,
        new_class_data: *mut *mut c_uchar,
    ) -> (),
>;
pub type jvmtiEventClassLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ) -> (),
>;
pub type jvmtiEventClassPrepare = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ) -> (),
>;
pub type jvmtiEventCompiledMethodLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        method: jmethodID,
        code_size: jint,
        code_addr: *const c_void,
        map_length: jint,
        map: *const jvmtiAddrLocationMap,
        compile_info: *const c_void,
    ) -> (),
>;
pub type jvmtiEventCompiledMethodUnload = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        method: jmethodID,
        code_addr: *const c_void,
    ) -> (),
>;
pub type jvmtiEventDataDumpRequest = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv) -> ()>;
pub type jvmtiEventDynamicCodeGenerated = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        name: *const c_char,
        address: *const c_void,
        length: jint,
    ) -> (),
>;
pub type jvmtiEventException = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
        catch_method: jmethodID,
        catch_location: jlocation,
    ) -> (),
>;
pub type jvmtiEventExceptionCatch = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
    ) -> (),
>;
pub type jvmtiEventFieldAccess = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
    ) -> (),
>;
pub type jvmtiEventFieldModification = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
        signature_type: c_char,
        new_value: jvalue,
    ) -> (),
>;
pub type jvmtiEventFramePop = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
    ) -> (),
>;
pub type jvmtiEventGarbageCollectionFinish =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv) -> ()>;
pub type jvmtiEventGarbageCollectionStart =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv) -> ()>;
pub type jvmtiEventMethodEntry = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
    ) -> (),
>;
pub type jvmtiEventMethodExit = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
        return_value: jvalue,
    ) -> (),
>;
pub type jvmtiEventMonitorContendedEnter = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ) -> (),
>;
pub type jvmtiEventMonitorContendedEntered = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ) -> (),
>;
pub type jvmtiEventMonitorWait = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timeout: jlong,
    ) -> (),
>;
pub type jvmtiEventMonitorWaited = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timed_out: jboolean,
    ) -> (),
>;
pub type jvmtiEventNativeMethodBind = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        address: *mut c_void,
        new_address_ptr: *mut *mut c_void,
    ) -> (),
>;
pub type jvmtiEventObjectFree =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, tag: jlong) -> ()>;
pub type jvmtiEventResourceExhausted = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        flags: jint,
        reserved: *const c_void,
        description: *const c_char,
    ) -> (),
>;
pub type jvmtiEventSingleStep = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ) -> (),
>;
pub type jvmtiEventThreadEnd = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> (),
>;
pub type jvmtiEventThreadStart = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> (),
>;
pub type jvmtiEventVMDeath =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv) -> ()>;
pub type jvmtiEventVMInit = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread) -> (),
>;
pub type jvmtiEventVMObjectAlloc = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    ) -> (),
>;
pub type jvmtiEventVMStart =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiEventCallbacks {
    pub VMInit: jvmtiEventVMInit,
    pub VMDeath: jvmtiEventVMDeath,
    pub ThreadStart: jvmtiEventThreadStart,
    pub ThreadEnd: jvmtiEventThreadEnd,
    pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
    pub ClassLoad: jvmtiEventClassLoad,
    pub ClassPrepare: jvmtiEventClassPrepare,
    pub VMStart: jvmtiEventVMStart,
    pub Exception: jvmtiEventException,
    pub ExceptionCatch: jvmtiEventExceptionCatch,
    pub SingleStep: jvmtiEventSingleStep,
    pub FramePop: jvmtiEventFramePop,
    pub Breakpoint: jvmtiEventBreakpoint,
    pub FieldAccess: jvmtiEventFieldAccess,
    pub FieldModification: jvmtiEventFieldModification,
    pub MethodEntry: jvmtiEventMethodEntry,
    pub MethodExit: jvmtiEventMethodExit,
    pub NativeMethodBind: jvmtiEventNativeMethodBind,
    pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
    pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
    pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
    pub DataDumpRequest: jvmtiEventDataDumpRequest,
    pub reserved72: jvmtiEventReserved,
    pub MonitorWait: jvmtiEventMonitorWait,
    pub MonitorWaited: jvmtiEventMonitorWaited,
    pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
    pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
    pub reserved77: jvmtiEventReserved,
    pub reserved78: jvmtiEventReserved,
    pub reserved79: jvmtiEventReserved,
    pub ResourceExhausted: jvmtiEventResourceExhausted,
    pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
    pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
    pub ObjectFree: jvmtiEventObjectFree,
    pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
}
impl Clone for jvmtiEventCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiEventCallbacks {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiInterface_1 {
    /*   1 :  RESERVED */
    pub reserved1: *mut c_void,
    /*   2 : Set Event Notification Mode */
    pub SetEventNotificationMode: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            mode: jvmtiEventMode,
            event_type: jvmtiEvent,
            event_thread: jthread,
            ...
        ) -> jvmtiError,
    >,
    /*   3 : Get All Modules */
    pub GetAllModules: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            module_count_ptr: *mut jint,
            modules_ptr: *mut *mut jobject,
        ) -> jvmtiError,
    >,
    /*   4 : Get All Threads */
    pub GetAllThreads: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            threads_count_ptr: *mut jint,
            threads_ptr: *mut *mut jthread,
        ) -> jvmtiError,
    >,
    /*   5 : Suspend Thread */
    pub SuspendThread:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    /*   6 : Resume Thread */
    pub ResumeThread:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    /*   7 : Stop Thread */
    pub StopThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError,
    >,
    /*   8 : Interrupt Thread */
    pub InterruptThread:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    /*   9 : Get Thread Info */
    pub GetThreadInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            info_ptr: *mut jvmtiThreadInfo,
        ) -> jvmtiError,
    >,
    /*   10 : Get Owned Monitor Info */
    pub GetOwnedMonitorInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            owned_monitor_count_ptr: *mut jint,
            owned_monitors_ptr: *mut *mut jobject,
        ) -> jvmtiError,
    >,
    /*   11 : Get Current Contended Monitor */
    pub GetCurrentContendedMonitor: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            monitor_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    /*   12 : Run Agent Thread */
    pub RunAgentThread: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            _proc: jvmtiStartFunction,
            arg: *const c_void,
            priority: jint,
        ) -> jvmtiError,
    >,
    /*   13 : Get Top Thread Groups */
    pub GetTopThreadGroups: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group_count_ptr: *mut jint,
            groups_ptr: *mut *mut jthreadGroup,
        ) -> jvmtiError,
    >,
    /*   14 : Get Thread Group Info */
    pub GetThreadGroupInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group: jthreadGroup,
            info_ptr: *mut jvmtiThreadGroupInfo,
        ) -> jvmtiError,
    >,
    /*   15 : Get Thread Group Children */
    pub GetThreadGroupChildren: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group: jthreadGroup,
            thread_count_ptr: *mut jint,
            threads_ptr: *mut *mut jthread,
            group_count_ptr: *mut jint,
            groups_ptr: *mut *mut jthreadGroup,
        ) -> jvmtiError,
    >,
    /*   16 : Get Frame Count */
    pub GetFrameCount: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   17 : Get Thread State */
    pub GetThreadState: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            thread_state_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   18 : Get Current Thread */
    pub GetCurrentThread:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError>,
    /*   19 : Get Frame Location */
    pub GetFrameLocation: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            method_ptr: *mut jmethodID,
            location_ptr: *mut jlocation,
        ) -> jvmtiError,
    >,
    /*   20 : Notify Frame Pop */
    pub NotifyFramePop: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
    >,
    /*   21 : Get Local Variable - Object */
    pub GetLocalObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    /*   22 : Get Local Variable - Int */
    pub GetLocalInt: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   23 : Get Local Variable - Long */
    pub GetLocalLong: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    /*   24 : Get Local Variable - Float */
    pub GetLocalFloat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jfloat,
        ) -> jvmtiError,
    >,
    /*   25 : Get Local Variable - Double */
    pub GetLocalDouble: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jdouble,
        ) -> jvmtiError,
    >,
    /*   26 : Set Local Variable - Object */
    pub SetLocalObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jobject,
        ) -> jvmtiError,
    >,
    /*   27 : Set Local Variable - Int */
    pub SetLocalInt: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jint,
        ) -> jvmtiError,
    >,
    /*   28 : Set Local Variable - Long */
    pub SetLocalLong: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jlong,
        ) -> jvmtiError,
    >,
    /*   29 : Set Local Variable - Float */
    pub SetLocalFloat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jfloat,
        ) -> jvmtiError,
    >,
    /*   30 : Set Local Variable - Double */
    pub SetLocalDouble: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jdouble,
        ) -> jvmtiError,
    >,
    /*   31 : Create Raw Monitor */
    pub CreateRawMonitor: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            name: *const c_char,
            monitor_ptr: *mut jrawMonitorID,
        ) -> jvmtiError,
    >,
    /*   32 : Destroy Raw Monitor */
    pub DestroyRawMonitor:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    /*   33 : Raw Monitor Enter */
    pub RawMonitorEnter:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    /*   34 : Raw Monitor Exit */
    pub RawMonitorExit:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    /*   35 : Raw Monitor Wait */
    pub RawMonitorWait: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            monitor: jrawMonitorID,
            millis: jlong,
        ) -> jvmtiError,
    >,
    /*   36 : Raw Monitor Notify */
    pub RawMonitorNotify:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    /*   37 : Raw Monitor Notify All */
    pub RawMonitorNotifyAll:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    /*   38 : Set Breakpoint */
    pub SetBreakpoint: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            location: jlocation,
        ) -> jvmtiError,
    >,
    /*   39 : Clear Breakpoint */
    pub ClearBreakpoint: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            location: jlocation,
        ) -> jvmtiError,
    >,
    /*   40 : Get Named Module */
    pub GetNamedModule: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, 
            class_loader: jobject, 
            package_name: *const c_char, 
            module_ptr: *mut jobject) -> jvmtiError,
    >,
    /*   41 : Set Field Access Watch */
    pub SetFieldAccessWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    /*   42 : Clear Field Access Watch */
    pub ClearFieldAccessWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    /*   43 : Set Field Modification Watch */
    pub SetFieldModificationWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    /*   44 : Clear Field Modification Watch */
    pub ClearFieldModificationWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    /*   45 : Is Modifiable Class */
    pub IsModifiableClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_modifiable_class_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   46 : Allocate */
    pub Allocate: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            size: jlong,
            mem_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    /*   47 : Deallocate */
    pub Deallocate:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError>,
    /*   48 : Get Class Signature */
    pub GetClassSignature: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   49 : Get Class Status */
    pub GetClassStatus: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            status_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   50 : Get Source File Name */
    pub GetSourceFileName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            source_name_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   51 : Get Class Modifiers */
    pub GetClassModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   52 : Get Class Methods */
    pub GetClassMethods: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            method_count_ptr: *mut jint,
            methods_ptr: *mut *mut jmethodID,
        ) -> jvmtiError,
    >,
    /*   53 : Get Class Fields */
    pub GetClassFields: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field_count_ptr: *mut jint,
            fields_ptr: *mut *mut jfieldID,
        ) -> jvmtiError,
    >,
    /*   54 : Get Implemented Interfaces */
    pub GetImplementedInterfaces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            interface_count_ptr: *mut jint,
            interfaces_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    /*   55 : Is Interface */
    pub IsInterface: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_interface_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   56 : Is Array Class */
    pub IsArrayClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_array_class_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   57 : Get Class Loader */
    pub GetClassLoader: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            classloader_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    /*   58 : Get Object Hash Code */
    pub GetObjectHashCode: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            hash_code_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   59 : Get Object Monitor Usage */
    pub GetObjectMonitorUsage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            info_ptr: *mut jvmtiMonitorUsage,
        ) -> jvmtiError,
    >,
    /*   60 : Get Field Name (and Signature) */
    pub GetFieldName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            name_ptr: *mut *mut c_char,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   61 : Get Field Declaring Class */
    pub GetFieldDeclaringClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            declaring_class_ptr: *mut jclass,
        ) -> jvmtiError,
    >,
    /*   62 : Get Field Modifiers */
    pub GetFieldModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   63 : Is Field Synthetic */
    pub IsFieldSynthetic: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            is_synthetic_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   64 : Get Method Name (and Signature) */
    pub GetMethodName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            name_ptr: *mut *mut c_char,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   65 : Get Method Declaring Class */
    pub GetMethodDeclaringClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            declaring_class_ptr: *mut jclass,
        ) -> jvmtiError,
    >,
    /*   66 : Get Method Modifiers */
    pub GetMethodModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   67 :  RESERVED */
    pub reserved67: *mut c_void,
    /*   68 : Get Max Locals */
    pub GetMaxLocals: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            max_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   69 : Get Arguments Size */
    pub GetArgumentsSize: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            size_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   70 : Get Line Number Table */
    pub GetLineNumberTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            entry_count_ptr: *mut jint,
            table_ptr: *mut *mut jvmtiLineNumberEntry,
        ) -> jvmtiError,
    >,
    /*   71 : Get Method Location */
    pub GetMethodLocation: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            start_location_ptr: *mut jlocation,
            end_location_ptr: *mut jlocation,
        ) -> jvmtiError,
    >,
    /*   72 : Get Local Variable Table */
    pub GetLocalVariableTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            entry_count_ptr: *mut jint,
            table_ptr: *mut *mut jvmtiLocalVariableEntry,
        ) -> jvmtiError,
    >,
    /*   73 : Set Native Method Prefix */
    pub SetNativeMethodPrefix:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError>,
    /*   74 : Set Native Method Prefixes */
    pub SetNativeMethodPrefixes: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            prefix_count: jint,
            prefixes: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   75 : Get Bytecodes */
    pub GetBytecodes: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            bytecode_count_ptr: *mut jint,
            bytecodes_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    /*   76 : Is Method Native */
    pub IsMethodNative: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_native_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   77 : Is Method Synthetic */
    pub IsMethodSynthetic: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_synthetic_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   78 : Get Loaded Classes */
    pub GetLoadedClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count_ptr: *mut jint,
            classes_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    /*   79 : Get Classloader Classes */
    pub GetClassLoaderClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            initiating_loader: jobject,
            class_count_ptr: *mut jint,
            classes_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    /*   80 : Pop Frame */
    pub PopFrame: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    /*   81 : Force Early Return - Object */
    pub ForceEarlyReturnObject: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError,
    >,
    /*   82 : Force Early Return - Int */
    pub ForceEarlyReturnInt: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
    >,
    /*   83 : Force Early Return - Long */
    pub ForceEarlyReturnLong: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
    >,
    /*   84 : Force Early Return - Float */
    pub ForceEarlyReturnFloat: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
    >,
    /*   85 : Force Early Return - Double */
    pub ForceEarlyReturnDouble: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError,
    >,
    /*   86 : Force Early Return - Void */
    pub ForceEarlyReturnVoid:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    /*   87 : Redefine Classes */
    pub RedefineClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count: jint,
            class_definitions: *const jvmtiClassDefinition,
        ) -> jvmtiError,
    >,
    /*   88 : Get Version Number */
    pub GetVersionNumber:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError>,
    /*   89 : Get Capabilities */
    pub GetCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *mut jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    /*   90 : Get Source Debug Extension */
    pub GetSourceDebugExtension: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            source_debug_extension_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   91 : Is Method Obsolete */
    pub IsMethodObsolete: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_obsolete_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    /*   92 : Suspend Thread List */
    pub SuspendThreadList: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            request_count: jint,
            request_list: *const jthread,
            results: *mut jvmtiError,
        ) -> jvmtiError,
    >,
    /*   93 : Resume Thread List */
    pub ResumeThreadList: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            request_count: jint,
            request_list: *const jthread,
            results: *mut jvmtiError,
        ) -> jvmtiError,
    >,
    /*   94 : Add Module Reads */
    pub AddModuleReads: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            to_module: jobject
        ) -> jvmtiError
    >,
    /*   95 : Add Module Exports */
    pub AddModuleExports: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            pkg_name: *const c_char,
            to_module: jobject
        ) -> jvmtiError
    >,
    /*   96 : Add Module Opens */
    pub AddModuleOpens: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            pkg_name: *const c_char,
            to_module: jobject
        ) -> jvmtiError
    >,
    /*   97 : Add Module Uses */
    pub AddModuleUses: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            service: jclass
        ) -> jvmtiError
    >,
    /*   98 : Add Module Provides */
    pub AddModuleProvides: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            service: jclass,
            impl_class: jclass
        ) -> jvmtiError
    >,
    /*   99 : Is Modifiable Module */
    pub IsModifiableModule: Option<
        unsafe extern "C" fn (
            env: *mut jvmtiEnv,
            module: jobject,
            is_modifiable_module_ptr: *mut jboolean
        ) -> jvmtiError
    >,
    /*   100 : Get All Stack Traces */
    pub GetAllStackTraces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            max_frame_count: jint,
            stack_info_ptr: *mut *mut jvmtiStackInfo,
            thread_count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   101 : Get Thread List Stack Traces */
    pub GetThreadListStackTraces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread_count: jint,
            thread_list: *const jthread,
            max_frame_count: jint,
            stack_info_ptr: *mut *mut jvmtiStackInfo,
        ) -> jvmtiError,
    >,
    /*   102 : Get Thread Local Storage */
    pub GetThreadLocalStorage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            data_ptr: *mut *mut c_void,
        ) -> jvmtiError,
    >,
    /*   103 : Set Thread Local Storage */
    pub SetThreadLocalStorage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   104 : Get Stack Trace */
    pub GetStackTrace: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            start_depth: jint,
            max_frame_count: jint,
            frame_buffer: *mut jvmtiFrameInfo,
            count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   105 :  RESERVED */
    pub reserved105: *mut c_void,
    /*   106 : Get Tag */
    pub GetTag: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            tag_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    /*   107 : Set Tag */
    pub SetTag:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError>,
    /*   108 : Force Garbage Collection */
    pub ForceGarbageCollection: Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    /*   109 : Iterate Over Objects Reachable From Object */
    pub IterateOverObjectsReachableFromObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            object_reference_callback: jvmtiObjectReferenceCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   110 : Iterate Over Reachable Objects */
    pub IterateOverReachableObjects: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_root_callback: jvmtiHeapRootCallback,
            stack_ref_callback: jvmtiStackReferenceCallback,
            object_ref_callback: jvmtiObjectReferenceCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   111 : Iterate Over Heap */
    pub IterateOverHeap: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object_filter: jvmtiHeapObjectFilter,
            heap_object_callback: jvmtiHeapObjectCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   112 : Iterate Over Instances Of Class */
    pub IterateOverInstancesOfClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            object_filter: jvmtiHeapObjectFilter,
            heap_object_callback: jvmtiHeapObjectCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   113 :  RESERVED */
    pub reserved113: *mut c_void,
    /*   114 : Get Objects With Tags */
    pub GetObjectsWithTags: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            tag_count: jint,
            tags: *const jlong,
            count_ptr: *mut jint,
            object_result_ptr: *mut *mut jobject,
            tag_result_ptr: *mut *mut jlong,
        ) -> jvmtiError,
    >,
    /*   115 : Follow References */
    pub FollowReferences: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_filter: jint,
            klass: jclass,
            initial_object: jobject,
            callbacks: *const jvmtiHeapCallbacks,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   116 : Iterate Through Heap */
    pub IterateThroughHeap: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_filter: jint,
            klass: jclass,
            callbacks: *const jvmtiHeapCallbacks,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    /*   117 :  RESERVED */
    pub reserved117: *mut c_void,
    /*   118 :  RESERVED */
    pub reserved118: *mut c_void,
    /*   119 :  RESERVED */
    pub reserved119: *mut c_void,
    /*   120 : Set JNI Function Table */
    pub SetJNIFunctionTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            function_table: *const JNINativeInterface_,
        ) -> jvmtiError,
    >,
    /*   121 : Get JNI Function Table */
    pub GetJNIFunctionTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            function_table: *mut *mut JNINativeInterface_,
        ) -> jvmtiError,
    >,
    /*   122 : Set Event Callbacks */
    pub SetEventCallbacks: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            callbacks: *const jvmtiEventCallbacks,
            size_of_callbacks: jint,
        ) -> jvmtiError,
    >,
    /*   123 : Generate Events */
    pub GenerateEvents:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError>,
    /*   124 : Get Extension Functions */
    pub GetExtensionFunctions: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_count_ptr: *mut jint,
            extensions: *mut *mut jvmtiExtensionFunctionInfo,
        ) -> jvmtiError,
    >,
    /*   125 : Get Extension Events */
    pub GetExtensionEvents: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_count_ptr: *mut jint,
            extensions: *mut *mut jvmtiExtensionEventInfo,
        ) -> jvmtiError,
    >,
    /*   126 : Set Extension Event Callback */
    pub SetExtensionEventCallback: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_event_index: jint,
            callback: jvmtiExtensionEvent,
        ) -> jvmtiError,
    >,
    /*   127 : Dispose Environment */
    pub DisposeEnvironment: Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    /*   128 : Get Error Name */
    pub GetErrorName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            error: jvmtiError,
            name_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   129 : Get JLocation Format */
    pub GetJLocationFormat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            format_ptr: *mut jvmtiJlocationFormat,
        ) -> jvmtiError,
    >,
    /*   130 : Get System Properties */
    pub GetSystemProperties: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            count_ptr: *mut jint,
            property_ptr: *mut *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   131 : Get System Property */
    pub GetSystemProperty: Option<
        unsafe extern "system" fn(
            env: *mut jvmtiEnv,
            property: *const c_char,
            value_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    /*   132 : Set System Property */
    pub SetSystemProperty: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            property: *const c_char,
            value: *const c_char,
        ) -> jvmtiError,
    >,
    /*   133 : Get Phase */
    pub GetPhase:
        Option<unsafe extern "system" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError>,
    /*   134 : Get Current Thread CPU Timer Information */
    pub GetCurrentThreadCpuTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    /*   135 : Get Current Thread CPU Time */
    pub GetCurrentThreadCpuTime:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    /*   136 : Get Thread CPU Timer Information */
    pub GetThreadCpuTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    /*   137 : Get Thread CPU Time */
    pub GetThreadCpuTime: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            nanos_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    /*   138 : Get Timer Information */
    pub GetTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    /*   139 : Get Time */
    pub GetTime:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    /*   140 : Get Potential Capabilities */
    pub GetPotentialCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *mut jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    /*   141 :  RESERVED */
    pub reserved141: *mut c_void,
    /*   142 : Add Capabilities */
    pub AddCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *const jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    /*   143 : Relinquish Capabilities */
    pub RelinquishCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *const jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    /*   144 : Get Available Processors */
    pub GetAvailableProcessors: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
    >,
    /*   145 : Get Class Version Numbers */
    pub GetClassVersionNumbers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            minor_version_ptr: *mut jint,
            major_version_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    /*   146 : Get Constant Pool */
    pub GetConstantPool: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            constant_pool_count_ptr: *mut jint,
            constant_pool_byte_count_ptr: *mut jint,
            constant_pool_bytes_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    /*   147 : Get Environment Local Storage */
    pub GetEnvironmentLocalStorage:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError>,
    /*   148 : Set Environment Local Storage */
    pub SetEnvironmentLocalStorage:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError>,
    /*   149 : Add To Bootstrap Class Loader Search */
    pub AddToBootstrapClassLoaderSearch:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    /*   150 : Set Verbose Flag */
    pub SetVerboseFlag: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            flag: jvmtiVerboseFlag,
            value: jboolean,
        ) -> jvmtiError,
    >,
    /*   151 : Add To System Class Loader Search */
    pub AddToSystemClassLoaderSearch:
        Option<unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    /*   152 : Retransform Classes */
    pub RetransformClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count: jint,
            classes: *const jclass,
        ) -> jvmtiError,
    >,
    /*   153 : Get Owned Monitor Stack Depth Info */
    pub GetOwnedMonitorStackDepthInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            monitor_info_count_ptr: *mut jint,
            monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
        ) -> jvmtiError,
    >,
    /*   154 : Get Object Size */
    pub GetObjectSize: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            size_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    /*   155 : Get Local Instance */
    pub GetLocalInstance: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            value_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    /*   156 : Set Heap Sampling Interval */
    pub SetHeapSamplingInterval: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            sampling_interval: jint,
        ) -> jvmtiError,
    >,
}
impl Clone for jvmtiInterface_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jvmtiInterface_1 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiEnv {
    pub functions: *const jvmtiInterface_1,
}
impl Clone for _jvmtiEnv {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for _jvmtiEnv {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
