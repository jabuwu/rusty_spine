use crate::{
    c::spAnimation,
    c_interface::{NewFromPtr, SyncPtr},
};

#[derive(Debug)]
pub struct Animation {
    c_animation: SyncPtr<spAnimation>,
}

impl NewFromPtr<spAnimation> for Animation {
    unsafe fn new_from_ptr(c_animation: *const spAnimation) -> Self {
        Self {
            c_animation: SyncPtr(c_animation as *mut spAnimation),
        }
    }
}

impl Animation {
    c_accessor_string!(name, name);
    c_accessor!(duration, duration, f32);
    c_ptr!(c_animation, spAnimation);
    // TODO: timeline accessors
}
