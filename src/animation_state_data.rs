use std::{ffi::CString, sync::Arc};

use crate::{
    animation::Animation,
    c::{
        c_void, spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose,
        spAnimationStateData_getMix, spAnimationStateData_setMix,
        spAnimationStateData_setMixByName, spSkeletonData,
    },
    c_interface::{NewFromPtr, SyncPtr},
    skeleton_data::SkeletonData,
};

#[derive(Debug)]
pub struct AnimationStateData {
    c_animation_state_data: SyncPtr<spAnimationStateData>,
    owns_memory: bool,
    _skeleton_data: Option<Arc<SkeletonData>>,
}

impl NewFromPtr<spAnimationStateData> for AnimationStateData {
    unsafe fn new_from_ptr(c_animation_state_data: *const spAnimationStateData) -> Self {
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data as *mut spAnimationStateData),
            owns_memory: false,
            _skeleton_data: None,
        }
    }
}

impl AnimationStateData {
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_animation_state_data = unsafe { spAnimationStateData_create(skeleton_data.c_ptr()) };
        Self {
            c_animation_state_data: SyncPtr(c_animation_state_data),
            owns_memory: true,
            _skeleton_data: Some(skeleton_data),
        }
    }

    pub fn set_mix_by_name(&mut self, from_name: &str, to_name: &str, duration: f32) {
        let c_from_name = CString::new(from_name).unwrap();
        let c_to_name = CString::new(to_name).unwrap();
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

    c_accessor_tmp_ptr!(
        skeleton_data,
        skeleton_data_mut,
        skeletonData,
        SkeletonData,
        spSkeletonData
    );
    c_accessor!(default_mix, defaultMix, f32);
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
