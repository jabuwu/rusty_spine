use crate::{
    c::{spAttachment, spClippingAttachment},
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct ClippingAttachment {
    c_clipping_attachment: SyncPtr<spClippingAttachment>,
}

impl ClippingAttachment {
    pub fn new_from_ptr(c_clipping_attachment: *const spClippingAttachment) -> Self {
        Self {
            c_clipping_attachment: SyncPtr(c_clipping_attachment as *mut spClippingAttachment),
        }
    }

    fn attachment(&self) -> &spAttachment {
        &self.c_ptr_ref().super_0.super_0
    }

    c_ptr!(c_clipping_attachment, spClippingAttachment);
    c_attachment_accessors!(super_0.super_0);
}
