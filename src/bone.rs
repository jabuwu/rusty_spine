use std::ffi::CStr;

use crate::{c::spBone, sync_ptr::SyncPtr};

#[derive(Debug)]
pub struct Bone {
    c_bone: SyncPtr<spBone>,
}

impl Bone {
    pub(crate) fn new(c_bone: *mut spBone) -> Self {
        Self {
            c_bone: SyncPtr(c_bone),
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*(*self.c_bone.0).data).name)
                .to_str()
                .unwrap()
        }
    }

    pub fn world_x(&self) -> f32 {
        unsafe { (*self.c_bone.0).worldX }
    }

    pub fn world_y(&self) -> f32 {
        unsafe { (*self.c_bone.0).worldY }
    }

    pub fn c_ptr(&self) -> *mut spBone {
        self.c_bone.0
    }
}
