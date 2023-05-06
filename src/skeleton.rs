use std::sync::Arc;

use crate::{
    bone::Bone,
    c::{
        spBone, spSkeleton, spSkeletonData, spSkeleton_create, spSkeleton_dispose,
        spSkeleton_getAttachmentForSlotIndex, spSkeleton_getAttachmentForSlotName,
        spSkeleton_setAttachment, spSkeleton_setBonesToSetupPose, spSkeleton_setSkin,
        spSkeleton_setSkinByName, spSkeleton_setSlotsToSetupPose, spSkeleton_setToSetupPose,
        spSkeleton_updateCache, spSkeleton_updateWorldTransform,
        spSkeleton_updateWorldTransformWith, spSkin, spSlot,
    },
    c_interface::{to_c_str, CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    error::SpineError,
    skeleton_data::SkeletonData,
    skin::Skin,
    slot::Slot,
    Attachment,
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

    /// # Safety
    ///
    /// The bone must original from this skeleton.
    pub unsafe fn update_world_transform_with(&mut self, parent: &Bone) {
        spSkeleton_updateWorldTransformWith(self.c_ptr(), parent.c_ptr());
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

    /// Set the skeleton's skin. If the skin is a user-created one (via [`Skin::new`]), then a
    /// clone is created and used instead, to help ensure memory safety. If this behavior is not
    /// desired then [`Skeleton::set_skin_unchecked`] can be used instead.
    pub fn set_skin(&mut self, skin: &Skin) {
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

    #[must_use]
    pub fn bone_root(&self) -> CTmpRef<Skeleton, Bone> {
        CTmpRef::new(self, unsafe { Bone::new_from_ptr(self.c_ptr_mut().root) })
    }

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

    pub fn set_attachment(&mut self, slot_name: &str, attachment_name: Option<&str>) -> bool {
        let c_slot_name = to_c_str(slot_name);
        if let Some(attachment_name) = attachment_name {
            let c_attachment_name = to_c_str(attachment_name);
            unsafe {
                spSkeleton_setAttachment(
                    self.c_ptr(),
                    c_slot_name.as_ptr(),
                    c_attachment_name.as_ptr(),
                ) != 0
            }
        } else {
            unsafe {
                spSkeleton_setAttachment(self.c_ptr(), c_slot_name.as_ptr(), std::ptr::null()) != 0
            }
        }
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

    c_accessor_tmp_ptr!(data, data_mut, data, SkeletonData, spSkeletonData);
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor!(bones_count, bonesCount, usize);
    c_accessor!(slots_count, slotsCount, usize);
    c_accessor!(ik_contraints_count, ikConstraintsCount, usize);
    c_accessor!(transform_contraints_count, transformConstraintsCount, usize);
    c_accessor!(path_contraints_count, pathConstraintsCount, usize);
    c_accessor_mut!(scale_x, set_scale_x, scaleX, f32);
    c_accessor_mut!(scale_y, set_scale_y, scaleY, f32);
    c_accessor_mut!(x, set_x, x, f32);
    c_accessor_mut!(y, set_y, y, f32);
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
    c_accessor_tmp_ptr_optional!(skin, skin_mut, skin, Skin, spSkin);
    c_ptr!(c_skeleton, spSkeleton);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl Skeleton {
    #[must_use]
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn set_position(&mut self, position: impl Into<Vector2<f32>>) {
        let position: Vector2<f32> = position.into();
        self.set_x(position.x);
        self.set_y(position.y);
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
