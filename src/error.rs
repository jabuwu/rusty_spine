use std::ffi::NulError;

// TODO: implement Error trait
#[derive(Debug)]
pub enum Error {
    Spine(String),
    NulError(NulError),
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
