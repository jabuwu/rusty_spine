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
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// rotation.
        mix_rotate,
        mixRotate,
        f32
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// scale X.
        mix_scale_x,
        mixScaleX, f32
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// scale Y.
        mix_scale_y,
        mixScaleY,
        f32
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// shear Y.
        mix_shear_y,
        mixShearY,
        f32
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// translation X.
        mix_x,
        mixX,
        f32
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// translation Y.
        mix_y,
        mixY,
        f32
    );
    c_accessor!(
        /// An offset added to the constrained bone rotation.
        offset_rotation,
        offsetRotation,
        f32
    );
    c_accessor!(
        /// An offset added to the constrained bone scale X.
        offset_scale_x,
        offsetScaleX,
        f32
    );
    c_accessor!(
        /// An offset added to the constrained bone scale Y.
        offset_scale_y,
        offsetScaleY,
        f32
    );
    c_accessor!(
        ///An offset added to the constrained bone shear Y.
        offset_shear_y,
        offsetShearY,
        f32
    );
    c_accessor!(
        /// An offset added to the constrained bone X translation.
        offset_x,
        offsetX,
        f32
    );
    c_accessor!(
        /// An offset added to the constrained bone Y translation.
        offset_y,
        offsetY,
        f32
    );
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
