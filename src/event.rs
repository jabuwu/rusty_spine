use crate::{
    c::{spEvent, spEventData},
    c_interface::{NewFromPtr, SyncPtr},
    TrackEntry,
};

#[allow(unused_imports)]
use crate::AnimationState;

/// A wrapper for [`Event`] that makes events slightly nicer to work with in Rust.
///
/// To receive events, see [`AnimationState::set_listener`].
pub enum AnimationEvent<'a> {
    Start {
        /// The track this event originated from.
        track_entry: TrackEntry,
    },
    Interrupt {
        /// The track this event originated from.
        track_entry: TrackEntry,
    },
    End {
        /// The track this event originated from.
        track_entry: TrackEntry,
    },
    Complete {
        /// The track this event originated from.
        track_entry: TrackEntry,
    },
    Dispose {
        /// The track this event originated from.
        track_entry: TrackEntry,
    },
    Event {
        /// The track this event originated from.
        track_entry: TrackEntry,
        /// The name of the event, which is unique across all events in the skeleton.
        name: &'a str,
        /// The animation time this event was keyed.
        time: f32,
        /// The event's int value.
        int: i32,
        /// The event's float value.
        float: f32,
        /// The event's string value or an empty string.
        string: &'a str,
        /// The event's audio path or an empty string.
        audio_path: &'a str,
        /// The event's audio volume.
        volume: f32,
        /// The event's audio balance.
        balance: f32,
        /// The raw event data.
        event: Event,
    },
}

/// Events fired from animations.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Event)
///
/// To receive events, see [`AnimationState::set_listener`].
#[derive(Debug)]
pub struct Event {
    c_event: SyncPtr<spEvent>,
}

impl NewFromPtr<spEvent> for Event {
    unsafe fn new_from_ptr(c_event: *mut spEvent) -> Self {
        Self {
            c_event: SyncPtr(c_event),
        }
    }
}

impl Event {
    c_accessor_tmp_ptr_mut!(
        /// The events's setup pose data.
        data,
        /// The events's mutable setup pose data.
        data_mut,
        data,
        EventData,
        spEventData
    );
    c_accessor!(
        /// The animation time this event was keyed.
        time,
        time,
        f32
    );
    c_accessor!(
        /// The event's int value.
        int_value,
        intValue,
        i32
    );
    c_accessor!(
        /// The event's float value.
        float_value,
        floatValue,
        f32
    );
    c_accessor_string!(
        /// The event's string value or an empty string.
        string_value,
        stringValue
    );
    c_accessor!(
        /// The event's audio volume value.
        volume,
        volume,
        f32
    );
    c_accessor!(
        /// The event's audio balance value.
        balance,
        balance,
        f32
    );
    c_ptr!(c_event, spEvent);
}

/// Static event data imported from Spine.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#EventData)
#[derive(Debug)]
pub struct EventData {
    c_event_data: SyncPtr<spEventData>,
}

impl NewFromPtr<spEventData> for EventData {
    unsafe fn new_from_ptr(c_event_data: *mut spEventData) -> Self {
        Self {
            c_event_data: SyncPtr(c_event_data),
        }
    }
}

impl EventData {
    c_accessor_string!(
        ///The name of the event, which is unique across all events in the skeleton.
        name,
        name
    );
    c_accessor!(
        /// The event's int value.
        int_value,
        intValue,
        i32
    );
    c_accessor!(
        /// The event's float value.
        float_value,
        floatValue,
        f32
    );
    c_accessor_string!(
        /// The event's string value or an empty string.
        string_value,
        stringValue
    );
    c_accessor_string!(
        /// The event's audio path.
        audio_path,
        audioPath
    );
    c_accessor!(
        /// The event's audio volume value.
        volume,
        volume,
        f32
    );
    c_accessor!(
        /// The event's audio balance value.
        balance,
        balance,
        f32
    );
    c_ptr!(c_event_data, spEventData);
}
