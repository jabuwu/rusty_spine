use std::{ffi::CString, sync::Arc};

use crate::{
    animation::Animation,
    animation_state_data::AnimationStateData,
    c::{
        c_void, spAnimation, spAnimationState, spAnimationStateData, spAnimationState_addAnimation,
        spAnimationState_addAnimationByName, spAnimationState_addEmptyAnimation,
        spAnimationState_apply, spAnimationState_clearListenerNotifications,
        spAnimationState_clearNext, spAnimationState_clearTrack, spAnimationState_clearTracks,
        spAnimationState_create, spAnimationState_dispose, spAnimationState_disposeStatics,
        spAnimationState_getCurrent, spAnimationState_setAnimation,
        spAnimationState_setAnimationByName, spAnimationState_setEmptyAnimation,
        spAnimationState_setEmptyAnimations, spAnimationState_update, spEvent, spEventType,
        spTrackEntry, spTrackEntry_getAnimationTime, spTrackEntry_getTrackComplete,
    },
    c_interface::CTmpRef,
    c_interface::NewFromPtr,
    error::Error,
    event::Event,
    skeleton::Skeleton,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct AnimationState {
    c_animation_state: SyncPtr<spAnimationState>,
    owns_memory: bool,
    _animation_state_data: Option<Arc<AnimationStateData>>,
}

impl NewFromPtr<spAnimationState> for AnimationState {
    unsafe fn new_from_ptr(c_animation_state: *const spAnimationState) -> Self {
        Self {
            c_animation_state: SyncPtr(c_animation_state as *mut spAnimationState),
            owns_memory: false,
            _animation_state_data: None,
        }
    }
}

// TODO: track entries are going to require their own memory management solution
// they can be disposed when finished (and an event is fired)
impl AnimationState {
    pub fn new(animation_state_data: Arc<AnimationStateData>) -> Self {
        let c_animation_state = unsafe { spAnimationState_create(animation_state_data.c_ptr()) };
        unsafe {
            (*c_animation_state).userData = Box::leak(Box::new(AnimationStateUserData::default()))
                as *mut AnimationStateUserData
                as *mut c_void;
        }
        Self {
            c_animation_state: SyncPtr(c_animation_state),
            owns_memory: true,
            _animation_state_data: Some(animation_state_data),
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

    pub fn get_current(&self, track_index: i32) -> Option<CTmpRef<Self, TrackEntry>> {
        unsafe {
            let ptr = spAnimationState_getCurrent(self.c_ptr(), track_index);
            if !ptr.is_null() {
                Some(CTmpRef::new(self, TrackEntry::new_from_ptr(ptr), None))
            } else {
                None
            }
        }
    }

    pub fn set_listener<F>(&mut self, listener: F)
    where
        F: Fn(&AnimationState, EventType, &TrackEntry, Option<&Event>) + 'static,
    {
        extern "C" fn c_listener(
            c_animation_state: *mut spAnimationState,
            c_event_type: spEventType,
            c_track_entry: *mut spTrackEntry,
            c_event: *mut spEvent,
        ) {
            let user_data =
                unsafe { &mut *((*c_animation_state).userData as *mut AnimationStateUserData) };
            if let Some(listener) = &user_data.listener {
                let animation_state = unsafe { AnimationState::new_from_ptr(c_animation_state) };
                let track_entry = unsafe { TrackEntry::new_from_ptr(c_track_entry) };
                let event = if !c_event.is_null() {
                    Some(unsafe { Event::new_from_ptr(c_event) })
                } else {
                    None
                };
                listener(
                    &animation_state,
                    EventType::from(c_event_type),
                    &track_entry,
                    event.as_ref(),
                );
            }
        }
        let user_data =
            unsafe { &mut *((*self.c_animation_state.0).userData as *mut AnimationStateUserData) };
        user_data.listener = Some(Box::new(listener));
        self.c_ptr_mut().listener = Some(c_listener);
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
    c_accessor!(tracks_count, set_tracks_count, tracksCount, i32);
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
    c_accessor!(timescale, set_timescale, timeScale, f32);
    c_accessor!(unkeyed_state, set_unkeyed_state, unkeyedState, i32);
    c_accessor_renderer_object!();

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
                drop(Box::from_raw(
                    (*self.c_animation_state.0).userData as *mut AnimationStateUserData,
                ));
            }
            unsafe {
                spAnimationState_dispose(self.c_animation_state.0);
            }
        }
    }
}

#[derive(Default)]
struct AnimationStateUserData {
    listener: Option<Box<dyn Fn(&AnimationState, EventType, &TrackEntry, Option<&Event>)>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Start = 0,
    Interrupt = 1,
    End = 2,
    Complete = 3,
    Dispose = 4,
    Event = 5,
    Unknown = 99,
}

impl From<spEventType> for EventType {
    fn from(event_type: spEventType) -> Self {
        match event_type {
            0 => Self::Start,
            1 => Self::Interrupt,
            2 => Self::End,
            3 => Self::Complete,
            4 => Self::Dispose,
            5 => Self::Event,
            _ => Self::Unknown,
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
    c_accessor!(track_index, set_track_index, trackIndex, i32);
    c_accessor_bool!(looping, set_looping, loop_0);
    c_accessor_bool!(hold_previous, set_hold_previous, holdPrevious);
    c_accessor_bool!(reverse, set_reverse, reverse);
    c_accessor_bool!(shortest_rotation, set_shortest_rotation, shortestRotation);
    c_accessor!(event_threshold, set_event_threshold, eventThreshold, f32);
    c_accessor!(
        attachment_threshold,
        set_attachment_threshold,
        attachmentThreshold,
        f32
    );
    c_accessor!(
        draw_order_threshold,
        set_draw_order_threshold,
        drawOrderThreshold,
        f32
    );
    c_accessor!(delay, set_delay, delay, f32);
    c_accessor!(track_time, set_track_time, trackTime, f32);
    c_accessor!(track_last, set_track_last, trackLast, f32);
    c_accessor!(next_track_last, set_next_track_last, nextTrackLast, f32);
    c_accessor!(track_end, set_track_end, trackEnd, f32);
    c_accessor!(timescale, set_timescale, timeScale, f32);
    c_accessor!(alpha, set_alpha, alpha, f32);
    c_accessor!(mix_time, set_mix_time, mixTime, f32);
    c_accessor!(mix_duration, set_mix_duration, mixDuration, f32);
    c_accessor!(interrupt_alpha, set_interrupt_alpha, interruptAlpha, f32);
    c_accessor!(total_alpha, set_total_alpha, totalAlpha, f32);

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
