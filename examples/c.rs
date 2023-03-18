use std::{
    ffi::{CStr, CString},
    fs::read,
};

use rusty_spine::c::spAtlas_createFromFile;

fn main() {
    rusty_spine::extension::set_read_file_cb(|path| read(path).ok());
    let c_atlas_path = CString::new("./assets/spineboy/export/spineboy.atlas").unwrap();
    let c_atlas = unsafe { spAtlas_createFromFile(c_atlas_path.as_ptr(), std::ptr::null_mut()) };
    let first_page = unsafe { (*c_atlas).pages };
    println!("Found atlas page: {:?}", unsafe {
        CStr::from_ptr((*first_page).name)
    });
}
