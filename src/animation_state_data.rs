use std::sync::Arc;

use crate::{
    c::{
        c_void, spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose,
        spSkeletonData,
    },
    c_interface::NewFromPtr,
    skeleton_data::SkeletonData,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct AnimationStateData {
    c_animation_state_data: SyncPtr<spAnimationStateData>,
    owns_memory: bool,
    _skeleton_data: Option<Arc<SkeletonData>>,
}

impl NewFromPtr<spAnimationStateData> for AnimationStateData {
    unsafe fn new_from_ptr(c_animation_state_data: *const spAnimationStateData) -> Self {
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data as *mut spAnimationStateData),
            owns_memory: false,
            _skeleton_data: None,
        }
    }
}

impl AnimationStateData {
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_animation_state_data = unsafe { spAnimationStateData_create(skeleton_data.c_ptr()) };
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data),
            owns_memory: true,
            _skeleton_data: Some(skeleton_data),
        }
    }

    c_ptr!(c_animation_state_data, spAnimationStateData);
    c_accessor_tmp_ptr!(
        skeleton_data,
        skeleton_data_mut,
        skeletonData,
        SkeletonData,
        spSkeletonData
    );
    c_accessor!(default_mix, set_default_mix, defaultMix, f32);
    c_accessor_passthrough!(entries, entries_mut, entries, *const c_void, *mut c_void);
}

impl Drop for AnimationStateData {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spAnimationStateData_dispose(self.c_animation_state_data.0);
            }
        }
    }
}
