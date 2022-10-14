use crate::{
    c::{c_float, spAttachment, spPathAttachment, spVertexAttachment},
    c_interface::{NewFromPtr, SyncPtr},
};

/// An attachment of vertices forming a Bezier curve.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#PathAttachment)
#[derive(Debug)]
pub struct PathAttachment {
    c_path_attachment: SyncPtr<spPathAttachment>,
}

impl NewFromPtr<spPathAttachment> for PathAttachment {
    unsafe fn new_from_ptr(c_path_attachment: *const spPathAttachment) -> Self {
        Self {
            c_path_attachment: SyncPtr(c_path_attachment as *mut spPathAttachment),
        }
    }
}

impl PathAttachment {
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0.super_0 }
    }

    fn vertex_attachment(&self) -> &spVertexAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    c_attachment_accessors!();
    c_vertex_attachment_accessors!();
    c_accessor_bool_mut!(closed, set_closed, closed);
    #[cfg(not(feature="spine38"))]
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_bool_mut!(constant_speed, set_constant_speed, constantSpeed);
    c_accessor_passthrough!(lengths, lengths, *mut c_float);
    c_ptr!(c_path_attachment, spPathAttachment);
}
