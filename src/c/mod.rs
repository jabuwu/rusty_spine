//! Direct access to the [Spine C Runtime](http://en.esotericsoftware.com/spine-c).
//!
//! spine_c.rs transpiled from spine-c commit
//! [674fab37f0d7e2dedb4ab4c32ae09dcb8ca67ca3](https://github.com/EsotericSoftware/spine-runtimes/tree/674fab37f0d7e2dedb4ab4c32ae09dcb8ca67ca3)
//!
//! Transpiled using c2rust commit [00ace7631ee3b5cc4447a0562c62b82eedd27812](https://github.com/immunant/c2rust/tree/00ace7631ee3b5cc4447a0562c62b82eedd27812)
//!
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
