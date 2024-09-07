use std::sync::Arc;

use crate::{
    animation::Animation,
    bone::BoneData,
    c::{
        spAnimation, spBoneData, spIkConstraintData, spPathConstraintData, spSkeletonData,
        spSkeletonData_dispose, spSkin, spSlotData, spTransformConstraintData,
    },
    c_interface::{CTmpRef, NewFromPtr, SyncPtr},
    skin::Skin,
    slot::SlotData,
    Atlas, IkConstraintData, PathConstraintData, TransformConstraintData,
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
    unsafe fn new_from_ptr(c_skeleton_data: *mut spSkeletonData) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
            owns_memory: false,
            _atlas: None,
        }
    }
}

impl SkeletonData {
    pub(crate) const fn new(
        c_skeleton_data: *mut spSkeletonData,
        atlas: Option<Arc<Atlas>>,
    ) -> Self {
        Self {
            c_skeleton_data: SyncPtr(c_skeleton_data),
            owns_memory: true,
            _atlas: atlas,
        }
    }

    #[must_use]
    pub fn find_bone(&self, name: &str) -> Option<CTmpRef<SkeletonData, BoneData>> {
        self.bones().find(|bone| bone.name() == name)
    }

    #[must_use]
    pub fn find_slot(&self, name: &str) -> Option<CTmpRef<SkeletonData, SlotData>> {
        self.slots().find(|slot| slot.name() == name)
    }

    #[must_use]
    pub fn find_skin(&self, name: &str) -> Option<CTmpRef<SkeletonData, Skin>> {
        self.skins().find(|skin| skin.name() == name)
    }

    #[must_use]
    pub fn find_animation(&self, name: &str) -> Option<CTmpRef<SkeletonData, Animation>> {
        self.animations().find(|animation| animation.name() == name)
    }

    #[must_use]
    pub fn find_ik_constraint(
        &self,
        name: &str,
    ) -> Option<CTmpRef<SkeletonData, IkConstraintData>> {
        self.ik_constraints()
            .find(|ik_constraint| ik_constraint.name() == name)
    }

    #[must_use]
    pub fn find_path_constraint(
        &self,
        name: &str,
    ) -> Option<CTmpRef<SkeletonData, PathConstraintData>> {
        self.path_constraints()
            .find(|path_constraint| path_constraint.name() == name)
    }

    #[must_use]
    pub fn find_transform_constraint(
        &self,
        name: &str,
    ) -> Option<CTmpRef<SkeletonData, TransformConstraintData>> {
        self.transform_constraints()
            .find(|transform_constraint| transform_constraint.name() == name)
    }

    c_accessor_string_optional!(
        /// The Spine version used to export the skeleton data, or [`None`].
        version,
        version
    );
    c_accessor_string!(
        /// The skeleton data hash. This value will change if any of the skeleton data has changed.
        hash,
        hash
    );
    c_accessor!(
        /// The X coordinate of the skeleton's axis aligned bounding box in the setup pose.
        x,
        x,
        f32
    );
    c_accessor!(
        /// The Y coordinate of the skeleton's axis aligned bounding box in the setup pose.
        y,
        y,
        f32
    );
    c_accessor!(
        /// The width of the skeleton's axis aligned bounding box in the setup pose.
        width,
        width,
        f32
    );
    c_accessor!(
        /// The height of the skeleton's axis aligned bounding box in the setup pose.
        height,
        height,
        f32
    );
    c_accessor!(bones_count, bonesCount, usize);
    c_accessor!(slots_count, slotsCount, usize);
    c_accessor!(skins_count, skinsCount, usize);
    c_accessor!(events_count, eventsCount, usize);
    c_accessor!(animations_count, animationsCount, usize);
    c_accessor!(
        /// The number of IK constraints in this skeleton.
        ik_contraints_count,
        ikConstraintsCount,
        usize
    );
    c_accessor!(
        /// The number of path constraints in this skeleton.
        path_contraints_count,
        pathConstraintsCount,
        usize
    );
    c_accessor!(
        /// The number of transform constraints in this skeleton.
        transform_contraints_count,
        transformConstraintsCount,
        usize
    );
    c_accessor_array!(
        bones,
        bone_at_index,
        SkeletonData,
        BoneData,
        spBoneData,
        bones,
        bones_count
    );
    c_accessor_array!(
        slots,
        slot_at_index,
        SkeletonData,
        SlotData,
        spSlotData,
        slots,
        slots_count
    );
    c_accessor_array!(
        skins,
        skin_at_index,
        SkeletonData,
        Skin,
        spSkin,
        skins,
        skins_count
    );
    c_accessor_tmp_ptr!(default_skin, defaultSkin, Skin, spSkin);
    c_accessor_array!(
        animations,
        animation_at_index,
        SkeletonData,
        Animation,
        spAnimation,
        animations,
        animations_count
    );
    c_accessor_array!(
        ik_constraints,
        ik_contraint_at_index,
        SkeletonData,
        IkConstraintData,
        spIkConstraintData,
        ikConstraints,
        ik_contraints_count
    );
    c_accessor_array!(
        path_constraints,
        path_contraint_at_index,
        SkeletonData,
        PathConstraintData,
        spPathConstraintData,
        pathConstraints,
        path_contraints_count
    );
    c_accessor_array!(
        transform_constraints,
        transform_contraint_at_index,
        SkeletonData,
        TransformConstraintData,
        spTransformConstraintData,
        transformConstraints,
        transform_contraints_count
    );
    c_ptr!(c_skeleton_data, spSkeletonData);

    // TODO: accessors and methods for the arrays in spSkeletonData
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl SkeletonData {
    /// The translation of the skeleton's axis aligned bounding box in the setup pose.
    #[must_use]
    pub fn translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    /// The size of the skeleton's axis aligned bounding box in the setup pose.
    #[must_use]
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
