use crate::{
    c::spTextureRegion,
    c_interface::{NewFromPtr, SyncPtr},
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// A region of a texture, usually imported from an [`AtlasRegion`](`crate::atlas::AtlasRegion`).
#[derive(Debug)]
pub struct TextureRegion {
    c_texture_region: SyncPtr<spTextureRegion>,
}

impl NewFromPtr<spTextureRegion> for TextureRegion {
    unsafe fn new_from_ptr(c_texture_region: *mut spTextureRegion) -> Self {
        Self {
            c_texture_region: SyncPtr(c_texture_region),
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

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl TextureRegion {
    #[must_use]
    pub fn uvs(&self) -> (Vector2<f32>, Vector2<f32>) {
        (
            Vector2 {
                x: self.u(),
                y: self.v(),
            },
            Vector2 {
                x: self.u2(),
                y: self.v2(),
            },
        )
    }

    #[must_use]
    pub fn offset(&self) -> Vector2<f32> {
        Vector2 {
            x: self.offset_x(),
            y: self.offset_y(),
        }
    }

    #[must_use]
    pub fn size(&self) -> Vector2<i32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }

    #[must_use]
    pub fn original_size(&self) -> Vector2<i32> {
        Vector2 {
            x: self.original_width(),
            y: self.original_height(),
        }
    }
}
