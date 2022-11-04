use crate::{
    c::{spEvent, spEventData},
    c_interface::{NewFromPtr, SyncPtr},
};

#[allow(unused_imports)]
use crate::AnimationState;

/// Events fired from animations.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Event)
///
/// To receive events, set a listener on [`AnimationState`].
/// ```
/// # #[path="./doctests.rs"]
/// # mod doctests;
/// # use rusty_spine::{AnimationState, EventType};
/// # let (_, mut animation_state) = doctests::test_spineboy_instance();
/// animation_state.set_listener(|animation_state, event_type, track_entry, event| {
///     match event_type {
///         EventType::Start => {
///             println!("Animation started!");
///         }
///         EventType::Interrupt => {
///             println!("Animation interrupted!");
///         }
///         EventType::End => {
///             println!("Animation ended!");
///         }
///         EventType::Complete => {
///             println!("Animation completed!");
///         }
///         EventType::Dispose => {
///             println!("Animation disposed!");
///         }
///         EventType::Event => {
///             println!("Animation event!");
///             if let Some(event) = event {
///                 println!("  Event name: {}", event.data().name());
///             }
///         }
///         _ => {}
///     }
/// });
/// ```
#[derive(Debug)]
pub struct Event {
    c_event: SyncPtr<spEvent>,
}

impl NewFromPtr<spEvent> for Event {
    unsafe fn new_from_ptr(c_event: *const spEvent) -> Self {
        Self {
            c_event: SyncPtr(c_event as *mut spEvent),
        }
    }
}

impl Event {
    c_accessor_tmp_ptr!(data, data_mut, data, EventData, spEventData);
    c_accessor!(time, time, f32);
    c_accessor!(int_value, intValue, i32);
    c_accessor!(float_value, floatValue, f32);
    c_accessor_string!(string_value, stringValue);
    c_accessor!(volume, volume, f32);
    c_accessor!(balance, balance, f32);
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
    unsafe fn new_from_ptr(c_event_data: *const spEventData) -> Self {
        Self {
            c_event_data: SyncPtr(c_event_data as *mut spEventData),
        }
    }
}

impl EventData {
    c_accessor_string!(name, name);
    c_accessor!(int_value, intValue, i32);
    c_accessor!(float_value, floatValue, f32);
    c_accessor_string!(string_value, stringValue);
    c_accessor_string!(audio_path, audioPath);
    c_accessor!(volume, volume, f32);
    c_accessor!(balance, balance, f32);
    c_ptr!(c_event_data, spEventData);
}
