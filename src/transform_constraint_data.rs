use crate::{
    c::{spBoneData, spTransformConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    BoneData,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#TransformConstraintData)
#[derive(Debug)]
pub struct TransformConstraintData {
    c_transform_constraint_data: SyncPtr<spTransformConstraintData>,
}

impl NewFromPtr<spTransformConstraintData> for TransformConstraintData {
    unsafe fn new_from_ptr(c_slot: *mut spTransformConstraintData) -> Self {
        Self {
            c_transform_constraint_data: SyncPtr(c_slot),
        }
    }
}

impl TransformConstraintData {
    c_accessor_string!(name, name);
    c_accessor!(order, order, i32);
    c_accessor_bool!(skin_required, skinRequired);

    c_accessor_bool!(local, local);
    c_accessor!(mix_rotate, mixRotate, f32);
    c_accessor!(mix_scale_x, mixScaleX, f32);
    c_accessor!(mix_scale_y, mixScaleY, f32);
    c_accessor!(mix_shear_y, mixShearY, f32);
    c_accessor!(mix_x, mixX, f32);
    c_accessor!(mix_y, mixY, f32);
    c_accessor!(offset_rotation, offsetRotation, f32);
    c_accessor!(offset_scale_x, offsetScaleX, f32);
    c_accessor!(offset_scale_y, offsetScaleY, f32);
    c_accessor!(offset_shear_y, offsetShearY, f32);
    c_accessor!(offset_x, offsetX, f32);
    c_accessor!(offset_y, offsetY, f32);
    c_accessor_bool!(relative, relative);

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        bones,
        bone_at_index,
        TransformConstraintData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr!(target, target, BoneData, spBoneData);

    c_ptr!(c_transform_constraint_data, spTransformConstraintData);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl TransformConstraintData {
    #[must_use]
    pub fn mix_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.mix_scale_x(),
            y: self.mix_scale_y(),
        }
    }

    #[must_use]
    pub fn mix(&self) -> Vector2<f32> {
        Vector2 {
            x: self.mix_x(),
            y: self.mix_y(),
        }
    }

    #[must_use]
    pub fn offset_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.offset_scale_x(),
            y: self.offset_scale_y(),
        }
    }

    #[must_use]
    pub fn offset(&self) -> Vector2<f32> {
        Vector2 {
            x: self.offset_x(),
            y: self.offset_y(),
        }
    }
}
