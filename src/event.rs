use crate::{
    c::{spEvent, spEventData},
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
};

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
    c_ptr!(c_event, spEvent);
    c_accessor_tmp_ptr!(data, data_mut, data, EventData, spEventData);
    c_accessor!(time, set_time, time, f32);
    c_accessor!(int_value, set_int_value, intValue, i32);
    c_accessor!(float_value, set_float_value, floatValue, f32);
    c_accessor_string!(string_value, stringValue);
    c_accessor!(volume, set_volume, volume, f32);
    c_accessor!(balance, set_balance, balance, f32);
}

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
    c_ptr!(c_event_data, spEventData);
    c_accessor_string!(name, name);
    c_accessor!(int_value, set_int_value, intValue, i32);
    c_accessor!(float_value, set_float_value, floatValue, f32);
    c_accessor_string!(string_value, stringValue);
    c_accessor_string!(audio_path, audioPath);
    c_accessor!(volume, set_volume, volume, f32);
    c_accessor!(balance, set_balance, balance, f32);
}
