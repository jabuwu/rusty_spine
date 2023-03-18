//! Direct access to the [Spine C Runtime](http://en.esotericsoftware.com/spine-c).
//!
//! Transpiled from spine-c commit
//! [7a5c6995effdeaf6c4846884aae0d7fe37982f9f](https://github.com/EsotericSoftware/spine-runtimes/tree/7a5c6995effdeaf6c4846884aae0d7fe37982f9f)
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
