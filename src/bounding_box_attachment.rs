use crate::{
    c::{spAttachment, spBoundingBoxAttachment, spVertexAttachment},
    c_interface::{NewFromPtr, SyncPtr},
};

/// An attachment made up of vertices for use in collision detection, hitboxes, etc.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#BoundingBoxAttachment)
#[derive(Debug)]
pub struct BoundingBoxAttachment {
    c_bounding_box_attachment: SyncPtr<spBoundingBoxAttachment>,
}

impl NewFromPtr<spBoundingBoxAttachment> for BoundingBoxAttachment {
    unsafe fn new_from_ptr(c_bounding_box_attachment: *mut spBoundingBoxAttachment) -> Self {
        Self {
            c_bounding_box_attachment: SyncPtr(c_bounding_box_attachment),
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
    c_ptr!(c_bounding_box_attachment, spBoundingBoxAttachment);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl BoundingBoxAttachment {
    c_vertex_attachment_accessors_mint!();
}
