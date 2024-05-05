use crate::{
    c::{spBoneData, spIkConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    BoneData,
};

/// Stores the setup pose for an [`IkConstraint`](`crate::IkConstraint`).
///
/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#IkConstraintData)
#[derive(Debug)]
pub struct IkConstraintData {
    c_ik_constraint_data: SyncPtr<spIkConstraintData>,
}

impl NewFromPtr<spIkConstraintData> for IkConstraintData {
    unsafe fn new_from_ptr(c_slot: *mut spIkConstraintData) -> Self {
        Self {
            c_ik_constraint_data: SyncPtr(c_slot),
        }
    }
}

impl IkConstraintData {
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

    c_accessor!(
        /// For two bone IK, controls the bend direction of the IK bones, either 1 or -1.
        bend_direction,
        bendDirection,
        i32
    );
    c_accessor_bool!(
        /// For one bone IK, when true and the target is too close, the bone is scaled to reach it.
        compress,
        compress
    );
    c_accessor!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained
        /// rotation.
        ///
        /// For two bone IK: if the parent bone has local nonuniform scale, the child bone's local Y
        /// translation is set to 0.
        mix,
        mix,
        f32
    );
    c_accessor!(
        /// For two bone IK, the target bone's distance from the maximum reach of the bones where
        /// rotation begins to slow. The bones will not straighten completely until the target is this
        /// far out of range.
        softness,
        softness,
        f32
    );
    c_accessor_bool!(
        /// When true and the target is out of range, the parent bone is scaled to reach it.
        ///
        /// For two bone IK: 1) the child bone's local Y translation is set to 0, 2) stretch is not
        /// applied if [`softness`](`Self::softness`) is > 0, and 3) if the parent bone has local
        /// nonuniform scale, stretch is not applied.
        stretch,
        stretch
    );
    c_accessor_bool!(
        /// When true and [`compress`](`Self::compress`) or [`stretch`](`Self::stretch`) is used,
        /// the bone is scaled on both the X and Y axes.
        uniform,
        uniform
    );

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        bones,
        bone_at_index,
        IkConstraintData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr!(target, target, BoneData, spBoneData);

    c_ptr!(c_ik_constraint_data, spIkConstraintData);
}
