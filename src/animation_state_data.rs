use std::sync::Arc;

use crate::{
    animation::Animation,
    c::{
        c_void, spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose,
        spAnimationStateData_getMix, spAnimationStateData_setMix,
        spAnimationStateData_setMixByName, spSkeletonData,
    },
    c_interface::{to_c_str, NewFromPtr, SyncPtr},
    skeleton_data::SkeletonData,
};

#[allow(unused_imports)]
use crate::AnimationState;

/// Animation settings used to instantiate [`AnimationState`].
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#AnimationStateData)
///
/// Mix durations can be applied to automatically blend between animations.  For example, to
/// smoothly mix between a `walk` and `run` animation for `0.2` seconds:
///
/// ```
/// # #[path="./test.rs"]
/// # mod test;
/// # let mut animation_state_data = test::TestAsset::spineboy().animation_state_data();
/// animation_state_data.set_mix_by_name("walk", "run", 0.2);
/// ```
///
/// This operation is one way, so to blend back and forth between the two animations, two mix
/// durations must be specified:
///
/// ```
/// # #[path="./test.rs"]
/// # mod test;
/// # let mut animation_state_data = test::TestAsset::spineboy().animation_state_data();
/// animation_state_data.set_mix_by_name("walk", "run", 0.2);
/// animation_state_data.set_mix_by_name("run", "walk", 0.2);
/// ```
#[derive(Debug)]
pub struct AnimationStateData {
    c_animation_state_data: SyncPtr<spAnimationStateData>,
    owns_memory: bool,
    _skeleton_data: Option<Arc<SkeletonData>>,
}

impl NewFromPtr<spAnimationStateData> for AnimationStateData {
    unsafe fn new_from_ptr(c_animation_state_data: *mut spAnimationStateData) -> Self {
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data),
            owns_memory: false,
            _skeleton_data: None,
        }
    }
}

impl AnimationStateData {
    #[must_use]
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_animation_state_data = unsafe { spAnimationStateData_create(skeleton_data.c_ptr()) };
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data),
            owns_memory: true,
            _skeleton_data: Some(skeleton_data),
        }
    }

    pub fn set_mix_by_name(&mut self, from_name: &str, to_name: &str, duration: f32) {
        let c_from_name = to_c_str(from_name);
        let c_to_name = to_c_str(to_name);
        unsafe {
            spAnimationStateData_setMixByName(
                self.c_ptr(),
                c_from_name.as_ptr(),
                c_to_name.as_ptr(),
                duration,
            );
        }
    }

    pub fn set_mix(&mut self, from: &Animation, to: &Animation, duration: f32) {
        unsafe {
            spAnimationStateData_setMix(self.c_ptr(), from.c_ptr(), to.c_ptr(), duration);
        }
    }

    pub fn get_mix(&mut self, from: &Animation, to: &Animation) -> f32 {
        unsafe { spAnimationStateData_getMix(self.c_ptr(), from.c_ptr(), to.c_ptr()) }
    }

    c_accessor_tmp_ptr_mut!(
        skeleton_data,
        skeleton_data_mut,
        skeletonData,
        SkeletonData,
        spSkeletonData
    );
    c_accessor_mut!(default_mix, set_default_mix, defaultMix, f32);
    c_accessor_passthrough!(entries, entries, *const c_void);
    c_ptr!(c_animation_state_data, spAnimationStateData);
}

impl Drop for AnimationStateData {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spAnimationStateData_dispose(self.c_animation_state_data.0);
            }
        }
    }
}
