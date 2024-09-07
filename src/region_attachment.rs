use crate::{
    c::{c_float, spAttachment, spRegionAttachment, spRegionAttachment_computeWorldVertices},
    c_interface::SyncPtr,
    Bone, Color,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// Properties for updating [`RegionAttachment`].
#[derive(Debug)]
pub struct RegionProps {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

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

    pub unsafe fn compute_world_vertices(
        &self,
        bone: &Bone,
        vertices: &mut [f32],
        offset: i32,
        stride: i32,
    ) {
        spRegionAttachment_computeWorldVertices(
            self.c_ptr() as *const spRegionAttachment as *mut spRegionAttachment,
            bone.c_ptr(),
            vertices.as_mut_ptr(),
            offset,
            stride,
        );
    }

    pub fn update_from_props(&mut self, props: &RegionProps) {
        self.set_x(props.x);
        self.set_y(props.y);
        self.set_scale_x(props.scale_x);
        self.set_scale_y(props.scale_y);
        self.set_rotation(props.rotation);
        self.set_width(props.width);
        self.set_height(props.height);
        *self.color_mut() = props.color;
    }

    c_attachment_accessors!();
    c_accessor_string_mut!(path, set_path, path);
    c_accessor_mut!(
        /// The local x translation.
        x,
        set_x,
        x,
        f32
    );
    c_accessor_mut!(
        /// The local y translation.
        y,
        set_y,
        y,
        f32
    );
    c_accessor_mut!(
        /// The local scaleX.
        scale_x,
        set_scale_x,
        scaleX,
        f32
    );
    c_accessor_mut!(
        /// The local scaleY.
        scale_y,
        set_scale_y,
        scaleY,
        f32
    );
    // TODO: docs: in degrees? counter-clockwise?
    c_accessor_mut!(
        /// The local rotation.
        rotation,
        set_rotation,
        rotation,
        f32
    );
    c_accessor_mut!(
        /// The width of the region attachment in Spine.
        width,
        set_width,
        width,
        f32
    );
    c_accessor_mut!(
        /// The height of the region attachment in Spine.
        height,
        set_height,
        height,
        f32
    );
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_passthrough!(uvs, uvs, [c_float; 8]);
    c_accessor_passthrough!(offset, offset, [c_float; 8]);
    c_accessor_renderer_object!();
    c_ptr!(c_region_attachment, spRegionAttachment);

    // TODO: sequence accessor
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl RegionAttachment {
    #[must_use]
    pub fn translation(&self) -> Vector2<f32> {
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
