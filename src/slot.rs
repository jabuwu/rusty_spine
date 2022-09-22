use crate::{
    attachment::Attachment,
    bone::Bone,
    c::{
        spAttachment, spBone, spSkeleton, spSlot, spSlotData, spSlot_setAttachment,
        spSlot_setToSetupPose,
    },
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
    Skeleton,
};

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
    pub fn set_attachment(&mut self, attachment: &Attachment) {
        unsafe {
            spSlot_setAttachment(self.c_ptr(), attachment.c_ptr());
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

    // TODO: accessors
}

c_handle_decl!(SlotHandle, Slot, Skeleton, spSlot, spSkeleton);

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
    c_accessor_string!(name, name);
    c_ptr!(c_slot_data, spSlotData);
    // TODO: accessors
}
