#![allow(non_snake_case, non_camel_case_types, dead_code)]

use crate::jvmti::jvmti_sys::jvmtiError;
use thiserror::Error;


pub type JvmtiResult<T> = Result<T, JvmtiError>;

#[derive(Debug, Error)]
pub enum JvmtiError {
    #[error("Invalid Thread")]
    InvalidThread = 10,
    #[error("Invalid Thread Group")]
    InvalidThreadGroup = 11,
    #[error("Invalid Priority")]
    InvalidPriority = 12,
    #[error("Thread Not Suspended")]
    ThreadNotSuspended = 13,
    #[error("Thread Suspended")]
    ThreadSuspended = 14,
    #[error("Thread Not Alive")]
    ThreadNotAlive = 15,
    #[error("Invalid Object")]
    InvalidObject = 20,
    #[error("Invalid Class")]
    InvalidClass = 21,
    #[error("Class Not Prepared")]
    ClassNotPrepared = 22,
    #[error("Invalid Methodid")]
    InvalidMethodid = 23,
    #[error("Invalid Location")]
    InvalidLocation = 24,
    #[error("Invalid Fieldid")]
    InvalidFieldid = 25,
    #[error("No More Frames")]
    NoMoreFrames = 31,
    #[error("Opaque Frame")]
    OpaqueFrame = 32,
    #[error("Type Mismatch")]
    TypeMismatch = 34,
    #[error("Invalid Slot")]
    InvalidSlot = 35,
    #[error("Duplicate")]
    Duplicate = 40,
    #[error("Not Found")]
    NotFound = 41,
    #[error("Invalid Monitor")]
    InvalidMonitor = 50,
    #[error("Not Monitor Owner")]
    NotMonitorOwner = 51,
    #[error("Interrupt")]
    Interrupt = 52,
    #[error("Invalid Class Format")]
    InvalidClassFormat = 60,
    #[error("Circular Class Definition")]
    CircularClassDefinition = 61,
    #[error("Fails Verification")]
    FailsVerification = 62,
    #[error("Unsupported Redefinition Method Added")]
    UnsupportedRedefinitionMethodAdded = 63,
    #[error("Unsupported Redefinition Schema Changed")]
    UnsupportedRedefinitionSchemaChanged = 64,
    #[error("Invalid Typestate")]
    InvalidTypestate = 65,
    #[error("Unsupported Redefinition Hierarchy Changed")]
    UnsupportedRedefinitionHierarchyChanged = 66,
    #[error("Unsupported Redefinition Method Deleted")]
    UnsupportedRedefinitionMethodDeleted = 67,
    #[error("Unsupported Version")]
    UnsupportedVersion = 68,
    #[error("Names Dont Match")]
    NamesDontMatch = 69,
    #[error("Unsupported Redefinition Class Modifiers Changed")]
    UnsupportedRedefinitionClassModifiersChanged = 70,
    #[error("Unsupported Redefinition Method Modifiers Changed")]
    UnsupportedRedefinitionMethodModifiersChanged = 71,
    #[error("Unmodifiable Class")]
    UnmodifiableClass = 79,
    #[error("Not Available")]
    NotAvailable = 98,
    #[error("Must Possess Capability")]
    MustPossessCapability = 99,
    #[error("Null Pointer")]
    NullPointer = 100,
    #[error("Absent Information")]
    AbsentInformation = 101,
    #[error("Invalid Event Type")]
    InvalidEventType = 102,
    #[error("Illegal Argument")]
    IllegalArgument = 103,
    #[error("Native Method")]
    NativeMethod = 104,
    #[error("Class Loader Unsupported")]
    ClassLoaderUnsupported = 106,
    #[error("Out Of Memory")]
    OutOfMemory = 110,
    #[error("Access Denied")]
    AccessDenied = 111,
    #[error("Wrong Phase")]
    WrongPhase = 112,
    #[error("Internal")]
    Internal = 113,
    #[error("Unattached Thread")]
    UnattachedThread = 115,
    #[error("Invalid Environment")]
    InvalidEnvironment = 116,
}

impl From<&jvmtiError> for JvmtiError {
    fn from(error: &jvmtiError) -> Self {
        match error {
            jvmtiError::JVMTI_ERROR_NONE => {
                panic!("This should not happen!")
            }
            jvmtiError::JVMTI_ERROR_INVALID_THREAD => JvmtiError::InvalidThread,
            jvmtiError::JVMTI_ERROR_INVALID_THREAD_GROUP => JvmtiError::InvalidThreadGroup,
            jvmtiError::JVMTI_ERROR_INVALID_PRIORITY => JvmtiError::InvalidPriority,
            jvmtiError::JVMTI_ERROR_THREAD_NOT_SUSPENDED => JvmtiError::ThreadNotSuspended,
            jvmtiError::JVMTI_ERROR_THREAD_SUSPENDED => JvmtiError::ThreadSuspended,
            jvmtiError::JVMTI_ERROR_THREAD_NOT_ALIVE => JvmtiError::ThreadNotAlive,
            jvmtiError::JVMTI_ERROR_INVALID_OBJECT => JvmtiError::InvalidObject,
            jvmtiError::JVMTI_ERROR_INVALID_CLASS => JvmtiError::InvalidClass,
            jvmtiError::JVMTI_ERROR_CLASS_NOT_PREPARED => JvmtiError::ClassNotPrepared,
            jvmtiError::JVMTI_ERROR_INVALID_METHODID => JvmtiError::InvalidMethodid,
            jvmtiError::JVMTI_ERROR_INVALID_LOCATION => JvmtiError::InvalidLocation,
            jvmtiError::JVMTI_ERROR_INVALID_FIELDID => JvmtiError::InvalidFieldid,
            jvmtiError::JVMTI_ERROR_NO_MORE_FRAMES => JvmtiError::NoMoreFrames,
            jvmtiError::JVMTI_ERROR_OPAQUE_FRAME => JvmtiError::OpaqueFrame,
            jvmtiError::JVMTI_ERROR_TYPE_MISMATCH => JvmtiError::TypeMismatch,
            jvmtiError::JVMTI_ERROR_INVALID_SLOT => JvmtiError::InvalidSlot,
            jvmtiError::JVMTI_ERROR_DUPLICATE => JvmtiError::Duplicate,
            jvmtiError::JVMTI_ERROR_NOT_FOUND => JvmtiError::NotFound,
            jvmtiError::JVMTI_ERROR_INVALID_MONITOR => JvmtiError::InvalidMonitor,
            jvmtiError::JVMTI_ERROR_NOT_MONITOR_OWNER => JvmtiError::NotMonitorOwner,
            jvmtiError::JVMTI_ERROR_INTERRUPT => JvmtiError::Interrupt,
            jvmtiError::JVMTI_ERROR_INVALID_CLASS_FORMAT => JvmtiError::InvalidClassFormat,
            jvmtiError::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION => {
                JvmtiError::CircularClassDefinition
            }
            jvmtiError::JVMTI_ERROR_FAILS_VERIFICATION => JvmtiError::FailsVerification,
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED => {
                JvmtiError::UnsupportedRedefinitionMethodAdded
            }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED => {
                JvmtiError::UnsupportedRedefinitionSchemaChanged
            }
            jvmtiError::JVMTI_ERROR_INVALID_TYPESTATE => JvmtiError::InvalidTypestate,
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED => {
                JvmtiError::UnsupportedRedefinitionHierarchyChanged
            }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED => {
                JvmtiError::UnsupportedRedefinitionMethodDeleted
            }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_VERSION => JvmtiError::UnsupportedVersion,
            jvmtiError::JVMTI_ERROR_NAMES_DONT_MATCH => JvmtiError::NamesDontMatch,
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED => {
                JvmtiError::UnsupportedRedefinitionClassModifiersChanged
            }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED => {
                JvmtiError::UnsupportedRedefinitionMethodModifiersChanged
            }
            jvmtiError::JVMTI_ERROR_UNMODIFIABLE_CLASS => JvmtiError::UnmodifiableClass,
            jvmtiError::JVMTI_ERROR_NOT_AVAILABLE => JvmtiError::NotAvailable,
            jvmtiError::JVMTI_ERROR_MUST_POSSESS_CAPABILITY => JvmtiError::MustPossessCapability,
            jvmtiError::JVMTI_ERROR_NULL_POINTER => JvmtiError::NullPointer,
            jvmtiError::JVMTI_ERROR_ABSENT_INFORMATION => JvmtiError::AbsentInformation,
            jvmtiError::JVMTI_ERROR_INVALID_EVENT_TYPE => JvmtiError::InvalidEventType,
            jvmtiError::JVMTI_ERROR_ILLEGAL_ARGUMENT => JvmtiError::IllegalArgument,
            jvmtiError::JVMTI_ERROR_NATIVE_METHOD => JvmtiError::NativeMethod,
            jvmtiError::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED => JvmtiError::ClassLoaderUnsupported,
            jvmtiError::JVMTI_ERROR_OUT_OF_MEMORY => JvmtiError::OutOfMemory,
            jvmtiError::JVMTI_ERROR_ACCESS_DENIED => JvmtiError::AccessDenied,
            jvmtiError::JVMTI_ERROR_WRONG_PHASE => JvmtiError::WrongPhase,
            jvmtiError::JVMTI_ERROR_INTERNAL => JvmtiError::Internal,
            jvmtiError::JVMTI_ERROR_UNATTACHED_THREAD => JvmtiError::UnattachedThread,
            jvmtiError::JVMTI_ERROR_INVALID_ENVIRONMENT => JvmtiError::InvalidEnvironment,
        }
    }
}
