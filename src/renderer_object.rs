use crate::{
    atlas::AtlasRegion,
    c::{c_void, spAtlasRegion},
    c_interface::CTmpRef,
    c_interface::NewFromPtr,
};

/// A wrapper around a user data void pointer found on a few Spine C structs.
///
/// The Spine C runtime sets this automatically to the attachment's atlas region for region and mesh
/// attachments if using the default atlas attachment loader. In this case,
/// [get_atlas_region](#method.get_atlas_region) can be used to get the Rust struct.
///
/// The value can be set manually but will panic if the value is already set. The previous value
/// can be disposed using [dispose](#method.dispose), but only if the value was allocated in Rust.
/// The value can be forgotten using [forget](#method.forget), but this can cause a memory leak.
pub struct RendererObject<'a> {
    renderer_object: &'a mut *mut c_void,
}

impl<'a> RendererObject<'a> {
    pub(crate) fn new(renderer_object: &'a mut *mut c_void) -> Self {
        Self { renderer_object }
    }

    pub fn set<T>(&mut self, data: T) {
        if !self.renderer_object.is_null() {
            panic!("Setting renderer object when it's already set.");
        }
        let ptr = Box::leak(Box::new(data)) as *mut T as *mut c_void;
        if ptr == 1 as *mut c_void {
            panic!(
                "Renderer object cannot be set to an empty type, please add member variables: {}",
                std::any::type_name::<T>()
            );
        }
        *self.renderer_object = ptr;
    }

    pub unsafe fn get<T>(&mut self) -> &mut T {
        &mut *(*self.renderer_object as *mut T)
    }

    pub unsafe fn get_atlas_region(&mut self) -> CTmpRef<Self, AtlasRegion> {
        CTmpRef::new(
            self,
            AtlasRegion::new_from_ptr(*self.renderer_object as *mut spAtlasRegion),
            None,
        )
    }

    pub unsafe fn dispose<T>(&mut self) {
        if !self.renderer_object.is_null() {
            drop(Box::from_raw(*self.renderer_object as *mut T));
            *self.renderer_object = std::ptr::null_mut();
        }
    }

    pub fn forget(&mut self) {
        *self.renderer_object = std::ptr::null_mut();
    }
}
