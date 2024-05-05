use crate::{
    c::{
        spBone, spPathConstraint, spPathConstraintData, spPathConstraint_setToSetupPose,
        spPathConstraint_update, spSlot,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Bone, PathConstraintData, Slot,
};

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#PathConstraint)
#[derive(Debug)]
pub struct PathConstraint {
    c_path_constraint: SyncPtr<spPathConstraint>,
}

impl NewFromPtr<spPathConstraint> for PathConstraint {
    unsafe fn new_from_ptr(c_slot: *mut spPathConstraint) -> Self {
        Self {
            c_path_constraint: SyncPtr(c_slot),
        }
    }
}

impl PathConstraint {
    /// Applies the constraint to the constrained bones.
    pub fn update(&self) {
        unsafe {
            spPathConstraint_update(self.c_path_constraint.0);
        }
    }

    pub fn set_to_setup_pose(&self) {
        unsafe {
            spPathConstraint_setToSetupPose(self.c_path_constraint.0);
        }
    }

    c_accessor_tmp_ptr_mut!(
        /// The path constraint's setup pose data.
        data,
        data_mut,
        data,
        PathConstraintData,
        spPathConstraintData
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
    c_accessor_mut!(
        /// The position along the path.
        position,
        set_position,
        position,
        f32
    );
    c_accessor_mut!(
        /// The spacing between bones.
        spacing,
        set_spacing,
        spacing,
        f32
    );

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        /// The bones that will be modified by this path constraint.
        bones,
        bone_at_index,
        PathConstraint,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr_mut!(
        /// The slot whose path attachment will be used to constrained the bones.
        target,
        target_mut,
        target,
        Slot,
        spSlot
    );

    c_ptr!(c_path_constraint, spPathConstraint);
}
