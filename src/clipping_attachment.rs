use crate::{
    c::{spAttachment, spClippingAttachment, spSlotData},
    c_interface::{NewFromPtr, SyncPtr},
    SlotData,
};

/// An attachment which clips rendering of other attachments.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#ClippingAttachment)
#[derive(Debug)]
pub struct ClippingAttachment {
    c_clipping_attachment: SyncPtr<spClippingAttachment>,
}

impl NewFromPtr<spClippingAttachment> for ClippingAttachment {
    unsafe fn new_from_ptr(c_clipping_attachment: *const spClippingAttachment) -> Self {
        Self {
            c_clipping_attachment: SyncPtr(c_clipping_attachment as *mut spClippingAttachment),
        }
    }
}

impl ClippingAttachment {
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0.super_0 }
    }

    c_attachment_accessors!();
    c_ptr!(c_clipping_attachment, spClippingAttachment);
    #[cfg(not(feature = "spine38"))]
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_tmp_ptr!(end_slot, end_slot_mut, endSlot, SlotData, spSlotData);
}
