use super::{
    c::{
        spBone, spTransformConstraint, spTransformConstraintData,
        spTransformConstraint_setToSetupPose, spTransformConstraint_update,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Bone, TransformConstraintData,
};

#[cfg(feature = "mint")]
use mint::Vector2;

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
        set_mix_scale_x,
        mixScaleX,
        f32
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// scale X.
        mix_scale_y,
        set_mix_scale_y,
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

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl TransformConstraint {
    #[must_use]
    pub fn mix_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.mix_scale_x(),
            y: self.mix_scale_y(),
        }
    }

    pub fn set_mix_scale(&mut self, mix_scale: impl Into<Vector2<f32>>) {
        let mix_scale: Vector2<f32> = mix_scale.into();
        self.set_mix_scale_x(mix_scale.x);
        self.set_mix_scale_y(mix_scale.y);
    }

    #[must_use]
    pub fn mix(&self) -> Vector2<f32> {
        Vector2 {
            x: self.mix_x(),
            y: self.mix_y(),
        }
    }

    pub fn set_mix(&mut self, mix: impl Into<Vector2<f32>>) {
        let mix: Vector2<f32> = mix.into();
        self.set_mix_x(mix.x);
        self.set_mix_y(mix.y);
    }
}
