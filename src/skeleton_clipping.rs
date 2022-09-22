use crate::{
    c::{
        spClippingAttachment, spSkeletonClipping, spSkeletonClipping_clipEnd,
        spSkeletonClipping_clipEnd2, spSkeletonClipping_clipStart, spSkeletonClipping_create,
        spSkeletonClipping_dispose, spSkeletonClipping_isClipping,
    },
    c_interface::SyncPtr,
    clipping_attachment::ClippingAttachment,
    slot::Slot,
};

#[derive(Debug)]
pub struct SkeletonClipping {
    c_skeleton_clipping: SyncPtr<spSkeletonClipping>,
    owns_memory: bool,
}

impl SkeletonClipping {
    pub fn new() -> Self {
        Self {
            c_skeleton_clipping: unsafe { SyncPtr(spSkeletonClipping_create()) },
            owns_memory: true,
        }
    }

    pub fn clip_start(&mut self, slot: &Slot, clip: &ClippingAttachment) {
        unsafe {
            spSkeletonClipping_clipStart(
                self.c_ptr_mut(),
                slot.c_ptr(),
                clip.c_ptr() as *mut spClippingAttachment,
            );
        }
    }

    pub fn clip_end(&mut self, slot: &Slot) {
        unsafe {
            spSkeletonClipping_clipEnd(self.c_ptr_mut(), slot.c_ptr());
        }
    }

    pub fn clip_end2(&mut self) {
        unsafe {
            spSkeletonClipping_clipEnd2(self.c_ptr_mut());
        }
    }

    pub fn is_clipping(&self) -> bool {
        unsafe { spSkeletonClipping_isClipping(self.c_ptr_mut()) != 0 }
    }

    c_ptr!(c_skeleton_clipping, spSkeletonClipping);
    /*spTriangulator *triangulator;
    spFloatArray *clippingPolygon;
    spFloatArray *clipOutput;
    spFloatArray *clippedVertices;
    spFloatArray *clippedUVs;
    spUnsignedShortArray *clippedTriangles;
    spFloatArray *scratch;
    spClippingAttachment *clipAttachment;
    spArrayFloatArray *clippingPolygons;*/
}

impl Drop for SkeletonClipping {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeletonClipping_dispose(self.c_skeleton_clipping.0);
            }
        }
    }
}
