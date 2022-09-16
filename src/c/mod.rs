mod spine_c;

#[cfg(not(feature = "libc"))]
mod wasm;

#[cfg(feature = "libc")]
mod libc;

pub use spine_c::*;

mod environment {
    #[cfg(not(feature = "libc"))]
    pub use super::wasm::*;

    #[cfg(feature = "libc")]
    pub use super::libc::*;
}
