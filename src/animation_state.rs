use std::{ffi::CString, sync::Arc};

use crate::{
    animation_state_data::AnimationStateData,
    c::{
        spAnimationState, spAnimationStateData, spAnimationState_apply, spAnimationState_create,
        spAnimationState_dispose, spAnimationState_setAnimationByName, spAnimationState_update,
    },
    error::Error,
    skeleton::Skeleton,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct AnimationState {
    c_animation_state: SyncPtr<spAnimationState>,
    owns_memory: bool,
    _animation_state_data: Arc<AnimationStateData>,
}

impl AnimationState {
    pub fn new(animation_state_data: Arc<AnimationStateData>) -> Self {
        let c_animation_state = unsafe { spAnimationState_create(animation_state_data.c_ptr()) };
        Self {
            c_animation_state: SyncPtr(c_animation_state),
            owns_memory: true,
            _animation_state_data: animation_state_data,
        }
    }

    pub unsafe fn set_animation_by_name_unchecked(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
    ) {
        let c_animation_name = CString::new(animation_name).unwrap();
        spAnimationState_setAnimationByName(
            self.c_ptr(),
            track_index,
            c_animation_name.as_ptr(),
            looping as i32,
        );
    }

    pub fn set_animation_by_name(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
    ) -> Result<(), Error> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .find(|animation| animation.name() == animation_name)
            .is_some()
        {
            unsafe { self.set_animation_by_name_unchecked(track_index, animation_name, looping) };
            Ok(())
        } else {
            Err(Error::NotFound)
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
    c_accessor_tmp_ptr!(
        data,
        data_mut,
        data,
        AnimationStateData,
        spAnimationStateData
    );
}

impl Drop for AnimationState {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spAnimationState_dispose(self.c_animation_state.0);
            }
        }
    }
}
