use std::sync::Arc;

use crate::{
    bone::Bone,
    c::{
        spBone, spSkeleton, spSkeleton_create, spSkeleton_setBonesToSetupPose,
        spSkeleton_setSlotsToSetupPose, spSkeleton_setToSetupPose, spSkeleton_updateCache,
        spSkeleton_updateWorldTransform, spSlot,
    },
    error::Error,
    skeleton_data::SkeletonData,
    slot::Slot,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct Skeleton {
    c_skeleton: SyncPtr<spSkeleton>,
    _skeleton_data: Arc<SkeletonData>,
}

impl Skeleton {
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Result<Self, Error> {
        let c_skeleton = unsafe { spSkeleton_create(skeleton_data.c_ptr()) };
        Ok(Self {
            c_skeleton: SyncPtr(c_skeleton),
            _skeleton_data: skeleton_data,
        })
    }

    pub fn update_cache(&mut self) {
        unsafe {
            spSkeleton_updateCache(self.c_ptr());
        }
    }

    pub fn update_world_transform(&mut self) {
        unsafe {
            spSkeleton_updateWorldTransform(self.c_ptr());
        }
    }

    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setToSetupPose(self.c_ptr());
        }
    }

    pub fn set_bones_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setBonesToSetupPose(self.c_ptr());
        }
    }

    pub fn set_slots_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setSlotsToSetupPose(self.c_ptr());
        }
    }

    // TODO: iterators for ik, transform, path constraints

    c_ptr!(c_skeleton, spSkeleton);
    c_accessor_color!(color, color_mut, color);
    c_accessor!(bones_count, bones_count_mut, bonesCount, i32);
    c_accessor!(slots_count, slots_count_mut, slotsCount, i32);
    c_accessor!(
        ik_contraints_count,
        ik_contraints_count_mut,
        ikConstraintsCount,
        i32
    );
    c_accessor!(
        transform_contraints_count,
        transform_contraints_count_mut,
        transformConstraintsCount,
        i32
    );
    c_accessor!(
        path_contraints_count,
        path_contraints_count_mut,
        pathConstraintsCount,
        i32
    );
    c_accessor!(scale_x, scale_x_mut, scaleX, f32);
    c_accessor!(scale_y, scale_y_mut, scaleY, f32);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor_array!(
        bones,
        bones_mut,
        bone_at_index,
        bone_at_index_mut,
        Skeleton,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_array!(
        slots,
        slots_mut,
        slot_at_index,
        slot_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        slots,
        slots_count
    );
    c_accessor_array!(
        draw_order,
        draw_order_mut,
        draw_order_at_index,
        draw_order_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        drawOrder,
        slots_count
    );
}
