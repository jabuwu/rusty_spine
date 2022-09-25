//! Direct access to the [Spine C Runtime](http://en.esotericsoftware.com/spine-c).
//!
//! Transpiled from spine-c commit
//! [d9935741c2b84abea85e961489ed71b5b473aa64](https://github.com/EsotericSoftware/spine-runtimes/tree/d9935741c2b84abea85e961489ed71b5b473aa64)
//!
//! Transpiled using c2rust commit [00ace7631ee3b5cc4447a0562c62b82eedd27812](https://github.com/immunant/c2rust/tree/00ace7631ee3b5cc4447a0562c62b82eedd27812)
#[cfg(not(feature = "libc"))]
#[macro_use]
mod wasm;

#[cfg(feature = "libc")]
#[macro_use]
mod libc;

mod spine_c;

pub use self::spine_c::*;

mod environment {
    #[cfg(not(feature = "libc"))]
    pub use super::wasm::*;

    #[cfg(feature = "libc")]
    pub use super::libc::*;
}
