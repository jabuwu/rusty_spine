use std::ffi::CStr;
use std::sync::{Arc, Mutex, Once};

use crate::{
    atlas::AtlasPage,
    c::{c_char, spAtlasPage},
};

#[derive(Default)]
pub struct Extension {
    create_texture_cb: Option<Box<dyn Fn(&AtlasPage, &str)>>,
    dispose_texture_cb: Option<Box<dyn Fn(&AtlasPage)>>,
}

impl Extension {
    fn singleton() -> Arc<Mutex<Extension>> {
        static START: Once = Once::new();
        static mut INSTANCE: Option<Arc<Mutex<Extension>>> = None;
        START.call_once(|| unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(Extension::default())));
        });
        unsafe {
            let singleton = INSTANCE.as_ref().unwrap();
            singleton.clone()
        }
    }
}

pub fn set_create_texture_cb<F>(create_texture_cb: F)
where
    F: Fn(&AtlasPage, &str) + 'static,
{
    let singleton = Extension::singleton();
    let mut extension = singleton.lock().unwrap();
    extension.create_texture_cb = Some(Box::new(create_texture_cb));
}

pub fn set_dispose_texture_cb<F>(dispose_texture_cb: F)
where
    F: Fn(&AtlasPage) + 'static,
{
    let singleton = Extension::singleton();
    let mut extension = singleton.lock().unwrap();
    extension.dispose_texture_cb = Some(Box::new(dispose_texture_cb));
}

#[no_mangle]
extern "C" fn _spAtlasPage_createTexture(c_atlas_page: *mut spAtlasPage, c_path: *const c_char) {
    let singleton = Extension::singleton();
    let extension = singleton.lock().unwrap();
    if let Some(cb) = &extension.create_texture_cb {
        unsafe {
            cb(
                &AtlasPage::new_from_ptr(c_atlas_page),
                CStr::from_ptr(c_path).to_str().unwrap(),
            );
        }
    }
}

#[no_mangle]
extern "C" fn _spAtlasPage_disposeTexture(c_atlas_page: *mut spAtlasPage) {
    let singleton = Extension::singleton();
    let extension = singleton.lock().unwrap();
    if let Some(cb) = &extension.dispose_texture_cb {
        cb(&AtlasPage::new_from_ptr(c_atlas_page));
    }
}
