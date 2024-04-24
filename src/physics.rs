use crate::c::spPhysics;

pub enum Physics {
    None = 0,
    Reset = 1,
    Update = 2,
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
