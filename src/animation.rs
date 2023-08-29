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
    c_accessor_string!(
        /// The animation's name, which is unique across all animations in the skeleton.
        name,
        name
    );
    c_accessor!(
        /// The duration of the animation in seconds, which is usually the highest time of all
        /// frames in the timeline. The duration is used to know when it has completed and when it
        /// should loop back to the start.
        duration,
        duration,
        f32
    );
    c_ptr!(c_animation, spAnimation);
    // TODO: timeline accessors
}
