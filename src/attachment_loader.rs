use crate::{
    c::{
        spAtlasAttachmentLoader_create, spAttachmentLoader, spAttachmentLoader_createAttachment,
        spAttachmentLoader_dispose,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Atlas, Attachment, AttachmentType, RegionProps, Skin, SpineError,
};

/// A loader for creating custom attachments.
///
/// Currently only supports [`Atlas`](`crate::Atlas`) based attachments.
#[derive(Debug)]
pub struct AttachmentLoader {
    c_attachment_loader: SyncPtr<spAttachmentLoader>,
}

impl NewFromPtr<spAttachmentLoader> for AttachmentLoader {
    unsafe fn new_from_ptr(c_attachment_loader: *mut spAttachmentLoader) -> Self {
        Self {
            c_attachment_loader: SyncPtr(c_attachment_loader),
        }
    }
}

impl AttachmentLoader {
    /// The spine runtime offers a default [`AttachmentLoader`](`crate::AttachmentLoader`) that
    /// loads attachments from an [`Atlas`](`crate::Atlas`).
    #[must_use]
    pub fn new_atlas_loader(atlas: &Atlas) -> Self {
        unsafe {
            let atlas_attachment_loader = spAtlasAttachmentLoader_create(atlas.c_ptr());
            let attachment_loader = &mut (*atlas_attachment_loader).super_0;
            Self::new_from_ptr(attachment_loader)
        }
    }

    /// Creates an [`Attachment`](`crate::Attachment`) of a specified type.
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::CreationFailed`] if creating the attachment failed.
    /// Check [`error1`](`Self::error1`) and [`error2`](`Self::error2`) for more information.
    /// Returns [`SpineError::NulError`] if `name` or `path` contain a null byte.
    pub fn create_attachment(
        &self,
        skin: Option<Skin>,
        attachment_type: AttachmentType,
        name: &str,
        path: &str,
    ) -> Result<Attachment, SpineError> {
        let c_name = std::ffi::CString::new(name)?;
        let c_path = std::ffi::CString::new(path)?;

        unsafe {
            let c_name = c_name.as_ptr();
            let c_path = c_path.as_ptr();
            let c_skin = skin.map_or(std::ptr::null_mut(), |skin| skin.c_ptr());

            let attachment = spAttachmentLoader_createAttachment(
                self.c_ptr(),
                c_skin,
                attachment_type as u32,
                c_name,
                c_path,
            );

            if attachment.is_null() {
                Err(SpineError::new_creation_failed("Attachment"))
            } else {
                Ok(Attachment::new_from_ptr(attachment))
            }
        }
    }

    /// Convenience function for creating a [`RegionAttachment`](`crate::RegionAttachment`).
    ///
    /// # Errors
    ///
    /// Returns [`SpineError::CreationFailed`] if creating the attachment failed.
    /// Check [`error1`](`Self::error1`) and [`error2`](`Self::error2`) for more information.
    /// Returns [`SpineError::NulError`] if `name` or `path` contain a null byte.
    pub fn create_region_attachment(
        &self,
        skin: Option<Skin>,
        name: &str,
        path: &str,
        props: &RegionProps,
    ) -> Result<Attachment, SpineError> {
        let attachment = self.create_attachment(skin, AttachmentType::Region, name, path)?;

        let Some(mut region) = attachment.as_region() else {
            return Err(SpineError::new_creation_failed("RegionAttachment"));
        };

        region.update_from_props(props);

        Ok(attachment)
    }

    c_accessor_string!(error1, error1);
    c_accessor_string!(error2, error2);
    c_ptr!(c_attachment_loader, spAttachmentLoader);
}

impl Drop for AttachmentLoader {
    fn drop(&mut self) {
        unsafe {
            spAttachmentLoader_dispose(self.c_ptr());
        }
    }
}
