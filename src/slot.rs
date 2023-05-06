use crate::{
    attachment::Attachment,
    bone::Bone,
    c::{
        spAttachment, spBlendMode, spBone, spBoneData, spBoundingBoxAttachment,
        spClippingAttachment, spMeshAttachment, spPointAttachment, spRegionAttachment, spSkeleton,
        spSlot, spSlotData, spSlotData_setAttachmentName, spSlot_setAttachment,
        spSlot_setToSetupPose,
    },
    c_interface::{to_c_str, CTmpRef, NewFromPtr, SyncPtr},
    AttachmentType, BoneData, BoundingBoxAttachment, ClippingAttachment, MeshAttachment,
    PointAttachment, RegionAttachment, Skeleton,
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

macro_rules! attachment_accessor {
    ($fn:ident, $fn_mut:ident, $type:ident, $c_type:ident, $attachment_type:expr) => {
        #[must_use]
        pub fn $fn(&self) -> Option<CTmpRef<Self, $type>> {
            let attachment = unsafe { self.c_ptr_ref().attachment };
            if !attachment.is_null() {
                if AttachmentType::from(unsafe { (*attachment).type_0 }) == $attachment_type {
                    #[allow(unused_unsafe)]
                    Some(CTmpRef::new(self, unsafe {
                        ($type::new_from_ptr(attachment as *mut $c_type))
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

impl Slot {
    /// Sets the attachment for this slot.
    ///
    /// # Safety
    ///
    /// The attachment must be compatible with this slot, usually by originating from it.
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

    #[must_use]
    pub fn handle(&self) -> SlotHandle {
        SlotHandle::new(self.c_ptr(), unsafe { self.bone().c_ptr_mut().skeleton })
    }

    attachment_accessor!(
        region_attachment,
        region_attachment_mut,
        RegionAttachment,
        spRegionAttachment,
        AttachmentType::Region
    );

    attachment_accessor!(
        bounding_box_attachment,
        bounding_box_attachment_mut,
        BoundingBoxAttachment,
        spBoundingBoxAttachment,
        AttachmentType::BoundingBox
    );

    attachment_accessor!(
        mesh_attachment,
        mesh_attachment_mut,
        MeshAttachment,
        spMeshAttachment,
        AttachmentType::Mesh
    );

    attachment_accessor!(
        point_attachment,
        point_attachment_mut,
        PointAttachment,
        spPointAttachment,
        AttachmentType::Point
    );

    attachment_accessor!(
        clipping_attachment,
        clipping_attachment_mut,
        ClippingAttachment,
        spClippingAttachment,
        AttachmentType::Clipping
    );

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
    c_accessor!(sequence_index, sequenceIndex, usize);

    // TODO: accessors for deform
}

c_handle_decl!(
    /// A storeable reference to a [`Slot`].
    ///
    /// Can be acquired from any instance of [`Slot`].
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, EventType, SlotHandle};
    /// # let (skeleton, _) = test::TestAsset::spineboy().instance();
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
        let c_attachment_name = to_c_str(attachment_name);
        unsafe { spSlotData_setAttachmentName(self.c_ptr(), c_attachment_name.as_ptr()) }
    }

    c_accessor!(index, index, usize);
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
            1 => Self::Additive,
            2 => Self::Multiply,
            3 => Self::Screen,
            _ => Self::Normal,
        }
    }
}
