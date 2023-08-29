use crate::{
    bounding_box_attachment::BoundingBoxAttachment,
    c::{
        spAttachment, spAttachmentType, spAttachment_dispose, spBoundingBoxAttachment,
        spClippingAttachment, spMeshAttachment, spPointAttachment, spRegionAttachment,
    },
    c_interface::{NewFromPtr, SyncPtr},
    clipping_attachment::ClippingAttachment,
    mesh_attachment::MeshAttachment,
    point_attachment::PointAttachment,
    region_attachment::RegionAttachment,
};

/// Slot attachments.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Attachment)
///
/// Attachments are reference counted and can be stored and worked with directly, however, some of
/// the underlying data that attachments point to may be cleared. For this reason, attachments
/// should be used with caution, and only used so long as the
/// [`SkeletonData`](`crate::SkeletonData`) they came from remains valid, and only attached to the
/// slot they are meant for.
#[derive(Debug)]
pub struct Attachment {
    c_attachment: SyncPtr<spAttachment>,
}

impl NewFromPtr<spAttachment> for Attachment {
    unsafe fn new_from_ptr(c_attachment: *mut spAttachment) -> Self {
        (*c_attachment).refCount += 1;
        Self {
            c_attachment: SyncPtr(c_attachment),
        }
    }
}

impl Attachment {
    /// Get this attachment as a [`RegionAttachment`], or [`None`] if it's a different type.
    #[must_use]
    pub fn as_region(&self) -> Option<RegionAttachment> {
        if self.attachment_type() == AttachmentType::Region {
            Some(RegionAttachment::new_from_ptr(
                self.c_attachment.0.cast::<spRegionAttachment>(),
            ))
        } else {
            None
        }
    }

    /// Get this attachment as a [`BoundingBoxAttachment`], or [`None`] if it's a different type.
    #[must_use]
    pub fn as_bounding_box(&self) -> Option<BoundingBoxAttachment> {
        if self.attachment_type() == AttachmentType::BoundingBox {
            Some(unsafe {
                BoundingBoxAttachment::new_from_ptr(
                    self.c_attachment.0.cast::<spBoundingBoxAttachment>(),
                )
            })
        } else {
            None
        }
    }

    /// Get this attachment as a [`MeshAttachment`], or [`None`] if it's a different type.
    #[must_use]
    pub fn as_mesh(&self) -> Option<MeshAttachment> {
        if self.attachment_type() == AttachmentType::Mesh {
            Some(unsafe {
                MeshAttachment::new_from_ptr(self.c_attachment.0.cast::<spMeshAttachment>())
            })
        } else {
            None
        }
    }

    /// Get this attachment as a [`PointAttachment`], or [`None`] if it's a different type.
    #[must_use]
    pub fn as_point(&self) -> Option<PointAttachment> {
        if self.attachment_type() == AttachmentType::Point {
            Some(unsafe {
                PointAttachment::new_from_ptr(self.c_attachment.0.cast::<spPointAttachment>())
            })
        } else {
            None
        }
    }

    /// Get this attachment as a [`ClippingAttachment`], or [`None`] if it's a different type.
    #[must_use]
    pub fn as_clipping(&self) -> Option<ClippingAttachment> {
        if self.attachment_type() == AttachmentType::Clipping {
            Some(unsafe {
                ClippingAttachment::new_from_ptr(self.c_attachment.0.cast::<spClippingAttachment>())
            })
        } else {
            None
        }
    }

    c_accessor_string!(
        /// The attachment's name.
        name,
        name
    );
    c_accessor_enum!(
        /// The attachment's type.
        attachment_type,
        type_0,
        AttachmentType
    );
    c_ptr!(c_attachment, spAttachment);
}

impl Clone for Attachment {
    fn clone(&self) -> Self {
        unsafe { Attachment::new_from_ptr(self.c_ptr()) }
    }
}

impl Drop for Attachment {
    fn drop(&mut self) {
        unsafe {
            spAttachment_dispose(self.c_ptr());
        }
    }
}

/// The type variants of an [`Attachment`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentType {
    Region = 0,
    BoundingBox = 1,
    Mesh = 2,
    LinkedMesh = 3,
    Path = 4,
    Point = 5,
    Clipping = 6,
    Unknown = 99,
}

impl From<spAttachmentType> for AttachmentType {
    fn from(attachment_type: spAttachmentType) -> Self {
        match attachment_type {
            0 => Self::Region,
            1 => Self::BoundingBox,
            2 => Self::Mesh,
            3 => Self::LinkedMesh,
            4 => Self::Path,
            5 => Self::Point,
            6 => Self::Clipping,
            _ => Self::Unknown,
        }
    }
}
