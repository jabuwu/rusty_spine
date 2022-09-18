use crate::{
    c::{spAttachment, spBoundingBoxAttachment, spVertexAttachment},
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
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
        &self.c_ptr_ref().super_0.super_0
    }

    fn vertex_attachment(&self) -> &spVertexAttachment {
        &self.c_ptr_ref().super_0
    }

    fn vertex_attachment_mut(&mut self) -> &mut spVertexAttachment {
        &mut self.c_ptr_mut().super_0
    }

    c_ptr!(c_bounding_box_attachment, spBoundingBoxAttachment);
    c_attachment_accessors!();
    c_vertex_attachment_accessors!();
    c_accessor_color!(color, color_mut, color);
}
