use crate::{c::spRegionAttachment, sync_ptr::SyncPtr, texture_region::TextureRegion};

#[derive(Debug)]
pub struct RegionAttachment {
    c_region_attachment: SyncPtr<spRegionAttachment>,
}

impl RegionAttachment {
    pub fn new_from_ptr(c_region_attachment: *const spRegionAttachment) -> Self {
        Self {
            c_region_attachment: SyncPtr(c_region_attachment as *mut spRegionAttachment),
        }
    }

    c_ptr!(c_region_attachment, spRegionAttachment);
    c_attachment_accessors!(c_region_attachment);
    c_accessor_string!(path, path);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(scale_x, scale_x_mut, scaleX, f32);
    c_accessor!(scale_y, scale_y_mut, scaleY, f32);
    c_accessor!(rotation, rotation_mut, rotation, f32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);
    c_accessor_color!(color, color_mut, color);
    c_accessor_void_ptr!(renderer_object, renderer_object_mut, rendererObject);
    c_accessor_tmp_ptr!(region, region_mut, region, TextureRegion);

    // TODO: sequence, offset, uvs
}
