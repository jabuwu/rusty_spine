use std::sync::Arc;

use crate::{
    animation::Animation,
    animation_state_data::AnimationStateData,
    c::{
        c_void, spAnimation, spAnimationState, spAnimationStateData, spAnimationState_addAnimation,
        spAnimationState_addAnimationByName, spAnimationState_addEmptyAnimation,
        spAnimationState_apply, spAnimationState_clearListenerNotifications,
        spAnimationState_clearTrack, spAnimationState_clearTracks, spAnimationState_create,
        spAnimationState_dispose, spAnimationState_disposeStatics, spAnimationState_getCurrent,
        spAnimationState_setAnimation, spAnimationState_setAnimationByName,
        spAnimationState_setEmptyAnimation, spAnimationState_setEmptyAnimations,
        spAnimationState_update, spEvent, spEventType, spTrackEntry, spTrackEntry_getAnimationTime,
    },
    c_interface::{to_c_str, CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    error::SpineError,
    event::Event,
    skeleton::Skeleton,
    AnimationEvent,
};

/// Applies animations over time, queues animations for later playback, mixes (crossfading) between
/// animations, and applies multiple animations on top of each other (layering).
///
/// See [Applying Animations](http://esotericsoftware.com/spine-applying-animations/) in the Spine
/// Runtimes Guide.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#AnimationState)
#[derive(Debug)]
pub struct AnimationState {
    c_animation_state: SyncPtr<spAnimationState>,
    owns_memory: bool,
    _animation_state_data: Option<Arc<AnimationStateData>>,
}

impl NewFromPtr<spAnimationState> for AnimationState {
    unsafe fn new_from_ptr(c_animation_state: *mut spAnimationState) -> Self {
        Self {
            c_animation_state: SyncPtr(c_animation_state),
            owns_memory: false,
            _animation_state_data: None,
        }
    }
}

impl AnimationState {
    #[must_use]
    pub fn new(animation_state_data: Arc<AnimationStateData>) -> Self {
        let c_animation_state = unsafe { spAnimationState_create(animation_state_data.c_ptr()) };
        unsafe {
            (*c_animation_state).userData =
                (Box::leak(Box::default()) as *mut AnimationStateUserData).cast::<c_void>();
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

    /// Clears all animations in all track entries in this animation state.
    pub fn clear_tracks(&mut self) {
        unsafe {
            spAnimationState_clearTracks(self.c_ptr());
        }
    }

    /// Clears animations for the given track entry index in this animation state.
    pub fn clear_track(&mut self, track_index: usize) {
        unsafe {
            spAnimationState_clearTrack(self.c_ptr(), track_index as i32);
        }
    }

    /// Sets the animation for the given track by name, clearing any queued tracks, and returning
    /// the track index. If the track index doesn't exist then it will be created.
    ///
    /// # Safety
    ///
    /// This function should only be called with valid animation names. It is faster than the safe
    /// alternative, [`AnimationState::set_animation_by_name`], but will likely segfault if the
    /// animation does not exist.
    pub unsafe fn set_animation_by_name_unchecked(
        &mut self,
        track_index: usize,
        animation_name: &str,
        looping: bool,
    ) -> CTmpMut<Self, TrackEntry> {
        let c_animation_name = to_c_str(animation_name);
        CTmpMut::new(
            self,
            TrackEntry::new_from_ptr(spAnimationState_setAnimationByName(
                self.c_ptr(),
                track_index as i32,
                c_animation_name.as_ptr(),
                i32::from(looping),
            )),
        )
    }

    /// Sets the animation for the given track by name, clearing any queued tracks, and returning
    /// the track index. If the track index doesn't exist then it will be created.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::NotFound`] if an animation doesn't exist with the given name.
    pub fn set_animation_by_name(
        &mut self,
        track_index: usize,
        animation_name: &str,
        looping: bool,
    ) -> Result<CTmpMut<Self, TrackEntry>, SpineError> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .any(|animation| animation.name() == animation_name)
        {
            Ok(unsafe {
                self.set_animation_by_name_unchecked(track_index, animation_name, looping)
            })
        } else {
            Err(SpineError::new_not_found("Animation", animation_name))
        }
    }

    /// Sets the animation for the given track, clearning any queued tracks, and returning the
    /// track index. If the track index doesn't exist then it will be created.
    pub fn set_animation(
        &mut self,
        track_index: usize,
        animation: &Animation,
        looping: bool,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_setAnimation(
                    self.c_ptr(),
                    track_index as i32,
                    animation.c_ptr(),
                    i32::from(looping),
                )),
            )
        }
    }

    /// Queues the animation in the given track by name, returning the track index. If the track
    /// index doesn't exist then it will be created.
    ///
    /// # Safety
    ///
    /// This function should only be called with valid animation names. It is faster than the safe
    /// alternative, [`AnimationState::add_animation_by_name`], but will likely segfault if the
    /// animation does not exist.
    pub unsafe fn add_animation_by_name_unchecked(
        &mut self,
        track_index: usize,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        let c_animation_name = to_c_str(animation_name);
        CTmpMut::new(
            self,
            TrackEntry::new_from_ptr(spAnimationState_addAnimationByName(
                self.c_ptr(),
                track_index as i32,
                c_animation_name.as_ptr(),
                i32::from(looping),
                delay,
            )),
        )
    }

    /// Queues the animation in the given track by name, returning the track index. If the track
    /// index doesn't exist then it will be created.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::NotFound`] if an animation doesn't exist with the given name.
    pub fn add_animation_by_name(
        &mut self,
        track_index: usize,
        animation_name: &str,
        looping: bool,
        delay: f32,
    ) -> Result<CTmpMut<Self, TrackEntry>, SpineError> {
        if self
            .data()
            .skeleton_data()
            .animations()
            .any(|animation| animation.name() == animation_name)
        {
            Ok(unsafe {
                self.add_animation_by_name_unchecked(track_index, animation_name, looping, delay)
            })
        } else {
            Err(SpineError::new_not_found("Animation", animation_name))
        }
    }

    /// Queues the animation in the given track, returning the track index. If the track index
    /// doesn't exist then it will be created.
    pub fn add_animation(
        &mut self,
        track_index: usize,
        animation: &Animation,
        looping: bool,
        delay: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_addAnimation(
                    self.c_ptr(),
                    track_index as i32,
                    animation.c_ptr(),
                    i32::from(looping),
                    delay,
                )),
            )
        }
    }

    pub fn set_empty_animation(
        &mut self,
        track_index: usize,
        mix_duration: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_setEmptyAnimation(
                    self.c_ptr(),
                    track_index as i32,
                    mix_duration,
                )),
            )
        }
    }

    pub fn add_empty_animation(
        &mut self,
        track_index: usize,
        mix_duration: f32,
        delay: f32,
    ) -> CTmpMut<Self, TrackEntry> {
        unsafe {
            CTmpMut::new(
                self,
                TrackEntry::new_from_ptr(spAnimationState_addEmptyAnimation(
                    self.c_ptr(),
                    track_index as i32,
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

    #[must_use]
    pub fn get_current(&self, track_index: usize) -> Option<CTmpRef<Self, TrackEntry>> {
        unsafe {
            let ptr = spAnimationState_getCurrent(self.c_ptr(), track_index as i32);
            if !ptr.is_null() {
                Some(CTmpRef::new(self, TrackEntry::new_from_ptr(ptr)))
            } else {
                None
            }
        }
    }

    /// Set the event listener on this animation state. An animation state can only have one event
    /// listener at a time.
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, AnimationEvent};
    /// # let (_, mut animation_state) = test::TestAsset::spineboy().instance(true);
    /// animation_state.set_listener(|_, animation_event| match animation_event {
    ///     AnimationEvent::Start { track_entry } => {
    ///         println!("Animation {} started!", track_entry.track_index());
    ///     }
    ///     AnimationEvent::Interrupt { track_entry } => {
    ///         println!("Animation {} interrupted!", track_entry.track_index());
    ///     }
    ///     AnimationEvent::End { track_entry } => {
    ///         println!("Animation {} ended!", track_entry.track_index());
    ///     }
    ///     AnimationEvent::Complete { track_entry } => {
    ///         println!("Animation {} completed!", track_entry.track_index());
    ///     }
    ///     AnimationEvent::Dispose { track_entry } => {
    ///         println!("Animation {} disposed!", track_entry.track_index());
    ///     }
    ///     AnimationEvent::Event {
    ///         track_entry,
    ///         name,
    ///         int,
    ///         float,
    ///         string,
    ///         audio_path,
    ///         volume,
    ///         balance,
    ///         ..
    ///     } => {
    ///         println!("Animation {} event!", track_entry.track_index());
    ///         println!("  Name: {name}");
    ///         println!("  Integer: {int}");
    ///         println!("  Float: {float}");
    ///         if !string.is_empty() {
    ///             println!("  String: \"{string}\"");
    ///         }
    ///         if !audio_path.is_empty() {
    ///             println!("  Audio: \"{audio_path}\"");
    ///             println!("    Volume: {volume}");
    ///             println!("    Balance: {balance}");
    ///         }
    ///     }
    /// });
    /// ```
    pub fn set_listener<F>(&mut self, listener: F)
    where
        F: Fn(&AnimationState, AnimationEvent) + 'static,
    {
        extern "C" fn c_listener(
            c_animation_state: *mut spAnimationState,
            c_event_type: spEventType,
            c_track_entry: *mut spTrackEntry,
            c_event: *mut spEvent,
        ) {
            let user_data = unsafe {
                &mut *((*c_animation_state)
                    .userData
                    .cast::<AnimationStateUserData>())
            };
            if let Some(listener) = &user_data.listener {
                let animation_state = unsafe { AnimationState::new_from_ptr(c_animation_state) };
                let track_entry = unsafe { TrackEntry::new_from_ptr(c_track_entry) };
                let event_type = EventType::from(c_event_type);
                match event_type {
                    EventType::Start => {
                        listener(&animation_state, AnimationEvent::Start { track_entry });
                    }
                    EventType::Interrupt => {
                        listener(&animation_state, AnimationEvent::Interrupt { track_entry });
                    }
                    EventType::End => {
                        listener(&animation_state, AnimationEvent::End { track_entry });
                    }
                    EventType::Complete => {
                        listener(&animation_state, AnimationEvent::Complete { track_entry });
                    }
                    EventType::Dispose => {
                        listener(&animation_state, AnimationEvent::Dispose { track_entry });
                    }
                    EventType::Event => {
                        assert!(!c_event.is_null());
                        let event = unsafe { Event::new_from_ptr(c_event) };
                        let raw_event = unsafe { Event::new_from_ptr(c_event) };
                        listener(
                            &animation_state,
                            AnimationEvent::Event {
                                track_entry,
                                name: event.data().name(),
                                time: event.time(),
                                int: event.int_value(),
                                float: event.float_value(),
                                string: event.string_value(),
                                audio_path: event.data().audio_path(),
                                volume: event.volume(),
                                balance: event.balance(),
                                event: raw_event,
                            },
                        );
                    }
                    EventType::Unknown => {}
                };
            }
        }
        let user_data = unsafe {
            &mut *((*self.c_animation_state.0)
                .userData
                .cast::<AnimationStateUserData>())
        };
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

    c_accessor_tmp_ptr_mut!(
        /// The [`AnimationStateData`] to look up mix durations.
        data,
        data_mut,
        data,
        AnimationStateData,
        spAnimationStateData
    );
    c_accessor!(tracks_count, tracksCount, usize);
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
    c_accessor_mut!(
        /// Multiplier for the delta time when the animation state is updated, causing time for all
        /// animations and mixes to play slower or faster. Defaults to 1.
        ///
        /// See [`TrackEntry::timescale`] for affecting a single animation.
        timescale,
        /// Set the timescale, see [`timescale`](`Self::timescale`).
        set_timescale,
        timeScale,
        f32
    );
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
                    (*self.c_animation_state.0)
                        .userData
                        .cast::<AnimationStateUserData>(),
                ));
            }
            unsafe {
                spAnimationState_dispose(self.c_animation_state.0);
            }
        }
    }
}

type AnimationStateListenerCb = Box<dyn Fn(&AnimationState, AnimationEvent)>;

#[derive(Default)]
struct AnimationStateUserData {
    listener: Option<AnimationStateListenerCb>,
}

/// The variants of event types.
///
/// Usually not necessary to check, instead use the variants of [`AnimationEvent`].
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

/// Stores settings and other state for the playback of an animation on an [`AnimationState`] track.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#TrackEntry)
#[derive(Debug)]
pub struct TrackEntry {
    c_track_entry: SyncPtr<spTrackEntry>,
}

impl NewFromPtr<spTrackEntry> for TrackEntry {
    unsafe fn new_from_ptr(c_track_entry: *mut spTrackEntry) -> Self {
        TrackEntry {
            c_track_entry: SyncPtr(c_track_entry),
        }
    }
}

impl TrackEntry {
    /// Uses [`track_time`](`Self::track_time`) to compute the
    /// [`animation_time`](`Self::animation_time`). When the [`track_time`](`Self::track_time`) is
    /// 0, the [`animation_time`](`Self::animation_time`) is equal to the
    /// [`animation_start`](`Self::animation_start`) time.
    ///
    /// The [`animation_time`](`Self::animation_time`) is between
    /// [`animation_start`](`Self::animation_start`) and
    /// [`animation_end`](`Self::animation_end`), except if this track entry is non-looping and
    /// [`animation_end`](`Self::animation_end`) is >= to the animation duration, then
    /// [`animation_time`](`Self::animation_time`) continues to increase past
    /// [`animation_end`](`Self::animation_end`).
    #[must_use]
    pub fn animation_time(&self) -> f32 {
        unsafe { spTrackEntry_getAnimationTime(self.c_ptr()) }
    }

    fn handle_valid(handle: &TrackEntryHandle) -> bool {
        let track_count = unsafe { (*handle.c_parent.0).tracksCount };
        if handle.index < track_count {
            let track_at_index =
                unsafe { *(*handle.c_parent.0).tracks.offset(handle.index as isize) };
            track_at_index == handle.c_item.0
        } else {
            false
        }
    }

    c_accessor_tmp_ptr_mut!(
        /// The animation to apply for this track entry.
        animation,
        /// The mutable animation to apply for this track entry.
        animation_mut,
        animation,
        Animation,
        spAnimation
    );
    c_accessor_tmp_ptr_optional!(
        /// The animation queued to start after this animation, or [`None`] if there is none. `next`
        /// makes up a doubly linked list.
        ///
        /// See [`AnimationState::clear_next`] to truncate the list.
        next,
        next,
        TrackEntry,
        spTrackEntry
    );
    c_accessor_tmp_ptr_optional!(
        /// The track entry for the previous animation when mixing from the previous animation to this
        /// animation, or [`None`] if no mixing is currently occuring. When mixing from multiple
        /// animations, `mixing_from` makes up a linked list.
        mixing_from,
        mixingFrom,
        TrackEntry,
        spTrackEntry
    );
    c_accessor_tmp_ptr_optional!(
        /// The track entry for the next animation when mixing from this animation to the next
        /// animation, or [`None`] if no mixing is currently occuring. When mixing to multiple
        /// animations, `mixing_to` makes up a linked list.
        mixing_to,
        mixingTo,
        TrackEntry,
        spTrackEntry
    );
    c_accessor!(track_index, trackIndex, usize);
    c_accessor_bool_mut!(
        /// If `true`, the animation will repeat. If `false` it will not, instead its last frame is
        /// applied if played beyond its duration.
        looping,
        /// Set looping, see [`looping`](`Self::looping`).
        set_looping,
        loop_0
    );
    c_accessor_bool_mut!(
        /// If `true`, when mixing from the previous animation to this animation, the previous
        /// animation is applied as normal instead of being mixed out.
        ///
        /// When mixing between animations that key the same property, if a lower track also keys
        /// that property then the value will briefly dip toward the lower track value during the
        /// mix. This happens because the first animation mixes from 100% to 0% while the second
        /// animation mixes from 0% to 100%. Setting `hold_previous` to true applies the first
        /// animation at 100% during the mix so the lower track value is overwritten. Such dipping
        /// does not occur on the lowest track which keys the property, only when a higher track
        /// also keys the property.
        ///
        /// Snapping will occur if `hold_previous` is true and this animation does not key all the
        /// same properties as the previous animation.
        hold_previous,
        /// Set hold previous, see [`hold_previous`](`Self::hold_previous`).
        set_hold_previous,
        holdPrevious
    );
    c_accessor_mut!(
        /// When the mix percentage ([`mix_time`](`Self::mix_time`) /
        /// [`mix_duration`](`Self::mix_duration`)) is less than the `event_threshold`, event
        /// timelines are applied while this animation is being mixed out. Defaults to 0, so event
        /// timelines are not applied while this animation is being mixed out.
        event_threshold,
        /// Set the event threshold, see [`event_threshold`](`Self::event_threshold`).
        set_event_threshold,
        eventThreshold,
        f32
    );
    c_accessor_mut!(
        /// Seconds when this animation starts, both initially and after looping. Defaults to 0.
        ///
        /// When changing the animationStart time, it often makes sense to set
        /// [`animation_last`](`Self::animation_last`) to the same value to prevent timeline keys
        /// before the start time from triggering.
        animation_start,
        /// Set the animation start, see [`animation_start`](`Self::animation_start`).
        set_animation_start,
        animationStart,
        f32
    );
    c_accessor_mut!(
        /// Seconds for the last frame of this animation. Non-looping animations won't play past
        /// this time. Looping animations will loop back to
        /// [`animation_start`](`Self::animation_start`) at this time. Defaults to the animation
        /// duration.
        animation_end,
        /// Set the animation end, see [`animation_end`](`Self::animation_end`).
        set_animation_end,
        animationEnd,
        f32
    );
    c_accessor_mut!(
        /// The time in seconds this animation was last applied. Some timelines use this for
        /// one-time triggers. Eg, when this animation is applied, event timelines will fire all
        /// events between the animationLast time (exclusive) and animationTime (inclusive).
        /// Defaults to -1 to ensure triggers on frame 0 happen the first time this animation is
        /// applied.
        animation_last,
        /// Set the animation last, see [`animation_last`](`Self::animation_last`).
        set_animation_last,
        animationLast,
        f32
    );
    c_accessor_mut!(
        /// Seconds to postpone playing the animation. When this track entry is the current track
        /// entry, delay postpones incrementing the [`track_time`](`Self::track_time`). When this
        /// track entry is queued, delay is the time from the start of the previous animation to
        /// when this track entry will become the current track entry (ie when the previous track
        /// entry [`track_time`](`Self::track_time`) >= this track entry's delay).
        ///
        /// [`timescale`](`Self::timescale`) affects the delay.
        ///
        /// When using addAnimation with a delay <= 0, the delay is set using the mix duration from
        /// the AnimationStateData. If mixDuration is set afterward, the delay may need to be
        /// adjusted.
        delay,
        /// Set the delay, see [`delay`](`Self::delay`).
        set_delay,
        delay,
        f32
    );
    c_accessor_mut!(
        /// Current time in seconds this track entry has been the current track entry. The track
        /// time determines [`animation_time`](`Self::animation_time`). The track time can be set
        /// to start the animation at a time other than 0, without affecting looping.
        track_time,
        /// Set the track time, see [`track_time`](`Self::track_time`).
        set_track_time,
        trackTime,
        f32
    );
    c_accessor_mut!(
        /// The track time in seconds when this animation will be removed from the track. Defaults
        /// to the highest possible float value, meaning the animation will be applied until a new
        /// animation is set or the track is cleared. If the track end time is reached, no other
        /// animations are queued for playback, and mixing from any previous animations is complete,
        /// then the properties keyed by the animation are set to the setup pose and the track is
        /// cleared.
        ///
        /// It may be desired to use [`AnimationState::add_empty_animation`] rather than have the
        /// animation abruptly cease being applied.
        track_end,
        /// Set the track end, see [`track_end`](`Self::track_end`).
        set_track_end,
        trackEnd,
        f32
    );
    c_accessor_mut!(
        /// Multiplier for the delta time when this track entry is updated, causing time for this
        /// animation to pass slower or faster. Defaults to 1.
        ///
        /// Values < 0 are not supported. To play an animation in reverse, use
        /// [`set_reverse`](`Self::set_reverse`).
        ///
        /// [`mix_time`](`Self::mix_time`) is not affected by track entry time scale, so
        /// [`mix_duration`](`Self::mix_duration`) may need to be adjusted to match the animation
        /// speed.
        ///
        /// When using [`AnimationState::add_animation`] with a delay <= 0, the delay is set using
        /// the mix duration from the [`AnimationStateData`], assuming time scale to be 1. If the
        /// time scale is not 1, the delay may need to be adjusted.
        ///
        /// See [`AnimationState::timescale`] for affecting all animations.
        timescale,
        /// Set the timescale, see [`timescale`](`Self::timescale`).
        set_timescale,
        timeScale,
        f32
    );
    c_accessor_mut!(
        /// Values < 1 mix this animation with the skeleton's current pose (usually the pose
        /// resulting from lower tracks). Defaults to 1, which overwrites the skeleton's current
        /// pose with this animation.
        ///
        /// Typically track 0 is used to completely pose the skeleton, then alpha is used on higher
        /// tracks. It doesn't make sense to use alpha on track 0 if the skeleton pose is from the
        /// last frame render.
        alpha,
        /// Set the alpha value, see [`alpha`](`Self::alpha`).
        set_alpha,
        alpha,
        f32
    );
    c_accessor_mut!(
        /// Seconds from 0 to the [`mix_duration`](`Self::mix_duration`) when mixing from the
        /// previous animation to this animation. May be slightly more than
        /// [`mix_duration`](`Self::mix_duration`) when the mix is complete.
        mix_time,
        /// Set the mix time, see [`mix_time`](`Self::mix_time`).
        set_mix_time,
        mixTime,
        f32
    );
    c_accessor_mut!(
        /// Seconds for mixing from the previous animation to this animation. Defaults to the value
        /// provided by [`AnimationStateData::get_mix`] based on the animation before this animation
        /// (if any).
        ///
        /// A mix duration of 0 still mixes out over one frame to provide the track entry being
        /// mixed out a chance to revert the properties it was animating.
        ///
        /// The mixDuration can be set manually rather than use the value from
        /// [`AnimationStateData::get_mix`]. In that case, the
        /// [`mix_duration`](`Self::mix_duration`) can be set for a new track entry only before
        /// update is first called.
        ///
        /// When using [`AnimationState::add_animation`] with a `delay <= 0`, the
        /// [`delay`](`Self::delay`) is set using the mix duration from the [`AnimationStateData`].
        /// If `mix_duration` is set afterward, the delay may need to be adjusted.
        ///
        /// For example:
        /// ```
        /// # fn foo(entry: &mut rusty_spine::TrackEntry) {
        /// entry.set_delay(entry.previous().unwrap().track_complete() - entry.mix_duration());
        /// # }
        /// ```
        mix_duration,
        /// Set the mix duration, see [`mix_duration`](`Self::mix_duration`).
        set_mix_duration,
        mixDuration,
        f32
    );
    c_accessor!(total_alpha, totalAlpha, f32);
    c_accessor_renderer_object!();
    c_ptr!(c_track_entry, spTrackEntry);
}

c_handle_indexed_decl!(
    /// A storeable reference to a [`TrackEntry`].
    ///
    /// Can be acquired from a
    /// [`CTmpRef<AnimationState, TrackEntry>`] or [`CTmpMut<AnimationState, TrackEntry>`] acquired
    /// from an [`AnimationState`] instance.
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, EventType, TrackEntryHandle};
    /// # let (_, animation_state) = test::TestAsset::spineboy().instance(true);
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
    #[must_use]
    pub fn handle(&self) -> TrackEntryHandle {
        TrackEntryHandle::new(self.track_index() as i32, self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpMut<'a, AnimationState, TrackEntry> {
    #[must_use]
    pub fn handle(&self) -> TrackEntryHandle {
        TrackEntryHandle::new(self.track_index() as i32, self.c_ptr(), self.parent.c_ptr())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::TestAsset;

    #[test]
    fn track_entry_optional() {
        let (_, mut animation_state) = TestAsset::spineboy().instance(true);
        let _ = animation_state.set_animation_by_name(0, "idle", true);
        let _ = animation_state.set_animation_by_name(2, "run", true);

        let track_0 = animation_state.tracks().next().unwrap();
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
        let (_, mut animation_state) = TestAsset::spineboy().instance(true);
        let _ = animation_state.set_animation_by_name(0, "idle", true);

        let track_handle = animation_state.track_at_index(0).unwrap().handle();

        assert!(track_handle.get(&animation_state).is_some());
        animation_state.clear_track(0);
        assert!(track_handle.get(&animation_state).is_none());
    }

    #[test]
    fn track_entry_invalidate_change() {
        let (_, mut animation_state) = TestAsset::spineboy().instance(true);
        let _ = animation_state.set_animation_by_name(0, "idle", true);

        let track_handle = animation_state.track_at_index(0).unwrap().handle();

        assert!(track_handle.get(&animation_state).is_some());
        let _ = animation_state.set_animation_by_name(0, "run", true);
        assert!(track_handle.get(&animation_state).is_none());
    }
}
