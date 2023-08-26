//! Allow overriding the Spine C Runtime extension functions.
//!
//! Because this library is primarily a wrapper around the official C runtime, it tries to mimic
//! the usage of that runtime as closely as possible. One awkward outcome of this goal is that the
//! C runtime expects you to implement a few functions directly. This module allows you to set
//! those callbacks which have been wrapped in Rust.
//!
//! The callbacks from C are:
//!
//! * `void _spAtlasPage_createTexture (spAtlasPage* self, const char* path);`
//! * `void _spAtlasPage_disposeTexture (spAtlasPage* self);`
//! * `char* _spUtil_readFile (const char* path, int* length);`
//!
//! They can be set with the functions found on this page.
//!
//! You can read more about these functions on the
//! [spine-c Runtime Docs](http://en.esotericsoftware.com/spine-c#Integrating-spine-c-in-your-engine).

use std::ffi::CStr;
use std::fs::read;
use std::sync::{Arc, Mutex, Once};

use crate::c::{c_int, c_void, size_t};
use crate::c_interface::NewFromPtr;
use crate::{
    atlas::AtlasPage,
    c::{c_char, spAtlasPage},
};

type CreateTextureCb = Box<dyn Fn(&mut AtlasPage, &str)>;
type DisposeTextureCb = Box<dyn Fn(&mut AtlasPage)>;
type ReadFileCb = Box<dyn Fn(&str) -> Option<Vec<u8>>>;

#[derive(Default)]
pub(crate) struct Extension {
    create_texture_cb: Option<CreateTextureCb>,
    dispose_texture_cb: Option<DisposeTextureCb>,
    read_file_cb: Option<ReadFileCb>,
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

/// Set `_spAtlasPage_createTexture`
///
/// The purpose of this callback is to allow loading textures in whichever engine is being used.
/// The following example shows the intended usage by storing the texture on the renderer object
/// of the [`AtlasPage`] which can be acquired later.
/// ```
/// struct SpineTexture(pub String);
///
/// rusty_spine::extension::set_create_texture_cb(|atlas_page, path| {
///     atlas_page.renderer_object().set(SpineTexture(path.to_owned()));
/// });
/// rusty_spine::extension::set_dispose_texture_cb(|atlas_page| unsafe {
///     atlas_page.renderer_object().dispose::<SpineTexture>();
/// });
/// ```
///
/// If using the [`SimpleDrawer`](`crate::draw::SimpleDrawer`) or
/// [`CombinedDrawer`](`crate::draw::CombinedDrawer`), the texture can be acquired from
/// [`SimpleRenderable::attachment_renderer_object`](`crate::draw::SimpleRenderable::attachment_renderer_object`)
/// or
/// [`CombinedRenderable::attachment_renderer_object`](`crate::draw::CombinedRenderable::attachment_renderer_object`)
/// respectively.
///
/// If using the [`SkeletonController`](`crate::controller::SkeletonController`), see
/// [`SkeletonRenderable::attachment_renderer_object`](`crate::controller::SkeletonRenderable::attachment_renderer_object`)
/// or
/// [`SkeletonCombinedRenderable::attachment_renderer_object`](`crate::controller::SkeletonCombinedRenderable::attachment_renderer_object`)
///
/// ```
/// # #[path="./test.rs"]
/// # mod test;
/// # use rusty_spine::{AnimationState, EventType, controller::SkeletonController};
/// # let (mut skeleton_data, mut animation_state_data) = test::TestAsset::spineboy().instance_data();
///
/// struct SpineTexture(pub String); // from example above
///
/// // ...
///
/// let mut skeleton_controller = SkeletonController::new(skeleton_data, animation_state_data);
/// let renderables = skeleton_controller.renderables();
/// for renderable in renderables.iter() {
///     if let Some(attachment_renderer_object) = renderable.attachment_renderer_object {
///         let texture = unsafe { &mut *(attachment_renderer_object as *mut SpineTexture) };
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if the internal mutex is poisoned.
pub fn set_create_texture_cb<F>(create_texture_cb: F)
where
    F: Fn(&mut AtlasPage, &str) + 'static,
{
    let singleton = Extension::singleton();
    let mut extension = singleton.lock().unwrap();
    extension.create_texture_cb = Some(Box::new(create_texture_cb));
}

/// Set `_spAtlasPage_disposeTexture`
///
/// For an example, see [`set_create_texture_cb`].
///
/// # Panics
///
/// Panics if the internal mutex is poisoned.
pub fn set_dispose_texture_cb<F>(dispose_texture_cb: F)
where
    F: Fn(&mut AtlasPage) + 'static,
{
    let singleton = Extension::singleton();
    let mut extension = singleton.lock().unwrap();
    extension.dispose_texture_cb = Some(Box::new(dispose_texture_cb));
}

/// Set `_spUtil_readFile`
///
/// Can be used to customize file loading when using functions which read files from disk. This
/// callback is largely unnecessary as its possible to avoid calling these sorts of functions
/// if read-from-disk is not desirable. Additionally, a default implementation using Rust's
/// `std::fs::read` is provided if this callback remains unset.
///
/// ```
/// rusty_spine::extension::set_read_file_cb(|path| {
///     std::fs::read(path).ok()
/// });
/// ```
///
/// # Panics
///
/// Panics if the internal mutex is poisoned.
pub fn set_read_file_cb<F>(read_file_cb: F)
where
    F: Fn(&str) -> Option<Vec<u8>> + 'static,
{
    let singleton = Extension::singleton();
    let mut extension = singleton.lock().unwrap();
    extension.read_file_cb = Some(Box::new(read_file_cb));
}

#[no_mangle]
extern "C" fn _spAtlasPage_createTexture(c_atlas_page: *mut spAtlasPage, c_path: *const c_char) {
    let singleton = Extension::singleton();
    let extension = singleton.lock().unwrap();
    if let Some(cb) = &extension.create_texture_cb {
        unsafe {
            cb(
                &mut AtlasPage::new_from_ptr(c_atlas_page),
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
        unsafe {
            cb(&mut AtlasPage::new_from_ptr(c_atlas_page));
        }
    }
}

extern "C" {
    fn spine_malloc(__size: size_t) -> *mut c_void;
    fn spine_memcpy(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
}

#[no_mangle]
extern "C" fn _spUtil_readFile(c_path: *const c_char, c_length: *mut c_int) -> *mut c_char {
    let singleton = Extension::singleton();
    let extension = singleton.lock().unwrap();
    extension.read_file_cb.as_ref().map_or_else(
        || {
            let str = unsafe { CStr::from_ptr(c_path).to_str().unwrap().to_owned() };
            read(str).map_or(std::ptr::null_mut(), |data| {
                let c_data = unsafe { spine_malloc(data.len() as size_t) };
                unsafe {
                    spine_memcpy(c_data, data.as_ptr().cast::<c_void>(), data.len() as size_t);
                    *c_length = data.len() as c_int;
                }
                c_data.cast::<c_char>()
            })
        },
        |cb| {
            cb(unsafe { CStr::from_ptr(c_path).to_str().unwrap() }).map_or(
                std::ptr::null_mut(),
                |data| unsafe {
                    *c_length = data.len() as c_int;
                    let c_data = spine_malloc(data.len() as size_t);
                    spine_memcpy(c_data, data.as_ptr().cast::<c_void>(), data.len() as size_t);
                    c_data.cast::<c_char>()
                },
            )
        },
    )
}
