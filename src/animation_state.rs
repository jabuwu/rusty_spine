use std::{ffi::CString, sync::Arc};

use crate::{
    animation::Animation,
    animation_state_data::AnimationStateData,
    c::{
        spAnimation, spAnimationState, spAnimationStateData, spAnimationState_addAnimation,
        spAnimationState_addAnimationByName, spAnimationState_addEmptyAnimation,
        spAnimationState_apply, spAnimationState_clearListenerNotifications,
        spAnimationState_clearNext, spAnimationState_clearTrack, spAnimationState_clearTracks,
        spAnimationState_create, spAnimationState_dispose, spAnimationState_disposeStatics,
        spAnimationState_getCurrent, spAnimationState_setAnimation,
        spAnimationState_setAnimationByName, spAnimationState_setEmptyAnimation,
        spAnimationState_setEmptyAnimations, spAnimationState_update, spTrackEntry,
        spTrackEntry_getAnimationTime, spTrackEntry_getTrackComplete,
    },
    c_interface::NewFromPtr,
    error::Error,
    skeleton::Skeleton,
    sync_ptr::SyncPtr,
    tmp_ref::TmpRef,
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

    pub fn get_current(&self, track_index: i32) -> Option<TmpRef<Self, TrackEntry>> {
        unsafe {
            let ptr = spAnimationState_getCurrent(self.c_ptr(), track_index);
            if !ptr.is_null() {
                Some(TmpRef::new(self, TrackEntry::new_from_ptr(ptr)))
            } else {
                None
            }
        }
    }

    pub fn clear_listener_notifications(&mut self) {
        unsafe {
            spAnimationState_clearListenerNotifications(self.c_ptr());
        }
    }

    pub fn clear_next(&mut self, entry: &TrackEntry) {
        unsafe {
            spAnimationState_clearNext(self.c_ptr(), entry.c_ptr());
        }
    }

    c_ptr!(c_animation_state, spAnimationState);
    c_accessor_tmp_ptr!(
        data,
        data_mut,
        data,
        AnimationStateData,
        spAnimationStateData
    );
    c_accessor!(tracks_count, tracks_count_mut, tracksCount, i32);
    c_accessor_array!(
        tracks,
        tracks_mut,
        track_at_index,
        track_at_index_mut,
        AnimationState,
        TrackEntry,
        spTrackEntry,
        tracks,
        tracks_count
    );
    c_accessor!(timescale, timescale_mut, timeScale, f32);
    c_accessor!(unkeyed_state, unkeyed_state_mut, unkeyedState, i32);
    c_accessor_renderer_object!();

    /*TODO
    spAnimationStateListener listener;
    void *userData;
    */

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

#[derive(Debug)]
pub struct TrackEntry {
    c_track_entry: SyncPtr<spTrackEntry>,
}

impl NewFromPtr<spTrackEntry> for TrackEntry {
    unsafe fn new_from_ptr(c_track_entry: *const spTrackEntry) -> Self {
        Self {
            c_track_entry: SyncPtr(c_track_entry as *mut spTrackEntry),
        }
    }
}

impl TrackEntry {
    pub fn get_animation_time(&self) -> f32 {
        unsafe { spTrackEntry_getAnimationTime(self.c_ptr()) }
    }

    pub fn get_track_complete(&self) -> f32 {
        unsafe { spTrackEntry_getTrackComplete(self.c_ptr()) }
    }

    c_ptr!(c_track_entry, spTrackEntry);
    c_accessor_tmp_ptr!(animation, animation_mut, animation, Animation, spAnimation);
    c_accessor!(track_index, track_index_mut, trackIndex, i32);
    c_accessor_bool!(looping, set_looping, loop_0);
    c_accessor_bool!(hold_previous, set_hold_previous, holdPrevious);
    c_accessor_bool!(reverse, set_reverse, reverse);
    c_accessor_bool!(shortest_rotation, set_shortest_rotation, shortestRotation);
    c_accessor!(event_threshold, event_threshold_mut, eventThreshold, f32);
    c_accessor!(
        attachment_threshold,
        attachment_threshold_mut,
        attachmentThreshold,
        f32
    );
    c_accessor!(
        draw_order_threshold,
        draw_order_threshold_mut,
        drawOrderThreshold,
        f32
    );
    c_accessor!(delay, delay_mut, delay, f32);
    c_accessor!(track_time, track_time_mut, trackTime, f32);
    c_accessor!(track_last, track_last_mut, trackLast, f32);
    c_accessor!(next_track_last, next_track_last_mut, nextTrackLast, f32);
    c_accessor!(track_end, track_end_mut, trackEnd, f32);
    c_accessor!(timescale, timescale_mut, timeScale, f32);
    c_accessor!(alpha, alpha_mut, alpha, f32);
    c_accessor!(mix_time, mix_time_mut, mixTime, f32);
    c_accessor!(mix_duration, mix_duration_mut, mixDuration, f32);
    c_accessor!(interrupt_alpha, interrupt_alpha_mut, interruptAlpha, f32);
    c_accessor!(total_alpha, total_alpha_mut, totalAlpha, f32);

    /*TODO
    spAnimation *animation;
    spTrackEntry *previous;
    spTrackEntry *next;
    spTrackEntry *mixingFrom;
    spTrackEntry *mixingTo;
    spAnimationStateListener listener;
    spMixBlend mixBlend;
    spIntArray *timelineMode;
    spTrackEntryArray *timelineHoldMix;
    float *timelinesRotation;
    int timelinesRotationCount;
    void *rendererObject;
    void *userData;*/
}
