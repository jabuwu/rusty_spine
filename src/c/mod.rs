mod spine_c;
mod wasm;

pub use spine_c::*;

mod environment {
    pub use super::wasm::*;
}

#[no_mangle]
extern "C" fn _spAtlasPage_createTexture(
    _s: *mut spAtlasPage,
    _path: *const environment::types::c_char,
) {
}

#[no_mangle]
extern "C" fn _spAtlasPage_disposeTexture(_s: *mut spAtlasPage) {}

#[no_mangle]
extern "C" fn _spUtil_readFile(
    _path: *const environment::types::c_char,
    _length: *mut environment::types::c_int,
) -> *mut environment::types::c_char {
    return std::ptr::null_mut();
}
