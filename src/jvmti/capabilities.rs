#![allow(non_snake_case, non_camel_case_types, dead_code)]

use crate::{jvmti::errors::JvmtiError, jvmti::jvmti_sys::jvmtiCapabilities};

pub const CAN_TAG_OBJECTS: usize = 0;
pub const CAN_GENERATE_FIELD_MODIFICATION_EVENTS: usize = 1;
pub const CAN_GENERATE_FIELD_ACCESS_EVENTS: usize = 2;
pub const CAN_GET_BYTECODES: usize = 3;
pub const CAN_GET_SYNTHETIC_ATTRIBUTE: usize = 4;
pub const CAN_GET_OWNED_MONITOR_INFO: usize = 5;
pub const CAN_GET_CURRENT_CONTENDED_MONITOR: usize = 6;
pub const CAN_GET_MONITOR_INFO: usize = 7;
pub const CAN_POP_FRAME: usize = 8;
pub const CAN_REDEFINE_CLASSES: usize = 9;
pub const CAN_SIGNAL_THREAD: usize = 10;
pub const CAN_GET_SOURCE_FILE_NAME: usize = 11;
pub const CAN_GET_LINE_NUMBERS: usize = 12;
pub const CAN_GET_SOURCE_DEBUG_EXTENSION: usize = 13;
pub const CAN_ACCESS_LOCAL_VARIABLES: usize = 14;
pub const CAN_MAINTAIN_ORIGINAL_METHOD_ORDER: usize = 15;
pub const CAN_GENERATE_SINGLE_STEP_EVENTS: usize = 16;
pub const CAN_GENERATE_EXCEPTION_EVENTS: usize = 17;
pub const CAN_GENERATE_FRAME_POP_EVENTS: usize = 18;
pub const CAN_GENERATE_BREAKPOINT_EVENTS: usize = 19;
pub const CAN_SUSPEND: usize = 20;
pub const CAN_REDEFINE_ANY_CLASS: usize = 21;
pub const CAN_GET_CURRENT_THREAD_CPU_TIME: usize = 22;
pub const CAN_GET_THREAD_CPU_TIME: usize = 23;
pub const CAN_GENERATE_METHOD_ENTRY_EVENTS: usize = 24;
pub const CAN_GENERATE_METHOD_EXIT_EVENTS: usize = 25;
pub const CAN_GENERATE_ALL_CLASS_HOOK_EVENTS: usize = 26;
pub const CAN_GENERATE_COMPILED_METHOD_LOAD_EVENTS: usize = 27;
pub const CAN_GENERATE_MONITOR_EVENTS: usize = 28;
pub const CAN_GENERATE_VM_OBJECT_ALLOC_EVENTS: usize = 29;
pub const CAN_GENERATE_NATIVE_METHOD_BIND_EVENTS: usize = 30;
pub const CAN_GENERATE_GARBAGE_COLLECTION_EVENTS: usize = 31;
pub const CAN_GENERATE_OBJECT_FREE_EVENTS: usize = 32;
pub const CAN_FORCE_EARLY_RETURN: usize = 33;
pub const CAN_GET_OWNED_MONITOR_STACK_DEPTH_INFO: usize = 34;
pub const CAN_GET_CONSTANT_POOL: usize = 35;
pub const CAN_SET_NATIVE_METHOD_PREFIX: usize = 36;
pub const CAN_RETRANSFORM_CLASSES: usize = 37;
pub const CAN_RETRANSFORM_ANY_CLASS: usize = 38;
pub const CAN_GENERATE_RESOURCE_EXHAUSTION_HEAP_EVENTS: usize = 39;
pub const CAN_GENERATE_RESOURCE_EXHAUSTION_THREADS_EVENTS: usize = 40;
pub const CAN_GENERATE_EARLY_VMSTART: usize = 41;
pub const CAN_GENERATE_EARLY_CLASS_HOOK_EVENTS: usize = 42;
pub const CAN_GENERATE_SAMPLED_OBJECT_ALLOC_EVENTS: usize = 43;

pub fn set_capability(capabilities: &mut jvmtiCapabilities, idx: usize) -> Result<(), JvmtiError> {
    let n = idx / 32;
    let bit = idx & 31;
    match n {
        0 => {
            capabilities._bindgen_bitfield_1_ |= 1 << bit;
            Ok(())
        }
        1 => {
            capabilities._bindgen_bitfield_2_ |= 1 << bit;
            Ok(())
        }
        _ => Err(JvmtiError::NotAvailable),
    }
}
