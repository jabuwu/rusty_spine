use std::{error, ffi::NulError, fmt};

/// Error types when interacting with the Spine runtime.
#[derive(Debug)]
pub enum SpineError {
    /// A parsing error straight from the Spine C runtime.
    ParsingFailed { reason: String },
    /// A wrapper for [`std::ffi::NulError`].
    NulError(NulError),
    /// An error when something couldn't be found, represented by `what` it was and its `name`.
    NotFound { what: String, name: String },
    /// An error when failing to read files.
    FailedToReadFile { file: String },
    /// An error when a specified path is not utf-8.
    PathNotUtf8,
    /// Failed to create the requested type.
    CreationFailed { what: String },
}

impl SpineError {
    pub(crate) fn new_from_spine(reason: &str) -> Self {
        Self::ParsingFailed {
            reason: reason.to_owned(),
        }
    }

    pub(crate) fn new_not_found(what: &str, name: &str) -> Self {
        Self::NotFound {
            what: what.to_owned(),
            name: name.to_owned(),
        }
    }

    pub(crate) fn new_creation_failed(what: &str) -> Self {
        Self::CreationFailed {
            what: what.to_owned(),
        }
    }
}

impl From<NulError> for SpineError {
    fn from(err: NulError) -> Self {
        Self::NulError(err)
    }
}

impl fmt::Display for SpineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpineError::ParsingFailed { reason } => {
                write!(f, "Spine parsing failed: {reason}")?;
                Ok(())
            }
            SpineError::NulError(error) => {
                write!(f, "Nul error: {error}")?;
                Ok(())
            }
            SpineError::NotFound { what, name } => {
                // TODO: make this error better, this is not helpful
                write!(f, "{what} not found: {name}")?;
                Ok(())
            }
            Self::FailedToReadFile { file } => {
                write!(f, "Failed to read file: {file}")?;
                Ok(())
            }
            SpineError::PathNotUtf8 => {
                write!(f, "Path not utf-8")?;
                Ok(())
            }
            SpineError::CreationFailed { what } => {
                write!(f, "Failed to create {what}")?;
                Ok(())
            }
        }
    }
}

impl error::Error for SpineError {}
