use crate::{
    c::{spBoneData, spIkConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    BoneData,
};

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
    c_accessor_string!(name, name);
    c_accessor!(order, order, i32);
    c_accessor_bool!(skin_required, skinRequired);

    c_accessor!(bend_direction, bendDirection, i32);
    c_accessor_bool!(compress, compress);
    c_accessor!(mix, mix, f32);
    c_accessor!(softness, softness, f32);
    c_accessor_bool!(stretch, stretch);
    c_accessor_bool!(uniform, uniform);

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
