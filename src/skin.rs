use crate::{
    c::{spSkeletonData, spSkin, spSkin_getAttachments},
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    Attachment, Skeleton, SkeletonData,
};

#[derive(Debug)]
pub struct Skin {
    c_skin: SyncPtr<spSkin>,
}

impl NewFromPtr<spSkin> for Skin {
    unsafe fn new_from_ptr(c_skin: *const spSkin) -> Self {
        Self {
            c_skin: SyncPtr(c_skin as *mut spSkin),
        }
    }
}

impl Skin {
    pub fn attachments(&self) -> Vec<AttachmentEntry> {
        let mut attachments = vec![];
        unsafe {
            let mut entry = spSkin_getAttachments(self.c_ptr());
            while !entry.is_null() {
                attachments.push(AttachmentEntry {
                    slot_index: (*entry).slotIndex,
                    attachment: Attachment::new_from_ptr((*entry).attachment),
                });
                entry = (*entry).next;
            }
        }
        attachments
    }

    c_accessor_string!(name, name);
    c_ptr!(c_skin, spSkin);
    // TODO: accessors
}

c_handle_decl!(SkinHandle, Skin, SkeletonData, spSkin, spSkeletonData);

impl<'a> CTmpRef<'a, SkeletonData, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpMut<'a, SkeletonData, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), self.parent.c_ptr())
    }
}

impl<'a> CTmpRef<'a, Skeleton, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), unsafe { self.parent.c_ptr_mut().data })
    }
}

impl<'a> CTmpMut<'a, Skeleton, Skin> {
    pub fn handle(&self) -> SkinHandle {
        SkinHandle::new(self.c_ptr(), unsafe { self.parent.c_ptr_mut().data })
    }
}

pub struct AttachmentEntry {
    pub slot_index: i32,
    pub attachment: Attachment,
}
