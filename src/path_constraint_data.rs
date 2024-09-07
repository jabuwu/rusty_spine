use crate::{
    c::{
        spBoneData, spPathConstraintData, spPositionMode, spRotateMode, spSlotData, spSpacingMode,
    },
    c_interface::{NewFromPtr, SyncPtr},
    BoneData, SlotData,
};

/// Stores the setup pose for a [`PathConstraint`](`crate::PathConstraint`).
///
/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#PathConstraintData)
#[derive(Debug)]
pub struct PathConstraintData {
    c_path_constraint_data: SyncPtr<spPathConstraintData>,
}

impl NewFromPtr<spPathConstraintData> for PathConstraintData {
    unsafe fn new_from_ptr(c_slot: *mut spPathConstraintData) -> Self {
        Self {
            c_path_constraint_data: SyncPtr(c_slot),
        }
    }
}

impl PathConstraintData {
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

    c_accessor_enum!(
        /// The mode for positioning the first bone on the path.
        position_mode,
        positionMode,
        PositionMode
    );
    c_accessor_enum!(
        /// The mode for positioning the bones after the first bone on the path.
        spacing_mode,
        spacingMode,
        SpacingMode
    );
    c_accessor_enum!(
        /// The mode for adjusting the rotation of the bones.
        rotate_mode,
        rotateMode,
        RotateMode
    );

    c_accessor!(
        /// An offset added to the constrained bone rotation.
        offset_rotation,
        offsetRotation,
        f32
    );
    c_accessor!(
        /// The position along the path.
        position,
        position,
        f32
    );
    c_accessor!(
        /// The spacing between bones.
        spacing,
        spacing,
        f32
    );

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        /// The bones that will be modified by this path constraint.
        bones,
        bone_at_index,
        PathConstraintData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr!(
        /// The slot whose path attachment will be used to constrained the bones.
        target,
        target,
        SlotData,
        spSlotData
    );

    c_ptr!(c_path_constraint_data, spPathConstraintData);
}

/// Controls how the first bone is positioned along the path.
///
/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#PositionMode)
pub enum PositionMode {
    Fixed = 0,
    Percent = 1,
    Unknown = 99,
}

impl From<spPositionMode> for PositionMode {
    fn from(mode: spPositionMode) -> Self {
        match mode {
            0 => Self::Fixed,
            1 => Self::Percent,
            _ => Self::Unknown,
        }
    }
}

/// Controls how the first bone is positioned along the path.
///
/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#SpacingMode)
pub enum SpacingMode {
    Length = 0,
    Fixed = 1,
    Percent = 2,
    Proportional = 3,
    Unknown = 99,
}

impl From<spSpacingMode> for SpacingMode {
    fn from(mode: spSpacingMode) -> Self {
        match mode {
            0 => Self::Length,
            1 => Self::Fixed,
            2 => Self::Percent,
            3 => Self::Proportional,
            _ => Self::Unknown,
        }
    }
}

/// Controls how bones are rotated, translated, and scaled to match the path.
///
/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#RotateMode)
pub enum RotateMode {
    Tangent = 0,
    Chain = 1,
    ChainScale = 2,
    Unknown = 99,
}

impl From<spRotateMode> for RotateMode {
    fn from(mode: spRotateMode) -> Self {
        match mode {
            0 => Self::Tangent,
            1 => Self::Chain,
            2 => Self::ChainScale,
            _ => Self::Unknown,
        }
    }
}
