use crate::{
    c::{
        spAttachment, spRegionAttachment, spRegionAttachment_computeWorldVertices, spTextureRegion,
    },
    slot::Slot,
    sync_ptr::SyncPtr,
    texture_region::TextureRegion,
};

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

    fn attachment(&self) -> &spAttachment {
        &self.c_ptr_ref().super_0
    }

    pub unsafe fn compute_world_vertices(
        &self,
        slot: &Slot,
        vertices: &mut [f32],
        offset: i32,
        stride: i32,
    ) {
        spRegionAttachment_computeWorldVertices(
            self.c_ptr() as *const spRegionAttachment as *mut spRegionAttachment,
            slot.c_ptr(),
            vertices.as_mut_ptr(),
            offset,
            stride,
        );
    }

    c_ptr!(c_region_attachment, spRegionAttachment);
    c_attachment_accessors!(self.c_ptr_ref().super_0);
    c_accessor_string!(path, path);
    c_accessor!(x, x_mut, x, f32);
    c_accessor!(y, y_mut, y, f32);
    c_accessor!(scale_x, scale_x_mut, scaleX, f32);
    c_accessor!(scale_y, scale_y_mut, scaleY, f32);
    c_accessor!(rotation, rotation_mut, rotation, f32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);
    c_accessor_color!(color, color_mut, color);
    c_accessor_renderer_object!();
    c_accessor_tmp_ptr!(region, region_mut, region, TextureRegion, spTextureRegion);

    // TODO: sequence, offset, uvs
}
