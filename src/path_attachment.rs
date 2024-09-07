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
    unsafe fn new_from_ptr(c_path_attachment: *mut spPathAttachment) -> Self {
        Self {
            c_path_attachment: SyncPtr(c_path_attachment),
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
    c_accessor_bool_mut!(
        /// If `true, the start and end knots are connected.
        closed,
        /// Set closed, see [`closed`](`Self::closed`).
        set_closed,
        closed
    );
    c_accessor_bool_mut!(
        /// If `true`, additional calculations are performed to make computing positions along the
        /// path more accurate and movement along the path have a constant speed.
        constant_speed,
        set_constant_speed,
        constantSpeed
    );
    // TODO: do not use passthrough
    c_accessor_passthrough!(
        /// The lengths along the path in the setup pose from the start of the path to the end of
        /// each Bezier curve.
        lengths,
        lengths,
        *mut c_float
    );
    c_ptr!(c_path_attachment, spPathAttachment);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl PathAttachment {
    c_vertex_attachment_accessors_mint!();
}
