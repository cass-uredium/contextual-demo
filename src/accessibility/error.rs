use objc2_application_services::AXError;
// use thiserror::Error;

// pub type Result<T> = std::result::Result<T, Error>;

// #[derive(Error, Debug)]
// pub enum Error {
//     #[error("AXError: {0:?}")]
//     AX(AXError),
// }

pub trait AXErrorExt {
    fn into_result(self) -> Result<(), AXError>;
}

impl AXErrorExt for AXError {
    fn into_result(self) -> Result<(), AXError> {
        match self {
            AXError::Success => Ok(()),
            _ => Err(self),
        }
    }
}
