use crate::{
    attachment::Attachment,
    bone::Bone,
    c::{spSlot, spSlotData},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct Slot {
    c_slot: SyncPtr<spSlot>,
    data: SlotData,
    bone: Bone,
}

impl Slot {
    pub(crate) fn new_from_ptr(c_slot: *const spSlot) -> Self {
        let c_slot_data = unsafe { (*c_slot).data };
        Self {
            c_slot: SyncPtr(c_slot as *mut spSlot),
            data: SlotData::new(c_slot_data),
            bone: unsafe { Bone::new_from_ptr((*c_slot).bone) },
        }
    }

    pub fn data(&self) -> &SlotData {
        &self.data
    }

    pub fn bone(&self) -> &Bone {
        &self.bone
    }

    pub fn bone_mut(&mut self) -> &mut Bone {
        &mut self.bone
    }

    pub fn attachment(&self) -> Option<Attachment> {
        let c_attachment = self.c_ptr_ref().attachment;
        if !c_attachment.is_null() {
            Some(Attachment::new_from_ptr(c_attachment))
        } else {
            None
        }
    }

    c_ptr!(c_slot, spSlot);
    c_accessor_color!(color, color_mut, color);

    // TODO: accessors
}

#[derive(Debug)]
pub struct SlotData {
    c_slot_data: SyncPtr<spSlotData>,
}

impl SlotData {
    fn new(c_slot_data: *mut spSlotData) -> Self {
        Self {
            c_slot_data: SyncPtr(c_slot_data),
        }
    }

    c_ptr!(c_slot_data, spSlotData);
    c_accessor_string!(name, name);
    // TODO: accessors
}
