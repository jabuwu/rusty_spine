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
    c_accessor!(index, index_mut, index, i32);
    c_accessor!(length, length_mut, length, f32);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(rotation, rotation_mut, rotation, f32);
    c_accessor!(scale_x, scale_x_mut, scaleX, f32);
    c_accessor!(scale_y, scale_y_mut, scaleY, f32);
    c_accessor!(shear_x, shear_x_mut, shearX, f32);
    c_accessor!(shear_y, shear_y_mut, shearY, f32);
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
