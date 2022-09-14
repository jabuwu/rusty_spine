use std::{ffi::CString, sync::Arc};

use crate::{
    animation_state_data::AnimationStateData,
    c::{
        spAnimationState, spAnimationState_apply, spAnimationState_create,
        spAnimationState_dispose, spAnimationState_setAnimationByName, spAnimationState_update,
    },
    skeleton::Skeleton,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct AnimationState {
    c_animation_state: SyncPtr<spAnimationState>,
    _animation_state_data: Arc<AnimationStateData>,
}

impl AnimationState {
    pub fn new(animation_state_data: Arc<AnimationStateData>) -> Self {
        let c_animation_state = unsafe { spAnimationState_create(animation_state_data.c_ptr()) };
        Self {
            c_animation_state: SyncPtr(c_animation_state),
            _animation_state_data: animation_state_data,
        }
    }

    pub fn set_animation_by_name(&mut self, track_index: i32, animation_name: &str, looping: bool) {
        let c_animation_name = CString::new(animation_name).unwrap();
        unsafe {
            spAnimationState_setAnimationByName(
                self.c_animation_state.0,
                track_index,
                c_animation_name.as_ptr(),
                looping as i32,
            );
        }
    }

    pub fn update(&mut self, delta: f32) {
        unsafe {
            spAnimationState_update(self.c_animation_state.0, delta);
        }
    }

    pub fn apply(&self, skeleton: &mut Skeleton) -> bool {
        unsafe { spAnimationState_apply(self.c_animation_state.0, skeleton.c_ptr()) != 0 }
    }

    c_ptr!(c_animation_state, spAnimationState);
}

impl Drop for AnimationState {
    fn drop(&mut self) {
        unsafe {
            spAnimationState_dispose(self.c_animation_state.0);
        }
    }
}
