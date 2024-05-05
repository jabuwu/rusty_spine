use crate::{
    c::{
        spBone, spTransformConstraint, spTransformConstraintData,
        spTransformConstraint_setToSetupPose, spTransformConstraint_update,
    },
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
    /// Applies the constraint to the constrained bones.
    pub fn update(&self) {
        unsafe {
            spTransformConstraint_update(self.c_transform_constraint.0);
        }
    }

    pub fn set_to_setup_pose(&self) {
        unsafe {
            spTransformConstraint_setToSetupPose(self.c_transform_constraint.0);
        }
    }

    c_accessor_tmp_ptr_mut!(
        /// The transform constraint's setup pose data.
        data,
        data_mut,
        data,
        TransformConstraintData,
        spTransformConstraintData
    );

    c_accessor_bool!(active, active);
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// rotation.
        mix_rotate,
        set_mix_rotate,
        mixRotate,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// scale X.
        mix_scale_x,
        set_scale_x,
        mixScaleX,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// scale X.
        mix_scale_y,
        set_scale_y,
        mixScaleY,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// shear Y.
        mix_shear_y,
        set_shear_y,
        mixShearY,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// translation X.
        mix_x,
        set_mix_x,
        mixX,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// translation Y.
        mix_y,
        set_mix_y,
        mixY,
        f32
    );

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
