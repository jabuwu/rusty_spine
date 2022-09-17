use std::{ffi::CString, sync::Arc};

use crate::{
    animation::Animation,
    animation_state_data::AnimationStateData,
    c::{
        spAnimationState, spAnimationStateData, spAnimationState_addAnimation,
        spAnimationState_addAnimationByName, spAnimationState_addEmptyAnimation,
        spAnimationState_apply, spAnimationState_clearListenerNotifications,
        spAnimationState_clearTrack, spAnimationState_clearTracks, spAnimationState_create,
        spAnimationState_dispose, spAnimationState_disposeStatics, spAnimationState_setAnimation,
        spAnimationState_setAnimationByName, spAnimationState_setEmptyAnimation,
        spAnimationState_setEmptyAnimations, spAnimationState_update,
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

    pub fn update(&mut self, delta: f32) {
        unsafe {
            spAnimationState_update(self.c_animation_state.0, delta);
        }
    }

    pub fn apply(&self, skeleton: &mut Skeleton) -> bool {
        unsafe { spAnimationState_apply(self.c_animation_state.0, skeleton.c_ptr()) != 0 }
    }

    pub fn clear_tracks(&mut self) {
        unsafe {
            spAnimationState_clearTracks(self.c_ptr());
        }
    }

    pub fn clear_track(&mut self, track_index: i32) {
        unsafe {
            spAnimationState_clearTrack(self.c_ptr(), track_index);
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

    pub fn set_animation(&mut self, track_index: i32, animation: &Animation, looping: bool) {
        unsafe {
            spAnimationState_setAnimation(
                self.c_ptr(),
                track_index,
                animation.c_ptr(),
                looping as i32,
            );
        }
    }

    pub unsafe fn add_animation_by_name_unchecked(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) {
        let c_animation_name = CString::new(animation_name).unwrap();
        spAnimationState_addAnimationByName(
            self.c_ptr(),
            track_index,
            c_animation_name.as_ptr(),
            looping as i32,
            delay,
        );
    }

    pub fn add_animation_by_name(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) -> Result<(), Error> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .find(|animation| animation.name() == animation_name)
            .is_some()
        {
            unsafe {
                self.add_animation_by_name_unchecked(track_index, animation_name, looping, delay)
            };
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn add_animation(
        &mut self,
        track_index: i32,
        animation: &Animation,
        looping: bool,
        delay: f32,
    ) {
        unsafe {
            spAnimationState_addAnimation(
                self.c_ptr(),
                track_index,
                animation.c_ptr(),
                looping as i32,
                delay,
            );
        }
    }

    pub fn set_empty_animation(&mut self, track_index: i32, mix_duration: f32) {
        unsafe {
            spAnimationState_setEmptyAnimation(self.c_ptr(), track_index, mix_duration);
        }
    }

    pub fn add_empty_animation(&mut self, track_index: i32, mix_duration: f32, delay: f32) {
        unsafe {
            spAnimationState_addEmptyAnimation(self.c_ptr(), track_index, mix_duration, delay);
        }
    }

    pub fn set_empty_animations(&mut self, mix_duration: f32) {
        unsafe {
            spAnimationState_setEmptyAnimations(self.c_ptr(), mix_duration);
        }
    }

    // TODO: SP_API spTrackEntry *spAnimationState_getCurrent(spAnimationState *self, int trackIndex);

    pub fn clear_listener_notifications(&mut self) {
        unsafe {
            spAnimationState_clearListenerNotifications(self.c_ptr());
        }
    }

    // TODO: SP_API float spTrackEntry_getAnimationTime(spTrackEntry *entry);
    // TODO: SP_API float spTrackEntry_getTrackComplete(spTrackEntry *entry);
    // TODO: SP_API void spAnimationState_clearNext(spAnimationState *self, spTrackEntry *entry);

    c_ptr!(c_animation_state, spAnimationState);
    c_accessor_tmp_ptr!(
        data,
        data_mut,
        data,
        AnimationStateData,
        spAnimationStateData
    );

    pub fn dispose_statics() {
        unsafe {
            spAnimationState_disposeStatics();
        }
    }
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
