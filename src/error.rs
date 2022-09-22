use std::{error, ffi::NulError, fmt};

#[derive(Debug)]
pub enum Error {
    Spine(String),
    NulError(NulError),
    NotFound,
    FailedToReadFile(String),
}

impl Error {
    pub fn new_from_spine(message: &str) -> Self {
        Self::Spine(String::from(message))
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::NulError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Spine(str) => {
                write!(f, "Spine error: {}", str)?;
                Ok(())
            }
            Error::NulError(error) => {
                write!(f, "Nul error: {}", error)?;
                Ok(())
            }
            Error::NotFound => {
                // TODO: make this error better, this is not helpful
                write!(f, "Not found.")?;
                Ok(())
            }
            Self::FailedToReadFile(file) => {
                write!(f, "Failed to read file: {}", file)?;
                Ok(())
            }
        }
    }
}

impl error::Error for Error {}
