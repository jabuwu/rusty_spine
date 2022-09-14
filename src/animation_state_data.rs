use std::sync::Arc;

use crate::{
    c::{spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose},
    skeleton_data::SkeletonData,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct AnimationStateData {
    c_animation_state_data: SyncPtr<spAnimationStateData>,
    _skeleton_data: Arc<SkeletonData>,
}

impl AnimationStateData {
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_animation_state_data = unsafe { spAnimationStateData_create(skeleton_data.c_ptr()) };
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data),
            _skeleton_data: skeleton_data,
        }
    }

    pub fn c_ptr(&self) -> *mut spAnimationStateData {
        self.c_animation_state_data.0
    }
}

impl Drop for AnimationStateData {
    fn drop(&mut self) {
        unsafe {
            spAnimationStateData_dispose(self.c_animation_state_data.0);
        }
    }
}
