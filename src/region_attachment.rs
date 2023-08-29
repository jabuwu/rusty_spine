use crate::{
    c::{
        c_float, spAttachment, spRegionAttachment, spRegionAttachment_computeWorldVertices,
        spRegionAttachment_updateRegion, spTextureRegion,
    },
    c_interface::SyncPtr,
    slot::Slot,
    texture_region::TextureRegion,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// An attachment which draws a texture.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#RegionAttachment)
#[derive(Debug)]
pub struct RegionAttachment {
    c_region_attachment: SyncPtr<spRegionAttachment>,
}

impl RegionAttachment {
    #[must_use]
    pub const fn new_from_ptr(c_region_attachment: *mut spRegionAttachment) -> Self {
        Self {
            c_region_attachment: SyncPtr(c_region_attachment),
        }
    }

    #[must_use]
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    /// # Safety
    ///
    /// The slot passed in must be the same slot this attachment originated from.
    pub unsafe fn compute_world_vertices(
        &self,
        slot: &Slot,
        vertices: &mut [f32],
        offset: i32,
        stride: i32,
    ) {
        spRegionAttachment_computeWorldVertices(
            self.c_ptr(),
            slot.c_ptr(),
            vertices.as_mut_ptr(),
            offset,
            stride,
        );
    }

    pub fn update_region(&mut self) {
        unsafe {
            spRegionAttachment_updateRegion(self.c_ptr());
        }
    }

    c_attachment_accessors!();
    c_accessor_string!(path, path);
    c_accessor!(
        /// The local x translation.
        x,
        x,
        f32
    );
    c_accessor!(
        /// The local y translation.
        y,
        y,
        f32
    );
    c_accessor!(
        /// The local scaleX.
        scale_x,
        scaleX,
        f32
    );
    c_accessor!(
        /// The local scaleY.
        scale_y,
        scaleY,
        f32
    );
    // TODO: docs: in degrees? counter-clockwise?
    c_accessor!(
        /// The local rotation.
        rotation,
        rotation,
        f32
    );
    c_accessor!(
        /// The width of the region attachment in Spine.
        width,
        width,
        f32
    );
    c_accessor!(
        /// The height of the region attachment in Spine.
        height,
        height,
        f32
    );
    c_accessor_color!(color, color);
    c_accessor_passthrough!(uvs, uvs, [c_float; 8]);
    c_accessor_passthrough!(offset, offset, [c_float; 8]);
    c_accessor_renderer_object!();
    c_accessor_tmp_ptr_optional_mut!(region, region_mut, region, TextureRegion, spTextureRegion);
    c_ptr!(c_region_attachment, spRegionAttachment);

    // TODO: sequence accessor
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl RegionAttachment {
    #[must_use]
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    #[must_use]
    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    #[must_use]
    pub fn size(&self) -> Vector2<f32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }
}
