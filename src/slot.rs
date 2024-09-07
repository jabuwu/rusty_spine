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
    unsafe fn new_from_ptr(c_slot: *mut spSlot) -> Self {
        Self {
            c_slot: SyncPtr(c_slot),
        }
    }
}

macro_rules! attachment_accessor {
    // TODO: fn_mut is unused? should fix this?
    ($(#[$($attrss1:tt)*])* $fn:ident, $(#[$($attrss2:tt)*])* $fn_mut:ident, $type:ident, $c_type:ident, $attachment_type:expr) => {
        $(#[$($attrss1)*])*
        #[must_use]
        pub fn $fn(&self) -> Option<CTmpRef<Self, $type>> {
            let attachment = unsafe { self.c_ptr_ref().attachment };
            if !attachment.is_null() {
                if AttachmentType::from(unsafe { (*attachment).type_0 }) == $attachment_type {
                    #[allow(unused_unsafe)]
                    Some(CTmpRef::new(self, unsafe {
                        ($type::new_from_ptr(attachment.cast::<$c_type>()))
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
    // TODO: add attachment() accessor?

    /// Sets the attachment for this slot.
    ///
    /// # Safety
    ///
    /// The attachment must be compatible with this slot, usually by originating from it.
    pub unsafe fn set_attachment(&mut self, attachment: Option<Attachment>) {
        attachment.map_or_else(
            || {
                spSlot_setAttachment(self.c_ptr(), std::ptr::null_mut());
            },
            |attachment| {
                spSlot_setAttachment(self.c_ptr(), attachment.c_ptr());
            },
        );
    }

    /// Sets this slot to the setup pose.
    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spSlot_setToSetupPose(self.c_ptr());
        }
    }

    /// Create a persistent [`SlotHandle`] to this [`Slot`].
    #[must_use]
    pub fn handle(&self) -> SlotHandle {
        SlotHandle::new(self.c_ptr(), unsafe { self.bone().c_ptr_mut().skeleton })
    }

    attachment_accessor!(
        /// The [`RegionAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        region_attachment,
        /// The mutable [`RegionAttachment`] attached to this slot, or [`None`] if the attachment is
        /// a different type.
        region_attachment_mut,
        RegionAttachment,
        spRegionAttachment,
        AttachmentType::Region
    );

    attachment_accessor!(
        /// The [`BoundingBoxAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        bounding_box_attachment,
        /// The mutable [`BoundingBoxAttachment`] attached to this slot, or [`None`] if the
        /// attachment is a different type.
        bounding_box_attachment_mut,
        BoundingBoxAttachment,
        spBoundingBoxAttachment,
        AttachmentType::BoundingBox
    );

    attachment_accessor!(
        /// The [`MeshAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        mesh_attachment,
        /// The mutable [`MeshAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        mesh_attachment_mut,
        MeshAttachment,
        spMeshAttachment,
        AttachmentType::Mesh
    );

    attachment_accessor!(
        /// The [`PointAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        point_attachment,
        /// The mutable [`PointAttachment`] attached to this slot, or [`None`] if the attachment is
        /// a different type.
        point_attachment_mut,
        PointAttachment,
        spPointAttachment,
        AttachmentType::Point
    );

    attachment_accessor!(
        /// The [`ClippingAttachment`] attached to this slot, or [`None`] if the attachment is a
        /// different type.
        clipping_attachment,
        /// The mutable [`ClippingAttachment`] attached to this slot, or [`None`] if the attachment
        /// is a different type.
        clipping_attachment_mut,
        ClippingAttachment,
        spClippingAttachment,
        AttachmentType::Clipping
    );

    c_accessor_color_mut!(
        /// The color used to tint the slot's attachment. If [`dark_color`](`Self::dark_color`) is
        /// set, this is used as the light color for two color tinting.
        color,
        /// Set the color used to tint the slot's attachment. If [`dark_color`](`Self::dark_color`)
        /// is set, this is used as the light color for two color tinting.
        color_mut,
        color
    );
    c_accessor_color_optional!(
        /// The dark color used to tint the slot's attachment for two color tinting, or [`None`] if
        /// two color tinting is not used. The dark color's alpha is not used.
        dark_color,
        darkColor
    );
    c_accessor_tmp_ptr_mut!(
        /// The slot's setup pose data.
        data,
        /// The slot's mutable setup pose data.
        data_mut,
        data,
        SlotData,
        spSlotData
    );
    c_accessor_tmp_ptr_mut!(
        /// The bone this slot belongs to.
        bone,
        /// The mutable bone this slot belongs to.
        bone_mut,
        bone,
        Bone,
        spBone
    );
    c_accessor_tmp_ptr_optional_mut!(
        /// The current attachment for the slot, or [`None`] if the slot has no attachment.
        attachment,
        /// The current mutable attachment for the slot, or [`None`] if the slot has no attachment.
        attachment_mut,
        attachment,
        Attachment,
        spAttachment
    );
    c_ptr!(c_slot, spSlot);

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
    /// # let (skeleton, _) = test::TestAsset::spineboy().instance(true);
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
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#SlotData)
#[derive(Debug)]
pub struct SlotData {
    c_slot_data: SyncPtr<spSlotData>,
}

impl NewFromPtr<spSlotData> for SlotData {
    unsafe fn new_from_ptr(c_slot_data: *mut spSlotData) -> Self {
        Self {
            c_slot_data: SyncPtr(c_slot_data),
        }
    }
}

impl SlotData {
    pub fn set_attachment_name(&mut self, attachment_name: &str) {
        let c_attachment_name = to_c_str(attachment_name);
        unsafe { spSlotData_setAttachmentName(self.c_ptr(), c_attachment_name.as_ptr()) }
    }

    c_accessor!(
        /// The index of the slot in [`Skeleton::slots`].
        index,
        index,
        usize
    );
    c_accessor_string!(
        /// The name of the slot, which is unique across all slots in the skeleton.
        name,
        name
    );
    c_accessor_tmp_ptr!(
        /// The bone this slot belongs to.
        bone_data,
        boneData,
        BoneData,
        spBoneData
    );
    c_accessor_string_optional!(
        /// The name of the attachment that is visible for this slot in the setup pose, or [`None`]
        /// if no attachment is visible.
        attachment_name,
        attachmentName
    );
    c_accessor_color!(
        /// The color used to tint the slot's attachment. If [`dark_color`](`Self::dark_color`) is
        /// set, this is used as the light color for two color tinting.
        color,
        color
    );
    c_accessor_color_optional!(
        /// The dark color used to tint the slot's attachment for two color tinting, or [`None`] if
        /// two color tinting is not used. The dark color's alpha is not used.
        dark_color,
        darkColor
    );
    c_accessor_enum!(
        /// The blend mode for drawing the slot's attachment.
        blend_mode,
        blendMode,
        BlendMode
    );
    c_ptr!(c_slot_data, spSlotData);
}

/// The variants of blend modes supported by Spine.
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
