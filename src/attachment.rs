use crate::{
    c::{
        spAttachment, spAttachmentType, spClippingAttachment, spMeshAttachment, spRegionAttachment,
    },
    c_interface::NewFromPtr,
    clipping_attachment::ClippingAttachment,
    mesh_attachment::MeshAttachment,
    region_attachment::RegionAttachment,
    sync_ptr::SyncPtr,
};

#[derive(Debug)]
pub struct Attachment {
    c_attachment: SyncPtr<spAttachment>,
}

impl NewFromPtr<spAttachment> for Attachment {
    unsafe fn new_from_ptr(c_attachment: *const spAttachment) -> Self {
        Self {
            c_attachment: SyncPtr(c_attachment as *mut spAttachment),
        }
    }
}

impl Attachment {
    pub fn as_region(&self) -> Option<RegionAttachment> {
        if self.attachment_type() == AttachmentType::Region {
            Some(RegionAttachment::new_from_ptr(
                self.c_attachment.0 as *mut spRegionAttachment,
            ))
        } else {
            None
        }
    }

    pub fn as_mesh(&self) -> Option<MeshAttachment> {
        if self.attachment_type() == AttachmentType::Mesh {
            Some(unsafe {
                MeshAttachment::new_from_ptr(self.c_attachment.0 as *mut spMeshAttachment)
            })
        } else {
            None
        }
    }

    pub fn as_clipping(&self) -> Option<ClippingAttachment> {
        if self.attachment_type() == AttachmentType::Clipping {
            Some(unsafe {
                ClippingAttachment::new_from_ptr(self.c_attachment.0 as *mut spClippingAttachment)
            })
        } else {
            None
        }
    }

    c_ptr!(c_attachment, spAttachment);
    c_accessor_string!(name, name);
    c_accessor_enum_no_set!(attachment_type, type_0, AttachmentType);
}

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
