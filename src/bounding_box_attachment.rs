use crate::{
    c::{spAttachment, spBoundingBoxAttachment, spVertexAttachment},
    c_interface::{NewFromPtr, SyncPtr},
};

#[derive(Debug)]
pub struct BoundingBoxAttachment {
    c_bounding_box_attachment: SyncPtr<spBoundingBoxAttachment>,
}

impl NewFromPtr<spBoundingBoxAttachment> for BoundingBoxAttachment {
    unsafe fn new_from_ptr(c_bounding_box_attachment: *const spBoundingBoxAttachment) -> Self {
        Self {
            c_bounding_box_attachment: SyncPtr(
                c_bounding_box_attachment as *mut spBoundingBoxAttachment,
            ),
        }
    }
}

impl BoundingBoxAttachment {
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0.super_0 }
    }

    fn vertex_attachment(&self) -> &spVertexAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    c_attachment_accessors!();
    c_vertex_attachment_accessors!();
    c_accessor_color!(color, color);
    c_ptr!(c_bounding_box_attachment, spBoundingBoxAttachment);
}
