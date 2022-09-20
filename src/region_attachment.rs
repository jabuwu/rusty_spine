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

    c_attachment_accessors!();
    c_accessor_string!(path, path);
    c_accessor!(x, set_x, x, f32);
    c_accessor!(y, set_y, y, f32);
    c_accessor!(scale_x, set_scale_x, scaleX, f32);
    c_accessor!(scale_y, set_scale_y, scaleY, f32);
    c_accessor!(rotation, set_rotation, rotation, f32);
    c_accessor!(width, set_width, width, f32);
    c_accessor!(height, set_height, height, f32);
    c_accessor_color!(color, color_mut, color);
    c_accessor_renderer_object!();
    c_accessor_tmp_ptr!(region, region_mut, region, TextureRegion, spTextureRegion);
    c_ptr!(c_region_attachment, spRegionAttachment);

    // TODO: sequence, offset, uvs
}
