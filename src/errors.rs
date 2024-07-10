use jni::errors::StartJvmError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneralError {
    #[error("{0}")]
    StartJvmError(
        #[from]
        #[source]
        StartJvmError
    ),

    #[error("{0}")]
    JvmArgsError(String),
}


