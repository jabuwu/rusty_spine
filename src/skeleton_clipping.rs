use crate::{
    c::{
        spClippingAttachment, spSkeletonClipping, spSkeletonClipping_clipEnd,
        spSkeletonClipping_clipEnd2, spSkeletonClipping_clipStart, spSkeletonClipping_create,
    },
    clipping_attachment::ClippingAttachment,
    slot::Slot,
    sync_ptr::SyncPtr,
};

pub struct SkeletonClipping {
    c_skeleton_clipping: SyncPtr<spSkeletonClipping>,
}

impl SkeletonClipping {
    pub fn new() -> Self {
        Self {
            c_skeleton_clipping: unsafe { SyncPtr(spSkeletonClipping_create()) },
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
