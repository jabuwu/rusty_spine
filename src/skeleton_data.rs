use std::sync::Arc;

use crate::{
    animation::Animation,
    bone::BoneData,
    c::{spAnimation, spBoneData, spSkeletonData, spSkeletonData_dispose, spSkin, spSlotData},
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    skin::Skin,
    slot::SlotData,
    Atlas,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// Static skeleton data imported from Spine.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#SkeletonData)
#[derive(Debug)]
pub struct SkeletonData {
    c_skeleton_data: SyncPtr<spSkeletonData>,
    owns_memory: bool,
    // TODO: this atlas arc is kind of a hack
    // skeleton data should keep a reference to data it requires
    // but that will not be an atlas if a custom attachment loader is used
    _atlas: Option<Arc<Atlas>>,
}

impl NewFromPtr<spSkeletonData> for SkeletonData {
    unsafe fn new_from_ptr(c_skeleton_data: *const spSkeletonData) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data as *mut spSkeletonData),
            owns_memory: false,
            _atlas: None,
        }
    }
}

impl SkeletonData {
    pub(crate) fn new(c_skeleton_data: *mut spSkeletonData, atlas: Option<Arc<Atlas>>) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
            owns_memory: true,
            _atlas: atlas,
        }
    }

    pub fn find_bone(&self, name: &str) -> Option<CTmpRef<SkeletonData, BoneData>> {
        self.bones().find(|bone| bone.name() == name)
    }

    pub fn find_bone_mut(&mut self, name: &str) -> Option<CTmpMut<SkeletonData, BoneData>> {
        self.bones_mut().find(|bone| bone.name() == name)
    }

    pub fn find_slot(&self, name: &str) -> Option<CTmpRef<SkeletonData, SlotData>> {
        self.slots().find(|slot| slot.name() == name)
    }

    pub fn find_slot_mut(&mut self, name: &str) -> Option<CTmpMut<SkeletonData, SlotData>> {
        self.slots_mut().find(|slot| slot.name() == name)
    }

    pub fn find_skin(&self, name: &str) -> Option<CTmpRef<SkeletonData, Skin>> {
        self.skins().find(|skin| skin.name() == name)
    }

    pub fn find_skin_mut(&mut self, name: &str) -> Option<CTmpMut<SkeletonData, Skin>> {
        self.skins_mut().find(|skin| skin.name() == name)
    }

    pub fn find_animation(&self, name: &str) -> Option<CTmpRef<SkeletonData, Animation>> {
        self.animations().find(|animation| animation.name() == name)
    }

    pub fn find_animation_mut(&mut self, name: &str) -> Option<CTmpMut<SkeletonData, Animation>> {
        self.animations_mut()
            .find(|animation| animation.name() == name)
    }

    c_accessor_string!(version, version);
    c_accessor_string!(hash, hash);
    c_accessor_string!(images_path, imagesPath);
    c_accessor_string!(audio_path, audioPath);
    c_accessor!(x, x, f32);
    c_accessor!(y, y, f32);
    c_accessor!(width, width, f32);
    c_accessor!(height, height, f32);
    c_accessor!(bones_count, bonesCount, i32);
    c_accessor!(slots_count, slotsCount, i32);
    c_accessor!(skins_count, skinsCount, i32);
    c_accessor!(events_count, eventsCount, i32);
    c_accessor!(animations_count, animationsCount, i32);
    c_accessor!(ik_constraints_count, ikConstraintsCount, i32);
    c_accessor!(transform_constraints_count, transformConstraintsCount, i32);
    c_accessor!(path_constraints_count, pathConstraintsCount, i32);
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
    c_accessor_array!(
        slots,
        slots_mut,
        slot_at_index,
        slot_at_index_mut,
        SkeletonData,
        SlotData,
        spSlotData,
        slots,
        slots_count
    );
    c_accessor_array!(
        skins,
        skins_mut,
        skin_at_index,
        skin_at_index_mut,
        SkeletonData,
        Skin,
        spSkin,
        skins,
        skins_count
    );
    c_accessor_tmp_ptr!(default_skin, default_skin_mut, defaultSkin, Skin, spSkin);
    c_accessor_array!(
        animations,
        animations_mut,
        animation_at_index,
        animation_at_index_mut,
        SkeletonData,
        Animation,
        spAnimation,
        animations,
        animations_count
    );
    c_ptr!(c_skeleton_data, spSkeletonData);

    // TODO: accessors and methods for the arrays in spSkeletonData
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl SkeletonData {
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn size(&self) -> Vector2<f32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }
}

impl Drop for SkeletonData {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeletonData_dispose(self.c_skeleton_data.0);
            }
        }
    }
}
