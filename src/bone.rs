use std::ffi::CStr;

use crate::{
    c::{spBone, spBoneData},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct Bone {
    c_bone: SyncPtr<spBone>,
    data: BoneData,
}

impl Bone {
    pub(crate) fn new(c_bone: *mut spBone) -> Self {
        let c_bone_data = unsafe { (*c_bone).data };
        Self {
            c_bone: SyncPtr(c_bone),
            data: BoneData::new(c_bone_data),
        }
    }

    pub fn data(&self) -> &BoneData {
        &self.data
    }

    c_ptr!(c_bone, spBone);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(rotation, rotation_mut, rotation, f32);
    c_accessor!(scale_x, scale_x_mut, scaleX, f32);
    c_accessor!(scale_y, scale_y_mut, scaleY, f32);
    c_accessor!(shear_x, shear_x_mut, shearX, f32);
    c_accessor!(shear_y, shear_y_mut, shearY, f32);
    c_accessor!(ax, ax_mut, ax, f32);
    c_accessor!(ay, ay_mut, ay, f32);
    c_accessor!(arotation, arotation_mut, arotation, f32);
    c_accessor!(ascale_x, ascale_x_mut, ascaleX, f32);
    c_accessor!(ascale_y, ascale_y_mut, ascaleY, f32);
    c_accessor!(ashear_x, ashear_x_mut, ashearX, f32);
    c_accessor!(ashear_y, ashear_y_mut, ashearY, f32);
    c_accessor!(a, a_mut, a, f32);
    c_accessor!(b, b_mut, b, f32);
    c_accessor!(c, c_mut, c, f32);
    c_accessor!(d, d_mut, d, f32);
    c_accessor!(world_x, world_x_mut, worldX, f32);
    c_accessor!(world_y, world_y_mut, worldY, f32);
    c_accessor_bool!(sorted, set_sorted, sorted);
    c_accessor_bool!(active, set_active, active);
}

#[derive(Debug)]
pub struct BoneData {
    c_bone_data: SyncPtr<spBoneData>,
}

impl BoneData {
    fn new(c_bone_data: *mut spBoneData) -> Self {
        Self {
            c_bone_data: SyncPtr(c_bone_data),
        }
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.c_ptr_ref().name).to_str().unwrap() }
    }

    c_ptr!(c_bone_data, spBoneData);
}
