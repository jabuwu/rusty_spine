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
    tmp_ref::{TmpRef, TmpRefMut},
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

    pub fn bones(&self) -> SkeletonBoneIterator {
        SkeletonBoneIterator {
            _skeleton: self,
            items: self.c_ptr_ref().bones,
            index: 0,
            count: self.bones_count() as usize,
        }
    }

    pub fn bones_mut(&mut self) -> SkeletonBoneMutIterator {
        SkeletonBoneMutIterator {
            _skeleton: self,
            items: self.c_ptr_mut().bones,
            index: 0,
            count: self.bones_count() as usize,
        }
    }

    pub fn bone_at_index(&self, index: usize) -> Option<TmpRef<Self, Bone>> {
        if index < self.bones_count() as usize {
            Some(TmpRef::new(
                self,
                Bone::new_from_ptr(unsafe { *self.c_ptr_ref().bones.offset(index as isize) }),
            ))
        } else {
            None
        }
    }

    pub fn bone_at_index_mut(&mut self, index: usize) -> Option<TmpRefMut<Self, Bone>> {
        if index < self.bones_count() as usize {
            Some(TmpRefMut::new(
                self,
                Bone::new_from_ptr(unsafe { *self.c_ptr_mut().bones.offset(index as isize) }),
            ))
        } else {
            None
        }
    }

    pub fn slots(&self) -> SkeletonSlotIterator {
        SkeletonSlotIterator {
            _skeleton: self,
            items: self.c_ptr_ref().slots,
            index: 0,
            count: self.slots_count() as usize,
        }
    }

    pub fn slots_mut(&mut self) -> SkeletonSlotMutIterator {
        SkeletonSlotMutIterator {
            _skeleton: self,
            items: self.c_ptr_mut().slots,
            index: 0,
            count: self.slots_count() as usize,
        }
    }

    pub fn slot_at_index(&self, index: usize) -> Option<TmpRef<Self, Slot>> {
        if index < self.slots_count() as usize {
            Some(TmpRef::new(
                self,
                Slot::new_from_ptr(unsafe { *self.c_ptr_ref().slots.offset(index as isize) }),
            ))
        } else {
            None
        }
    }

    pub fn slot_at_index_mut(&mut self, index: usize) -> Option<TmpRefMut<Self, Slot>> {
        if index < self.slots_count() as usize {
            Some(TmpRefMut::new(
                self,
                Slot::new_from_ptr(unsafe { *self.c_ptr_mut().slots.offset(index as isize) }),
            ))
        } else {
            None
        }
    }

    pub fn draw_order(&self) -> SkeletonSlotIterator {
        SkeletonSlotIterator {
            _skeleton: self,
            items: self.c_ptr_ref().drawOrder,
            index: 0,
            count: self.slots_count() as usize,
        }
    }

    pub fn draw_order_mut(&mut self) -> SkeletonSlotMutIterator {
        SkeletonSlotMutIterator {
            _skeleton: self,
            items: self.c_ptr_mut().drawOrder,
            index: 0,
            count: self.slots_count() as usize,
        }
    }

    pub fn draw_order_at_index(&self, index: usize) -> Option<TmpRef<Self, Slot>> {
        if index < self.slots_count() as usize {
            Some(TmpRef::new(
                self,
                Slot::new_from_ptr(unsafe { *self.c_ptr_ref().drawOrder.offset(index as isize) }),
            ))
        } else {
            None
        }
    }

    pub fn draw_order_at_index_mut(&self, index: usize) -> Option<TmpRefMut<Self, Slot>> {
        if index < self.slots_count() as usize {
            Some(TmpRefMut::new(
                self,
                Slot::new_from_ptr(unsafe { *self.c_ptr_mut().drawOrder.offset(index as isize) }),
            ))
        } else {
            None
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
}

macro_rules! iterator {
    ($name:ident, $name_mut:ident, $type:ident, $c_type:ident) => {
        pub struct $name<'a> {
            _skeleton: &'a Skeleton,
            items: *mut *mut $c_type,
            index: usize,
            count: usize,
        }

        impl<'a> Iterator for $name<'a> {
            type Item = TmpRef<'a, Skeleton, $type>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.count {
                    let item = $type::new_from_ptr(unsafe { *self.items.offset(1) });
                    self.index += 1;
                    Some(TmpRef::new(self._skeleton, item))
                } else {
                    None
                }
            }
        }

        pub struct $name_mut<'a> {
            _skeleton: &'a Skeleton,
            items: *mut *mut $c_type,
            index: usize,
            count: usize,
        }

        impl<'a> Iterator for $name_mut<'a> {
            type Item = TmpRefMut<'a, Skeleton, $type>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.count {
                    let item = $type::new_from_ptr(unsafe { *self.items.offset(1) });
                    self.index += 1;
                    Some(TmpRefMut::new(self._skeleton, item))
                } else {
                    None
                }
            }
        }
    };
}

iterator!(SkeletonBoneIterator, SkeletonBoneMutIterator, Bone, spBone);
iterator!(SkeletonSlotIterator, SkeletonSlotMutIterator, Slot, spSlot);
