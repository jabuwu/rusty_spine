use crate::{
    c::{spBone, spIkConstraint, spIkConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    Bone, IkConstraintData,
};

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#IkConstraint)
#[derive(Debug)]
pub struct IkConstraint {
    c_ik_constraint: SyncPtr<spIkConstraint>,
}

impl NewFromPtr<spIkConstraint> for IkConstraint {
    unsafe fn new_from_ptr(c_slot: *mut spIkConstraint) -> Self {
        Self {
            c_ik_constraint: SyncPtr(c_slot),
        }
    }
}

impl IkConstraint {
    c_accessor_tmp_ptr_mut!(
        /// The IK constraint's setup pose data.
        data,
        data_mut,
        data,
        IkConstraintData,
        spIkConstraintData
    );

    c_accessor_bool!(active, active);
    c_accessor_mut!(
        /// For two bone IK, controls the bend direction of the IK bones, either 1 or -1.
        bend_direction,
        set_bend_direction,
        bendDirection,
        i32
    );
    c_accessor_bool_mut!(
        /// For one bone IK, when true and the target is too close, the bone is scaled to reach it.
        compress,
        set_compress,
        compress
    );
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// rotation.
        ///
        /// For two bone IK: if the parent bone has local nonuniform scale, the child bone's local Y
        /// translation is set to 0.
        mix,
        set_mix,
        mix,
        f32
    );
    c_accessor_mut!(
        /// For two bone IK, the target bone's distance from the maximum reach of the bones where
        /// rotation begins to slow. The bones will not straighten completely until the target is
        /// this far out of range.
        softness,
        set_softness,
        softness,
        f32
    );
    c_accessor_bool_mut!(
        /// When true and the target is out of range, the parent bone is scaled to reach it.
        ///
        /// For two bone IK: 1) the child bone's local Y translation is set to 0, 2) stretch is not
        /// applied if [`softness`](`Self::softness`) is > 0, and 3) if the parent bone has local
        /// nonuniform scale, stretch is not applied.
        stretch,
        set_stretch,
        stretch
    );

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        /// The bones that will be modified by this IK constraint.
        bones,
        bone_at_index,
        IkConstraint,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr_mut!(
        /// The bone that is the IK target.
        target,
        target_mut,
        target,
        Bone,
        spBone
    );

    c_ptr!(c_ik_constraint, spIkConstraint);
}
