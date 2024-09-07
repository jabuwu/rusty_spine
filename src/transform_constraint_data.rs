use crate::{
    c::{spBoneData, spTransformConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    BoneData,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// Stores the setup pose for a [`TransformConstraint`](`crate::TransformConstraint`).
///
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
    c_accessor_string!(
        /// The constraint's name, which is unique across all constraints in the skeleton of the
        /// same type.
        name,
        name
    );
    c_accessor!(
        /// The ordinal of this constraint for the order a skeleton's constraints will be applied by
        /// [`Skeleton::update_world_transform`](`crate::Skeleton::update_world_transform`).
        order,
        order,
        i32
    );
    c_accessor_bool!(
        /// When true,
        /// [`Skeleton::update_world_transform`](`crate::Skeleton::update_world_transform`) only
        /// updates this constraint if the skin contains this constraint.
        skin_required,
        skinRequired
    );

    c_accessor_bool!(local, local);
    c_accessor!(rotate_mix, rotateMix, f32);
    c_accessor!(translate_mix, translateMix, f32);
    c_accessor!(scale_mix, scaleMix, f32);
    c_accessor!(shear_mix, shearMix, f32);
    c_accessor!(offset_rotation, offsetRotation, f32);
    c_accessor!(offset_x, offsetX, f32);
    c_accessor!(offset_y, offsetY, f32);
    c_accessor!(offset_scale_x, offsetScaleX, f32);
    c_accessor!(offset_scale_y, offsetScaleY, f32);
    c_accessor!(offset_shear_y, offsetShearY, f32);
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
    c_accessor_tmp_ptr!(
        /// The target bone whose world transform will be copied to the constrained bones.
        target,
        target,
        BoneData,
        spBoneData
    );

    c_ptr!(c_transform_constraint_data, spTransformConstraintData);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl TransformConstraintData {
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
