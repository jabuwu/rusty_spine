use std::ffi::CString;

use crate::{
    attachment::Attachment,
    bone::Bone,
    c::{
        spAttachment, spBlendMode, spBone, spBoneData, spSkeleton, spSlot, spSlotData,
        spSlotData_setAttachmentName, spSlot_setAttachment, spSlot_setToSetupPose,
    },
    c_interface::{NewFromPtr, SyncPtr},
    BoneData, Skeleton,
};

/// A slot for an attachment.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Slot)
#[derive(Debug)]
pub struct Slot {
    c_slot: SyncPtr<spSlot>,
}

impl NewFromPtr<spSlot> for Slot {
    unsafe fn new_from_ptr(c_slot: *const spSlot) -> Self {
        Self {
            c_slot: SyncPtr(c_slot as *mut spSlot),
        }
    }
}

impl Slot {
    /// Sets the attachment for this slot. This function is unsafe because there is no way to know
    /// if the attachment is compatible with this slot and may segfault if used incorrectly.
    pub unsafe fn set_attachment(&mut self, attachment: Option<Attachment>) {
        if let Some(attachment) = attachment {
            spSlot_setAttachment(self.c_ptr(), attachment.c_ptr());
        } else {
            spSlot_setAttachment(self.c_ptr(), std::ptr::null_mut());
        }
    }

    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spSlot_setToSetupPose(self.c_ptr());
        }
    }

    pub fn handle(&self) -> SlotHandle {
        SlotHandle::new(self.c_ptr(), unsafe { self.bone().c_ptr_mut().skeleton })
    }

    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_color_optional!(dark_color, darkColor);
    c_accessor_tmp_ptr!(data, data_mut, data, SlotData, spSlotData);
    c_accessor_tmp_ptr!(bone, bone_mut, bone, Bone, spBone);
    c_accessor_tmp_ptr_optional!(
        attachment,
        attachment_mut,
        attachment,
        Attachment,
        spAttachment
    );
    c_ptr!(c_slot, spSlot);
    #[cfg(not(feature = "spine38"))]
    c_accessor!(sequence_index, sequenceIndex, i32);

    // TODO: accessors for deform
}

c_handle_decl!(
    /// A storeable reference to a [Slot](struct.Slot.html).
    ///
    /// Can be acquired from any instance of [Slot](struct.Slot.html).
    ///
    /// ```
    /// # #[path="./doctests.rs"]
    /// # mod doctests;
    /// # use rusty_spine::{AnimationState, EventType, SlotHandle};
    /// # let (skeleton, _) = doctests::test_spineboy_instance();
    /// let slot_handles: Vec<SlotHandle> = skeleton.slots().map(|slot| slot.handle()).collect();
    /// for slot_handle in slot_handles.iter() {
    ///     let slot = slot_handle.get(&skeleton).unwrap();
    ///     println!("{}", slot.data().name());
    /// }
    /// ```
    SlotHandle,
    Slot,
    Skeleton,
    spSlot,
    spSkeleton
);

/// Static slot data imported from Spine.
#[derive(Debug)]
pub struct SlotData {
    c_slot_data: SyncPtr<spSlotData>,
}

impl NewFromPtr<spSlotData> for SlotData {
    unsafe fn new_from_ptr(c_slot_data: *const spSlotData) -> Self {
        Self {
            c_slot_data: SyncPtr(c_slot_data as *mut spSlotData),
        }
    }
}

impl SlotData {
    pub fn set_attachment_name(&mut self, attachment_name: &str) {
        let c_attachment_name = CString::new(attachment_name).unwrap();
        unsafe { spSlotData_setAttachmentName(self.c_ptr(), c_attachment_name.as_ptr()) }
    }

    c_accessor!(index, index, i32);
    c_accessor_string!(name, name);
    c_accessor_tmp_ptr!(bone_data, bone_data_mut, boneData, BoneData, spBoneData);
    c_accessor_string!(attachment_name, attachmentName);
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_color_optional!(dark_color, darkColor);
    c_accessor_enum_mut!(blend_mode, set_blend_mode, blendMode, BlendMode);
    c_ptr!(c_slot_data, spSlotData);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal = 0,
    Additive = 1,
    Multiply = 2,
    Screen = 3,
}

impl From<spBlendMode> for BlendMode {
    fn from(attachment_type: spBlendMode) -> Self {
        match attachment_type {
            0 => Self::Normal,
            1 => Self::Additive,
            2 => Self::Multiply,
            3 => Self::Screen,
            _ => Self::Normal,
        }
    }
}
