use std::sync::Arc;

use crate::{
    atlas::Atlas,
    bone::BoneData,
    c::{spBoneData, spSkeletonData, spSkeletonData_dispose},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct SkeletonData {
    c_skeleton_data: SyncPtr<spSkeletonData>,
    // TODO: this atlas arc is kind of a hack
    // skeleton data should keep a reference to data it requires
    // but that will not be an atlas if a custom attachment loader is used
    _atlas: Option<Arc<Atlas>>,
}

impl SkeletonData {
    pub(crate) fn new(c_skeleton_data: *mut spSkeletonData, atlas: Option<Arc<Atlas>>) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
            _atlas: atlas,
        }
    }

    c_ptr!(c_skeleton_data, spSkeletonData);
    c_accessor_string!(version, version);
    c_accessor_string!(hash, hash);
    c_accessor_string!(images_path, imagesPath);
    c_accessor_string!(audio_path, audioPath);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);
    c_accessor!(bones_count, bones_count_mut, bonesCount, i32);
    c_accessor!(slots_count, slots_count_mut, slotsCount, i32);
    c_accessor!(skins_count, skins_count_mut, skinsCount, i32);
    c_accessor!(events_count, events_count_mut, eventsCount, i32);
    c_accessor!(animations_count, animations_count_mut, animationsCount, i32);
    c_accessor!(
        ik_constraints_count,
        ik_constraints_count_mut,
        ikConstraintsCount,
        i32
    );
    c_accessor!(
        transform_constraints_count,
        transform_constraints_count_mut,
        transformConstraintsCount,
        i32
    );
    c_accessor!(
        path_constraints_count,
        path_constraints_count_mut,
        pathConstraintsCount,
        i32
    );
    c_accessor_array!(
        bones,
        bones_mut,
        bone_at_index,
        bone_at_index_mut,
        SkeletonData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );

    // TODO: accessors and methods for the arrays in spSkeletonData
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.c_skeleton_data.0);
        }
    }
}
