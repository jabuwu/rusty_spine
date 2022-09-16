use crate::{c::spTextureRegion, sync_ptr::SyncPtr};

#[derive(Debug)]
pub struct TextureRegion {
    c_texture_region: SyncPtr<spTextureRegion>,
}

impl TextureRegion {
    pub(crate) fn new_from_ptr(c_texture_region: *const spTextureRegion) -> Self {
        Self {
            c_texture_region: SyncPtr(c_texture_region as *mut spTextureRegion),
        }
    }

    c_ptr!(c_texture_region, spTextureRegion);
    c_accessor!(u, u_mut, u, f32);
    c_accessor!(v, v_mut, v, f32);
    c_accessor!(u2, u2_mut, u2, f32);
    c_accessor!(v2, v2_mut, v2, f32);
    c_accessor!(degrees, degrees_mut, degrees, i32);
    c_accessor!(offset_x, offset_x_mut, offsetX, f32);
    c_accessor!(offset_y, offset_y_mut, offsetY, f32);
    c_accessor!(width, width_mut, width, i32);
    c_accessor!(height, height_mut, height, i32);
    c_accessor!(original_width, original_width_mut, originalWidth, i32);
    c_accessor!(original_height, original_height_mut, originalHeight, i32);
    c_accessor_renderer_object!();
}
