use crate::{c::spTextureRegion, sync_ptr::SyncPtr};

#[derive(Debug)]
pub struct TextureRegion {
    c_texture_region: SyncPtr<spTextureRegion>,
}

impl TextureRegion {
    pub(crate) fn new_from_ptr(c_texture_region: *mut spTextureRegion) -> Self {
        Self {
            c_texture_region: SyncPtr(c_texture_region),
        }
    }

    c_ptr!(c_texture_region, spTextureRegion);
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
    c_accessor_void_ptr!(renderer_object, rendererObject);
}
