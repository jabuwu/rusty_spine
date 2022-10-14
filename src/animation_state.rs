use std::{ffi::CString, sync::Arc};

use crate::{
    animation::Animation,
    animation_state_data::AnimationStateData,
    c::{
        c_void, spAnimation, spAnimationState, spAnimationStateData, spAnimationState_addAnimation,
        spAnimationState_addAnimationByName, spAnimationState_addEmptyAnimation,
        spAnimationState_apply, spAnimationState_clearListenerNotifications,
        spAnimationState_clearTrack, spAnimationState_clearTracks,
        spAnimationState_create, spAnimationState_dispose, spAnimationState_disposeStatics,
        spAnimationState_getCurrent, spAnimationState_setAnimation,
        spAnimationState_setAnimationByName, spAnimationState_setEmptyAnimation,
        spAnimationState_setEmptyAnimations, spAnimationState_update, spEvent, spEventType,
        spTrackEntry, spTrackEntry_getAnimationTime,
    },
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    error::Error,
    event::Event,
    skeleton::Skeleton,
};

#[cfg(not(feature="spine38"))]
use crate::{
    c::{
        spAnimationState_clearNext, spTrackEntry_getTrackComplete
    }
};

/// The live animation state for a skeleton, allowing animation layering and crossfading.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#AnimationState)
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
            spAnimationState_update(self.c_ptr(), delta);
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
    ) -> CTmpMut<Self, TrackEntry> {
        let c_animation_name = CString::new(animation_name).unwrap();
        CTmpMut::new(
            self,
            TrackEntry::new_from_ptr(spAnimationState_setAnimationByName(
                self.c_ptr(),
                track_index,
                c_animation_name.as_ptr(),
                looping as i32,
            )),
        )
    }

    pub fn set_animation_by_name(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
    ) -> Result<CTmpMut<Self, TrackEntry>, Error> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .find(|animation| animation.name() == animation_name)
            .is_some()
        {
            Ok(unsafe {
                self.set_animation_by_name_unchecked(track_index, animation_name, looping)
            })
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn set_animation(
        &mut self,
        track_index: i32,
        animation: &Animation,
        looping: bool,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_setAnimation(
                    self.c_ptr(),
                    track_index,
                    animation.c_ptr(),
                    looping as i32,
                )),
            )
        }
    }

    pub unsafe fn add_animation_by_name_unchecked(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        let c_animation_name = CString::new(animation_name).unwrap();
        CTmpMut::new(
            self,
            TrackEntry::new_from_ptr(spAnimationState_addAnimationByName(
                self.c_ptr(),
                track_index,
                c_animation_name.as_ptr(),
                looping as i32,
                delay,
            )),
        )
    }

    pub fn add_animation_by_name(
        &mut self,
        track_index: i32,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) -> Result<CTmpMut<Self, TrackEntry>, Error> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .find(|animation| animation.name() == animation_name)
            .is_some()
        {
            Ok(unsafe {
                self.add_animation_by_name_unchecked(track_index, animation_name, looping, delay)
            })
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
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_addAnimation(
                    self.c_ptr(),
                    track_index,
                    animation.c_ptr(),
                    looping as i32,
                    delay,
                )),
            )
        }
    }

    pub fn set_empty_animation(
        &mut self,
        track_index: i32,
        mix_duration: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_setEmptyAnimation(
                    self.c_ptr(),
                    track_index,
                    mix_duration,
                )),
            )
        }
    }

    pub fn add_empty_animation(
        &mut self,
        track_index: i32,
        mix_duration: f32,
        delay: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_addEmptyAnimation(
                    self.c_ptr(),
                    track_index,
                    mix_duration,
                    delay,
                )),
            )
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
                Some(CTmpRef::new(self, TrackEntry::new_from_ptr(ptr)))
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
        unsafe {
            self.c_ptr_mut().listener = Some(c_listener);
        }
    }

    pub fn clear_listener_notifications(&mut self) {
        unsafe {
            spAnimationState_clearListenerNotifications(self.c_ptr());
        }
    }

    #[cfg(not(feature="spine38"))]
    pub fn clear_next(&mut self, entry: &TrackEntry) {
        unsafe {
            spAnimationState_clearNext(self.c_ptr(), entry.c_ptr());
        }
    }

    c_accessor_tmp_ptr!(
        data,
        data_mut,
        data,
        AnimationStateData,
        spAnimationStateData
    );
    c_accessor!(tracks_count, tracksCount, i32);
    c_accessor_array_nullable!(
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
    c_accessor_mut!(timescale, set_timescale, timeScale, f32);
    c_accessor_renderer_object!();

    pub fn dispose_statics() {
        unsafe {
            spAnimationState_disposeStatics();
        }
    }
    c_ptr!(c_animation_state, spAnimationState);
}

impl Drop for AnimationState {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                (*self.c_animation_state.0).listener = None;
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
        TrackEntry {
            c_track_entry: SyncPtr(c_track_entry as *mut spTrackEntry),
        }
    }
}

impl TrackEntry {
    pub fn animation_time(&self) -> f32 {
        unsafe { spTrackEntry_getAnimationTime(self.c_ptr()) }
    }

    #[cfg(not(feature="spine38"))]
    pub fn track_complete(&self) -> f32 {
        unsafe { spTrackEntry_getTrackComplete(self.c_ptr()) }
    }

    fn handle_valid(handle: &TrackEntryHandle) -> bool {
        let track_count = unsafe { (*handle.c_parent.0).tracksCount };
        if handle.index < track_count {
            let track_at_index =
                unsafe { *(*handle.c_parent.0).tracks.offset(handle.index as isize) };
            if track_at_index == handle.c_item.0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    c_accessor_tmp_ptr!(animation, animation_mut, animation, Animation, spAnimation);
    #[cfg(not(feature="spine38"))]
    c_accessor_tmp_ptr!(previous, previous_mut, previous, TrackEntry, spTrackEntry);
    c_accessor_tmp_ptr!(next, next_mut, next, TrackEntry, spTrackEntry);
    c_accessor_tmp_ptr!(
        mixing_from,
        mixing_from_mut,
        mixingFrom,
        TrackEntry,
        spTrackEntry
    );
    c_accessor_tmp_ptr!(mixing_to, mixing_to_mut, mixingTo, TrackEntry, spTrackEntry);
    c_accessor!(track_index, trackIndex, i32);
    c_accessor_bool_mut!(looping, set_looping, loop_0);
    c_accessor_bool_mut!(hold_previous, set_hold_previous, holdPrevious);
    #[cfg(not(feature="spine38"))]
    c_accessor_bool_mut!(reverse, set_reverse, reverse);
    #[cfg(not(feature="spine38"))]
    c_accessor_bool_mut!(shortest_rotation, set_shortest_rotation, shortestRotation);
    c_accessor_mut!(event_threshold, set_event_threshold, eventThreshold, f32);
    c_accessor_mut!(
        attachment_threshold,
        set_attachment_threshold,
        attachmentThreshold,
        f32
    );
    c_accessor_mut!(
        draw_order_threshold,
        set_draw_order_threshold,
        drawOrderThreshold,
        f32
    );
    c_accessor_mut!(animation_start, set_animation_start, animationStart, f32);
    c_accessor_mut!(animation_end, set_animation_end, animationEnd, f32);
    c_accessor_mut!(animation_last, set_animation_last, animationLast, f32);
    c_accessor_mut!(delay, set_delay, delay, f32);
    c_accessor_mut!(track_time, set_track_time, trackTime, f32);
    c_accessor_mut!(track_end, set_track_end, trackEnd, f32);
    c_accessor_mut!(timescale, set_timescale, timeScale, f32);
    c_accessor_mut!(alpha, set_alpha, alpha, f32);
    c_accessor_mut!(mix_time, set_mix_time, mixTime, f32);
    c_accessor_mut!(mix_duration, set_mix_duration, mixDuration, f32);
    c_accessor!(total_alpha, totalAlpha, f32);
    c_accessor_renderer_object!();
    c_ptr!(c_track_entry, spTrackEntry);
}

c_handle_indexed_decl!(
    /// A storeable reference to a [TrackEntry](struct.TrackEntry.html).
    ///
    /// Can be acquired from a
    /// [CTmpRef<AnimationState, TrackEntry>](c_interface/struct.CTmpRef.html) or
    /// [CTmpMut<AnimationState, TrackEntry>](c_interface/struct.CTmpMut.html) acquired from an
    /// [AnimationState](struct.AnimationState.html) instance.
    ///
    /// ```
    /// # #[path="./doctests.rs"]
    /// # mod doctests;
    /// # use rusty_spine::{AnimationState, EventType, TrackEntryHandle};
    /// # let (_, animation_state) = doctests::test_spineboy_instance();
    /// let track_entry_handles: Vec<TrackEntryHandle> = animation_state.tracks().map(|track| track.unwrap().handle()).collect();
    /// for track_entry_handle in track_entry_handles.iter() {
    ///     let track_entry = track_entry_handle.get(&animation_state).unwrap();
    ///     println!("{}", track_entry.animation().name());
    /// }
    /// ```
    TrackEntryHandle,
    TrackEntry,
    AnimationState,
    spTrackEntry,
    spAnimationState
);

impl<'a> CTmpRef<'a, AnimationState, TrackEntry> {
    pub fn handle(&self) -> TrackEntryHandle {
        TrackEntryHandle::new(self.track_index(), self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpMut<'a, AnimationState, TrackEntry> {
    pub fn handle(&self) -> TrackEntryHandle {
        TrackEntryHandle::new(self.track_index(), self.c_ptr(), self.parent.c_ptr())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_spineboy_instance;

    #[test]
    fn track_entry_optional() {
        let (_, mut animation_state) = test_spineboy_instance();
        let _ = animation_state.set_animation_by_name(0, "idle", true);
        let _ = animation_state.set_animation_by_name(2, "run", true);

        let track_0 = animation_state.tracks().nth(0).unwrap();
        let track_1 = animation_state.tracks().nth(1).unwrap();
        let track_2 = animation_state.tracks().nth(2).unwrap();
        assert!(track_0.is_some());
        assert!(track_1.is_none());
        assert!(track_2.is_some());
        assert_eq!(track_0.unwrap().animation().name(), "idle");
        assert_eq!(track_2.unwrap().animation().name(), "run");
        assert!(animation_state.tracks().nth(3).is_none());

        assert!(animation_state.track_at_index(0).is_some());
        assert!(animation_state.track_at_index(1).is_none());
        assert!(animation_state.track_at_index(2).is_some());
    }

    #[test]
    fn track_entry_invalidate_clear() {
        let (_, mut animation_state) = test_spineboy_instance();
        let _ = animation_state.set_animation_by_name(0, "idle", true);

        let track_handle = animation_state.track_at_index(0).unwrap().handle();

        assert!(track_handle.get(&animation_state).is_some());
        animation_state.clear_track(0);
        assert!(track_handle.get(&animation_state).is_none());
    }

    #[test]
    fn track_entry_invalidate_change() {
        let (_, mut animation_state) = test_spineboy_instance();
        let _ = animation_state.set_animation_by_name(0, "idle", true);

        let track_handle = animation_state.track_at_index(0).unwrap().handle();

        assert!(track_handle.get(&animation_state).is_some());
        let _ = animation_state.set_animation_by_name(0, "run", true);
        assert!(track_handle.get(&animation_state).is_none());
    }
}
