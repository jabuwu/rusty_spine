use crate::{
    c::spAnimation,
    c_interface::{NewFromPtr, SyncPtr},
};

/// Stores timelines for animating a skeleton.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Animation)
#[derive(Debug)]
pub struct Animation {
    c_animation: SyncPtr<spAnimation>,
}

impl NewFromPtr<spAnimation> for Animation {
    unsafe fn new_from_ptr(c_animation: *mut spAnimation) -> Self {
        Self {
            c_animation: SyncPtr(c_animation),
        }
    }
}

impl Animation {
    c_accessor_string!(name, name);
    c_accessor!(duration, duration, f32);
    c_ptr!(c_animation, spAnimation);
    // TODO: timeline accessors
}
