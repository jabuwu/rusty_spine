use crate::{
    c::{
        spBoneData, spPathConstraintData, spPositionMode, spRotateMode, spSlotData, spSpacingMode,
    },
    c_interface::{NewFromPtr, SyncPtr},
    BoneData, SlotData,
};

#[cfg(feature = "mint")]
use mint::Vector2;

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
    c_accessor_string!(name, name);
    c_accessor!(order, order, i32);
    c_accessor_bool!(skin_required, skinRequired);

    c_accessor_enum!(position_mode, positionMode, PositionMode);
    c_accessor_enum!(spacing_mode, spacingMode, SpacingMode);
    c_accessor_enum!(rotate_mode, rotateMode, RotateMode);

    c_accessor!(offset_rotation, offsetRotation, f32);
    c_accessor!(position, position, f32);
    c_accessor!(spacing, spacing, f32);
    c_accessor!(mix_rotate, mixRotate, f32);
    c_accessor!(mix_x, mixX, f32);
    c_accessor!(mix_y, mixY, f32);

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        bones,
        bone_at_index,
        PathConstraintData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr!(target, target, SlotData, spSlotData);

    c_ptr!(c_path_constraint_data, spPathConstraintData);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl PathConstraintData {
    #[must_use]
    pub fn mix(&self) -> Vector2<f32> {
        Vector2 {
            x: self.mix_x(),
            y: self.mix_y(),
        }
    }
}

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
