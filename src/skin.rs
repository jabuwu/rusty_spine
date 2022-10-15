use std::ffi::CString;
use crate::{
    c::{
        spSkeletonData, spSkin, spSkin_getAttachments, spSkin_create, spSkin_addSkin,
        spSkin_copySkin, spSkin_dispose,
    },
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    Attachment, Skeleton, SkeletonData,
};

/// A container for attachments which can be applied to a skeleton.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Skin)
#[derive(Debug)]
pub struct Skin {
    c_skin: SyncPtr<spSkin>,
    owns_memory: bool,
}

impl NewFromPtr<spSkin> for Skin {
    unsafe fn new_from_ptr(c_skin: *const spSkin) -> Self {
        Self {
            c_skin: SyncPtr(c_skin as *mut spSkin),
            owns_memory: true
        }
    }
}

impl Skin {
    pub fn new(name: &str) -> Skin {
        let name_c = CString::new(name).unwrap();
        unsafe {
            Skin::new_from_ptr(spSkin_create(name_c.as_ptr()))
        }
    }

    pub fn add_skin(&mut self, other: &Skin) {
        unsafe {
            spSkin_addSkin(self.c_ptr_mut(), other.c_ptr());
        }
    }

    pub fn copy_skin(&mut self, other: &Skin) {
        unsafe {
            spSkin_copySkin(self.c_ptr_mut(), other.c_ptr());
        }
    }

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

impl Drop for Skin {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkin_dispose(self.c_skin.0);
            }
        }
    }
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
