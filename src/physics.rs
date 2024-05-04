use crate::c::spPhysics;

/// Determines how physics and other non-deterministic updates are applied.
pub enum Physics {
    /// Physics are not updated or applied.
    None = 0,
    /// Physics are reset to the current pose.
    Reset = 1,
    /// Physics are updated and the pose from physics is applied.
    Update = 2,
    /// Physics are not updated but the pose from physics is applied.
    Pose = 3,
    Unknown = 99,
}

impl From<spPhysics> for Physics {
    fn from(mode: spPhysics) -> Self {
        match mode {
            0 => Self::None,
            1 => Self::Reset,
            2 => Self::Update,
            3 => Self::Pose,
            _ => Self::Unknown,
        }
    }
}
