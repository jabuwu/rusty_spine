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

    /// Set the renderer object to a Rust object. Panics if the renderer object is already set.
    /// Must be manually freed later using [dispose](#method.dispose).
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

    /// Gets the type pointed to by this renderer object. It is not guaranteed that the returned
    /// option has a valid object and could segfault if the renderer object is a different type.
    pub unsafe fn get<T>(&mut self) -> Option<&mut T> {
        let ptr = *self.renderer_object;
        if !ptr.is_null() {
            Some(&mut *(*self.renderer_object as *mut T))
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked<T>(&mut self) -> &mut T {
        &mut *(*self.renderer_object as *mut T)
    }

    /// Gets the atlas region on mesh and region attachments if the default attachment loader was
    /// used to create the skeleton. This function does not guarantee that the returned option
    /// has a valid AtlasRegion and could segfault if the renderer object is a different type.
    pub unsafe fn get_atlas_region(&mut self) -> Option<CTmpRef<Self, AtlasRegion>> {
        let ptr = *self.renderer_object;
        if !ptr.is_null() {
            Some(CTmpRef::new(
                self,
                AtlasRegion::new_from_ptr(ptr as *mut spAtlasRegion),
            ))
        } else {
            None
        }
    }

    /// Drop the underlying data. This only works for Rust types and might segfault if the type
    /// was allocated in C.
    pub unsafe fn dispose<T>(&mut self) {
        if !self.renderer_object.is_null() {
            drop(Box::from_raw(*self.renderer_object as *mut T));
            *self.renderer_object = std::ptr::null_mut();
        }
    }

    /// Set renderer object to null, potentially leaking the memory previously pointed to.
    pub fn forget(&mut self) {
        *self.renderer_object = std::ptr::null_mut();
    }
}
