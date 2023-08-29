use crate::{
    bone::Bone,
    c::{
        spAttachment, spPointAttachment, spPointAttachment_computeWorldPosition,
        spPointAttachment_computeWorldRotation,
    },
    c_interface::{NewFromPtr, SyncPtr},
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// A lightweight, single point attachment with a position and rotation.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#PointAttachment)
#[derive(Debug)]
pub struct PointAttachment {
    c_point_attachment: SyncPtr<spPointAttachment>,
}

impl NewFromPtr<spPointAttachment> for PointAttachment {
    unsafe fn new_from_ptr(c_point_attachment: *mut spPointAttachment) -> Self {
        Self {
            c_point_attachment: SyncPtr(c_point_attachment),
        }
    }
}

impl PointAttachment {
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    #[must_use]
    pub fn compute_world_position(&self, bone: &Bone) -> (f32, f32) {
        let mut x = 0.;
        let mut y = 0.;
        unsafe {
            spPointAttachment_computeWorldPosition(self.c_ptr(), bone.c_ptr(), &mut x, &mut y);
        }
        (x, y)
    }

    #[must_use]
    pub fn compute_world_rotation(&self, bone: &Bone) -> f32 {
        unsafe { spPointAttachment_computeWorldRotation(self.c_ptr(), bone.c_ptr()) }
    }

    c_attachment_accessors!();
    c_accessor_color!(
        /// The color of the point attachment as it was in Spine, or a default color if nonessential
        /// data was not exported. Point attachments are not usually rendered at runtime.
        color,
        color
    );
    c_accessor_mut!(rotation, set_rotation, rotation, f32);
    c_accessor_mut!(x, set_x, x, f32);
    c_accessor_mut!(y, set_y, x, f32);
    c_ptr!(c_point_attachment, spPointAttachment);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl PointAttachment {
    #[must_use]
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }
}
