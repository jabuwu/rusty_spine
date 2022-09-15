use crate::{
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
    pub(crate) fn new_from_ptr(c_slot: *mut spSlot) -> Self {
        let c_slot_data = unsafe { (*c_slot).data };
        Self {
            c_slot: SyncPtr(c_slot),
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

    c_ptr!(c_slot, spSlot);

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
