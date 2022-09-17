use crate::{
    c::{spBone, spBoneData, spTransformMode},
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct Bone {
    c_bone: SyncPtr<spBone>,
}

impl NewFromPtr<spBone> for Bone {
    unsafe fn new_from_ptr(c_bone: *const spBone) -> Self {
        Self {
            c_bone: SyncPtr(c_bone as *mut spBone),
        }
    }
}

impl Bone {
    c_ptr!(c_bone, spBone);
    c_accessor!(x, set_x, x, f32);
    c_accessor!(y, set_y, y, f32);
    c_accessor!(rotation, rotation_mut, rotation, f32);
    c_accessor!(scale_x, set_scale_x, scaleX, f32);
    c_accessor!(scale_y, set_scale_y, scaleY, f32);
    c_accessor!(shear_x, set_shear_x, shearX, f32);
    c_accessor!(shear_y, set_shear_y, shearY, f32);
    c_accessor!(ax, set_ax, ax, f32);
    c_accessor!(ay, set_ay, ay, f32);
    c_accessor!(arotation, set_arotation, arotation, f32);
    c_accessor!(ascale_x, set_ascale_x, ascaleX, f32);
    c_accessor!(ascale_y, set_ascale_y, ascaleY, f32);
    c_accessor!(ashear_x, set_ashear_x, ashearX, f32);
    c_accessor!(ashear_y, set_ashear_y, ashearY, f32);
    c_accessor!(a, set_a, a, f32);
    c_accessor!(b, set_b, b, f32);
    c_accessor!(c, set_c, c, f32);
    c_accessor!(d, set_d, d, f32);
    c_accessor!(world_x, set_world_x, worldX, f32);
    c_accessor!(world_y, set_world_y, worldY, f32);
    c_accessor_bool!(sorted, set_sorted, sorted);
    c_accessor_bool!(active, set_active, active);
    c_accessor_tmp_ptr!(data, data_mut, data, BoneData, spBoneData);
}

#[derive(Debug)]
pub struct BoneData {
    c_bone_data: SyncPtr<spBoneData>,
}

impl NewFromPtr<spBoneData> for BoneData {
    unsafe fn new_from_ptr(c_bone_data: *const spBoneData) -> Self {
        Self {
            c_bone_data: SyncPtr(c_bone_data as *mut spBoneData),
        }
    }
}

impl BoneData {
    c_ptr!(c_bone_data, spBoneData);
    c_accessor_string!(name, name);
    c_accessor!(index, set_index, index, i32);
    c_accessor!(length, set_length, length, f32);
    c_accessor!(x, set_x, x, f32);
    c_accessor!(y, set_y, y, f32);
    c_accessor!(rotation, set_rotation, rotation, f32);
    c_accessor!(scale_x, set_scale_x, scaleX, f32);
    c_accessor!(scale_y, set_scale_y, scaleY, f32);
    c_accessor!(shear_x, set_shear_x, shearX, f32);
    c_accessor!(shear_y, set_shear_y, shearY, f32);
    c_accessor_color!(color, color_mut, color);
    c_accessor_bool!(skin_required, set_skin_required, skinRequired);
    c_accessor_enum!(
        transform_mode,
        set_transform_mode,
        transformMode,
        TransformMode
    );
    c_accessor_tmp_ptr_optional!(parent, parent_mut, parent, BoneData, spBoneData);
}

pub enum TransformMode {
    Normal = 0,
    OnlyTranslation = 1,
    NoRotationOrReflection = 2,
    NoScale = 3,
    NoScaleOrReflection = 4,
    Unknown = 99,
}

impl From<spTransformMode> for TransformMode {
    fn from(mode: spTransformMode) -> Self {
        match mode {
            0 => Self::Normal,
            1 => Self::OnlyTranslation,
            2 => Self::NoRotationOrReflection,
            3 => Self::NoScale,
            4 => Self::NoScaleOrReflection,
            _ => Self::Unknown,
        }
    }
}
