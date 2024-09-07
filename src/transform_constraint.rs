use crate::{
    c::{spBone, spTransformConstraint, spTransformConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    Bone, TransformConstraintData,
};

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#TransformConstraint)
#[derive(Debug)]
pub struct TransformConstraint {
    c_transform_constraint: SyncPtr<spTransformConstraint>,
}

impl NewFromPtr<spTransformConstraint> for TransformConstraint {
    unsafe fn new_from_ptr(c_slot: *mut spTransformConstraint) -> Self {
        Self {
            c_transform_constraint: SyncPtr(c_slot),
        }
    }
}

impl TransformConstraint {
    c_accessor_tmp_ptr_mut!(
        /// The transform constraint's setup pose data.
        data,
        data_mut,
        data,
        TransformConstraintData,
        spTransformConstraintData
    );

    c_accessor_bool!(active, active);
    c_accessor_mut!(rotate_mix, set_rotate_mix, rotateMix, f32);
    c_accessor_mut!(translate_mix, set_translate_mix, translateMix, f32);
    c_accessor_mut!(scale_mix, set_scale_mix, scaleMix, f32);
    c_accessor_mut!(shear_mix, set_shear_mix, shearMix, f32);

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        /// The bones that will be modified by this transform constraint.
        bones,
        bone_at_index,
        TransformConstraint,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr_mut!(
        /// The target bone whose world transform will be copied to the constrained bones.
        target,
        target_mut,
        target,
        Bone,
        spBone
    );

    c_ptr!(c_transform_constraint, spTransformConstraint);
}
