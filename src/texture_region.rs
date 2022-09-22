use crate::{
    c::spTextureRegion,
    c_interface::{NewFromPtr, SyncPtr},
};

#[derive(Debug)]
pub struct TextureRegion {
    c_texture_region: SyncPtr<spTextureRegion>,
}

impl NewFromPtr<spTextureRegion> for TextureRegion {
    unsafe fn new_from_ptr(c_texture_region: *const spTextureRegion) -> Self {
        Self {
            c_texture_region: SyncPtr(c_texture_region as *mut spTextureRegion),
        }
    }
}

impl TextureRegion {
    c_accessor!(u, u, f32);
    c_accessor!(v, v, f32);
    c_accessor!(u2, u2, f32);
    c_accessor!(v2, v2, f32);
    c_accessor!(degrees, degrees, i32);
    c_accessor!(offset_x, offsetX, f32);
    c_accessor!(offset_y, offsetY, f32);
    c_accessor!(width, width, i32);
    c_accessor!(height, height, i32);
    c_accessor!(original_width, originalWidth, i32);
    c_accessor!(original_height, originalHeight, i32);
    c_accessor_renderer_object!();
    c_ptr!(c_texture_region, spTextureRegion);
}
