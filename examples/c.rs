#[cfg(feature = "spine38")]
compile_error!("This example does not work with Spine 3.8");

use std::{
    ffi::{CStr, CString},
    fs::read,
};

use rusty_spine::c::spAtlas_createFromFile;

fn main() {
    rusty_spine::extension::set_read_file_cb(|path| read(path).map_or(None, |data| Some(data)));
    let c_atlas_path = CString::new("./assets/spineboy/export/spineboy.atlas").unwrap();
    let c_atlas = unsafe { spAtlas_createFromFile(c_atlas_path.as_ptr(), std::ptr::null_mut()) };
    let first_page = unsafe { (*c_atlas).pages };
    println!("Found atlas page: {:?}", unsafe {
        CStr::from_ptr((*first_page).name)
    });
}
