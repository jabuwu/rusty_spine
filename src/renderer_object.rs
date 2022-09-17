use crate::{
    atlas::AtlasRegion,
    c::{c_void, spAtlasRegion},
    c_interface::NewFromPtr,
    tmp_ref::TmpRef,
};

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

    pub unsafe fn get_atlas_region(&mut self) -> TmpRef<Self, AtlasRegion> {
        TmpRef::new(
            self,
            AtlasRegion::new_from_ptr(*self.renderer_object as *mut spAtlasRegion),
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
