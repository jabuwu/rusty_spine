//! Direct access to the [Spine C Runtime](http://en.esotericsoftware.com/spine-c).
//!
//! Transpiled from spine-c commit
//! [f85fb25a35d69a659e3d8d01a99b1fc93f759778](https://github.com/EsotericSoftware/spine-runtimes/tree/f85fb25a35d69a659e3d8d01a99b1fc93f759778)
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
