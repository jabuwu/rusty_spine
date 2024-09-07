use std::{borrow::Cow, sync::Arc};

use crate::{
    bone::Bone,
    c::{
        spBone, spIkConstraint, spPathConstraint, spSkeleton, spSkeletonData, spSkeleton_create,
        spSkeleton_dispose, spSkeleton_getAttachmentForSlotIndex,
        spSkeleton_getAttachmentForSlotName, spSkeleton_setAttachment,
        spSkeleton_setBonesToSetupPose, spSkeleton_setSkin, spSkeleton_setSkinByName,
        spSkeleton_setSlotsToSetupPose, spSkeleton_setToSetupPose, spSkeleton_update,
        spSkeleton_updateCache, spSkeleton_updateWorldTransform, spSkin, spSlot,
        spTransformConstraint,
    },
    c_interface::{to_c_str, CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    error::SpineError,
    skeleton_data::SkeletonData,
    skin::Skin,
    slot::Slot,
    Attachment, IkConstraint, PathConstraint, TransformConstraint,
};

#[allow(unused_imports)]
use crate::{SkeletonBinary, SkeletonJson};

#[cfg(feature = "mint")]
use mint::Vector2;

/// A live Skeleton instance created from [`SkeletonData`].
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Skeleton)
#[derive(Debug)]
pub struct Skeleton {
    c_skeleton: SyncPtr<spSkeleton>,
    owns_memory: bool,
    _skeleton_data: Arc<SkeletonData>,
    _skin: Option<Skin>, // keep-alive for user created skins
}

impl Skeleton {
    /// Create a new instance of the skeleton loaded in [`SkeletonData`].
    ///
    /// See [`SkeletonJson`] or [`SkeletonBinary`] for a complete example of loading a skeleton.
    #[must_use]
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_skeleton = unsafe { spSkeleton_create(skeleton_data.c_ptr()) };
        Self {
            c_skeleton: SyncPtr(c_skeleton),
            owns_memory: true,
            _skeleton_data: skeleton_data,
            _skin: None, // keep alive user-created skins
        }
    }

    pub fn update(&mut self, delta: f32) {
        unsafe {
            spSkeleton_update(self.c_ptr(), delta);
        }
    }

    /// Caches information about bones and constraints. Must be called if the skin is modified or if
    /// bones, constraints, or weighted path attachments are added or removed.
    pub fn update_cache(&mut self) {
        unsafe {
            spSkeleton_updateCache(self.c_ptr());
        }
    }

    /// Updates the world transform for each bone and applies all constraints.
    ///
    /// See
    /// [`World transforms`](http://esotericsoftware.com/spine-runtime-skeletons#World-transforms)
    /// in the Spine Runtimes Guide.
    pub fn update_world_transform(&mut self) {
        unsafe {
            spSkeleton_updateWorldTransform(self.c_ptr());
        }
    }

    /// Sets the bones, constraints, slots, and draw order to their setup pose values.
    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setToSetupPose(self.c_ptr());
        }
    }

    /// Sets the bones and constraints to their setup pose values.
    pub fn set_bones_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setBonesToSetupPose(self.c_ptr());
        }
    }

    /// Sets the slots and draw order to their setup pose values.
    pub fn set_slots_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setSlotsToSetupPose(self.c_ptr());
        }
    }

    /// Set the skeleton's skin. If the skin is a user-created one (via [`Skin::new`]), then a
    /// clone is created and used instead, to help ensure memory safety. If this behavior is not
    /// desired then [`Skeleton::set_skin_unchecked`] can be used instead.
    ///
    /// # Safety
    ///
    /// The skin must originate from the same [`SkeletonData`] that this skeleton uses.
    pub unsafe fn set_skin(&mut self, skin: &Skin) {
        if skin.owns_memory {
            let cloned_skin = skin.clone();
            unsafe { spSkeleton_setSkin(self.c_ptr(), cloned_skin.c_ptr()) };
            self._skin = Some(cloned_skin);
        } else {
            unsafe { spSkeleton_setSkin(self.c_ptr(), skin.c_ptr()) };
            self._skin = None;
        }
        self.set_to_setup_pose();
    }

    /// Set the skeleton's skin.
    ///
    /// # Safety
    ///
    /// The skin must originate from the same [`SkeletonData`] that this skeleton uses.
    ///
    /// The [`Skin`] struct will destroy the underlying C representation of the skin in its [`Drop`]
    /// implementation. Skins assigned to a skeleton must live as long as the skeletons using them
    /// or else the skeleton may cause a segfault.
    pub unsafe fn set_skin_unchecked(&mut self, skin: &Skin) {
        spSkeleton_setSkin(self.c_ptr(), skin.c_ptr());
    }

    /// Set the skeleton's skin by name.
    ///
    /// # Safety
    ///
    /// This function should only be called with valid skin names. It is faster than the safe
    /// alternative, [`Skeleton::set_skin_by_name`], but will likely segfault if the skin does not
    /// exist.
    pub unsafe fn set_skin_by_name_unchecked(&mut self, skin_name: &str) {
        let c_skin_name = to_c_str(skin_name);
        spSkeleton_setSkinByName(self.c_ptr(), c_skin_name.as_ptr());
        self._skin = None;
    }

    /// Set the skeleton's skin by name.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::NotFound`] if the specified skin name doesn't exist.
    pub fn set_skin_by_name(&mut self, skin_name: &str) -> Result<(), SpineError> {
        if self.data().skins().any(|skin| skin.name() == skin_name) {
            unsafe { self.set_skin_by_name_unchecked(skin_name) };
            Ok(())
        } else {
            Err(SpineError::new_not_found("Skin", skin_name))
        }
    }

    /// Create a conglomerate skin containing `skin_names` and attach to this skeleton.
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, AnimationEvent};
    /// # let (mut skeleton, _) = test::TestAsset::spineboy().instance(true);
    /// skeleton.set_skins_by_name("combined-skin", ["hat", "suit", "tie"]);
    /// ```
    ///
    /// The name assigned to this skin (via `combined_skin_name`) is unimportant and does not need
    /// to be unique.
    ///
    /// A faster (but `unsafe`) way to create conglomerate skins is to use [`Skin::new`] and
    /// [`Skin::add_skin`] to create a pre-configured skin that can be attached at any time with
    /// [`Skeleton::set_skin`].
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::NotFound`] if any of the specified skin names do not exist (in this
    /// case, the current skin remains unchanged).
    pub fn set_skins_by_name<'a, T>(
        &mut self,
        combined_skin_name: &str,
        skin_names: impl IntoIterator<Item = T>,
    ) -> Result<(), SpineError>
    where
        Cow<'a, str>: From<T>,
    {
        let mut combined_skin = Skin::new(combined_skin_name);
        for skin_name in skin_names {
            let skin_name_str = &*Cow::<'a, str>::from(skin_name);
            unsafe {
                combined_skin.add_skin(
                    self.data()
                        .find_skin(skin_name_str)
                        .ok_or_else(|| SpineError::new_not_found("Skin", skin_name_str))?
                        .as_ref(),
                );
            }
        }
        unsafe {
            self.set_skin(&combined_skin);
        }
        Ok(())
    }

    /// The root bone, or [`None`] if the skeleton has no bones.
    #[must_use]
    pub fn bone_root(&self) -> CTmpRef<Skeleton, Bone> {
        CTmpRef::new(self, unsafe { Bone::new_from_ptr(self.c_ptr_mut().root) })
    }

    /// The mutable root bone, or [`None`] if the skeleton has no bones.
    #[must_use]
    pub fn bone_root_mut(&mut self) -> CTmpMut<Skeleton, Bone> {
        CTmpMut::new(self, unsafe { Bone::new_from_ptr(self.c_ptr_mut().root) })
    }

    #[must_use]
    pub fn find_bone(&self, name: &str) -> Option<CTmpRef<Skeleton, Bone>> {
        self.bones().find(|bone| bone.data().name() == name)
    }

    #[must_use]
    pub fn find_bone_mut(&mut self, name: &str) -> Option<CTmpMut<Skeleton, Bone>> {
        self.bones_mut().find(|bone| bone.data().name() == name)
    }

    #[must_use]
    pub fn find_slot(&self, name: &str) -> Option<CTmpRef<Skeleton, Slot>> {
        self.slots().find(|slot| slot.data().name() == name)
    }

    #[must_use]
    pub fn find_slot_mut(&mut self, name: &str) -> Option<CTmpMut<Skeleton, Slot>> {
        self.slots_mut().find(|slot| slot.data().name() == name)
    }

    #[must_use]
    pub fn find_ik_constraint(&self, name: &str) -> Option<CTmpRef<Skeleton, IkConstraint>> {
        self.ik_constraints()
            .find(|ik_constraint| ik_constraint.data().name() == name)
    }

    #[must_use]
    pub fn find_ik_constraint_mut(
        &mut self,
        name: &str,
    ) -> Option<CTmpMut<Skeleton, IkConstraint>> {
        self.ik_constraints_mut()
            .find(|ik_constraint| ik_constraint.data().name() == name)
    }

    #[must_use]
    pub fn find_path_constraint(&self, name: &str) -> Option<CTmpRef<Skeleton, PathConstraint>> {
        self.path_constraints()
            .find(|path_constraint| path_constraint.data().name() == name)
    }

    #[must_use]
    pub fn find_path_constraint_mut(
        &mut self,
        name: &str,
    ) -> Option<CTmpMut<Skeleton, PathConstraint>> {
        self.path_constraints_mut()
            .find(|path_constraint| path_constraint.data().name() == name)
    }

    #[must_use]
    pub fn find_transform_constraint(
        &self,
        name: &str,
    ) -> Option<CTmpRef<Skeleton, TransformConstraint>> {
        self.transform_constraints()
            .find(|transform_constraint| transform_constraint.data().name() == name)
    }

    #[must_use]
    pub fn find_transform_constraint_mut(
        &mut self,
        name: &str,
    ) -> Option<CTmpMut<Skeleton, TransformConstraint>> {
        self.transform_constraints_mut()
            .find(|transform_constraint| transform_constraint.data().name() == name)
    }

    pub fn set_attachment(&mut self, slot_name: &str, attachment_name: Option<&str>) -> bool {
        let c_slot_name = to_c_str(slot_name);
        attachment_name.map_or_else(
            || unsafe {
                spSkeleton_setAttachment(self.c_ptr(), c_slot_name.as_ptr(), std::ptr::null()) != 0
            },
            |attachment_name| {
                let c_attachment_name = to_c_str(attachment_name);
                unsafe {
                    spSkeleton_setAttachment(
                        self.c_ptr(),
                        c_slot_name.as_ptr(),
                        c_attachment_name.as_ptr(),
                    ) != 0
                }
            },
        )
    }

    pub fn get_attachment_for_slot_name(
        &mut self,
        slot_name: &str,
        attachment_name: &str,
    ) -> Option<Attachment> {
        let c_slot_name = to_c_str(slot_name);
        let c_attachment_name = to_c_str(attachment_name);
        unsafe {
            let c_attachment = spSkeleton_getAttachmentForSlotName(
                self.c_ptr(),
                c_slot_name.as_ptr(),
                c_attachment_name.as_ptr(),
            );
            if !c_attachment.is_null() {
                Some(Attachment::new_from_ptr(c_attachment))
            } else {
                None
            }
        }
    }

    pub fn get_attachment_for_slot_index(
        &mut self,
        slot_index: usize,
        attachment_name: &str,
    ) -> Option<Attachment> {
        let c_attachment_name = to_c_str(attachment_name);
        unsafe {
            let c_attachment = spSkeleton_getAttachmentForSlotIndex(
                self.c_ptr(),
                slot_index as i32,
                c_attachment_name.as_ptr(),
            );
            if !c_attachment.is_null() {
                Some(Attachment::new_from_ptr(c_attachment))
            } else {
                None
            }
        }
    }

    // TODO: iterators for ik, transform, path constraints

    c_accessor_tmp_ptr_mut!(
        /// The skeleton's setup pose data.
        data,
        /// The skeleton's mutable setup pose data.
        data_mut,
        data,
        SkeletonData,
        spSkeletonData
    );
    c_accessor_color_mut!(
        /// The color to tint all the skeleton's attachments.
        color,
        /// Set the color to tint all the skeleton's attachments.
        color_mut,
        color
    );
    c_accessor!(
        /// The number of bones in this skeleton.
        bones_count,
        bonesCount,
        usize
    );
    c_accessor!(
        /// The number of slots in this skeleton.
        slots_count,
        slotsCount,
        usize
    );
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
    c_accessor_mut!(
        /// Scales the entire skeleton on the X axis.
        scale_x,
        /// Sets the scale the entire skeleton on the X axis.
        /// Bones that do not inherit scale are still affected by this property.
        set_scale_x,
        scaleX,
        f32
    );
    c_accessor_mut!(
        /// Scales the entire skeleton on the Y axis.
        scale_y,
        /// Sets the scale the entire skeleton on the Y axis.
        /// Bones that do not inherit scale are still affected by this property.
        set_scale_y,
        scaleY,
        f32
    );
    c_accessor_mut!(
        /// The skeleton X translation, which is added to the root bone worldX translation.
        x,
        /// Sets the skeleton X translation, which is added to the root bone worldX translation.
        /// Bones that do not inherit translation are still affected by this property.
        set_x,
        x,
        f32
    );
    c_accessor_mut!(
        /// The skeleton Y translation, which is added to the root bone worldY translation.
        y,
        /// Sets the skeleton Y translation, which is added to the root bone worldY translation.
        /// Bones that do not inherit translation are still affected by this property.
        set_y,
        y,
        f32
    );
    c_accessor_array_mut!(
        /// An iterator to the skeleton's bones.
        bones,
        /// A mutable iterator to the skeleton's bones.
        bones_mut,
        /// The nth bone in the skeleton.
        bone_at_index,
        /// The nth mutable bone in the skeleton.
        bone_at_index_mut,
        Skeleton,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_array_mut!(
        /// An iterator to the skeleton's slots.
        slots,
        /// A mutable iterator to the skeleton's slots.
        slots_mut,
        /// The nth slot in the skeleton.
        slot_at_index,
        /// The nth mutable slot in the skeleton.
        slot_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        slots,
        slots_count
    );
    c_accessor_array_mut!(
        /// An iterator to the skeleton's slots in the order they should be drawn.
        draw_order,
        /// A mutable iterator to the skeleton's slots in the order they should be drawn.
        draw_order_mut,
        /// The nth skeleton slot, indexed in the order they should be drawn.
        draw_order_at_index,
        /// The nth mutable skeleton slot, indexed in the order they should be drawn.
        draw_order_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        drawOrder,
        slots_count
    );
    c_accessor_array_mut!(
        ik_constraints,
        ik_constraints_mut,
        ik_contraint_at_index,
        ik_constraint_at_index_mut,
        Skeleton,
        IkConstraint,
        spIkConstraint,
        ikConstraints,
        ik_contraints_count
    );
    c_accessor_array_mut!(
        path_constraints,
        path_constraints_mut,
        path_contraint_at_index,
        path_constraint_at_index_mut,
        Skeleton,
        PathConstraint,
        spPathConstraint,
        pathConstraints,
        path_contraints_count
    );
    c_accessor_array_mut!(
        transform_constraints,
        transform_constraints_mut,
        transform_contraint_at_index,
        transform_constraint_at_index_mut,
        Skeleton,
        TransformConstraint,
        spTransformConstraint,
        transformConstraints,
        transform_contraints_count
    );
    c_accessor_tmp_ptr_optional_mut!(skin, skin_mut, skin, Skin, spSkin);
    c_ptr!(c_skeleton, spSkeleton);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl Skeleton {
    #[must_use]
    pub fn translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn set_translation(&mut self, translation: impl Into<Vector2<f32>>) {
        let translation: Vector2<f32> = translation.into();
        self.set_x(translation.x);
        self.set_y(translation.y);
    }

    #[must_use]
    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    pub fn set_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_scale_x(scale.x);
        self.set_scale_y(scale.y);
    }
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeleton_dispose(self.c_skeleton.0);
            }
        }
    }
}
