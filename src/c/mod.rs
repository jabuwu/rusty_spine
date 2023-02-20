//! Direct access to the [Spine C Runtime](http://en.esotericsoftware.com/spine-c).
//!
//! Transpiled from spine-c commit
//! [de808262bb84f61f24627c96a50a25370390af76](https://github.com/EsotericSoftware/spine-runtimes/tree/de808262bb84f61f24627c96a50a25370390af76)
//!
//! Transpiled using c2rust commit [0a2b64b4f83b42f08fe13c3d4fbd8b5b167f07a8](https://github.com/immunant/c2rust/tree/0a2b64b4f83b42f08fe13c3d4fbd8b5b167f07a8)
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
