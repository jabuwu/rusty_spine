use crate::{
    c::{
        spBone, spIkConstraint, spIkConstraintData, spIkConstraint_setToSetupPose,
        spIkConstraint_update,
    },
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
    pub fn update(&self) {
        unsafe {
            spIkConstraint_update(self.c_ik_constraint.0);
        }
    }

    pub fn set_to_setup_pose(&self) {
        unsafe {
            spIkConstraint_setToSetupPose(self.c_ik_constraint.0);
        }
    }

    c_accessor_tmp_ptr_mut!(data, data_mut, data, IkConstraintData, spIkConstraintData);

    c_accessor_bool!(active, active);
    c_accessor_mut!(bend_direction, set_bend_direction, bendDirection, i32);
    c_accessor_bool_mut!(compress, set_compress, compress);
    c_accessor_mut!(mix, set_mix, mix, f32);
    c_accessor_mut!(softness, set_softness, softness, f32);
    c_accessor_bool_mut!(stretch, set_stretch, stretch);

    c_accessor!(bones_count, bonesCount, usize);
    c_accessor_array!(
        bones,
        bone_at_index,
        IkConstraint,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_tmp_ptr_mut!(target, target_mut, target, Bone, spBone);

    c_ptr!(c_ik_constraint, spIkConstraint);
}
