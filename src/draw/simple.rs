use crate::{
    c::{c_void, spMeshAttachment_updateUVs},
    BlendMode, Color, Skeleton, SkeletonClipping,
};

use super::{ColorSpace, CullDirection};

#[allow(unused_imports)]
use crate::extension;

/// Renderables generated from [`SimpleDrawer::draw`].
#[derive(Clone)]
pub struct SimpleRenderable {
    /// The index of the slot in [`Skeleton`] that this renderable represents.
    pub slot_index: usize,
    /// A list of vertex attributes for a mesh.
    pub vertices: Vec<[f32; 2]>,
    /// A list of UV attributes for a mesh.
    pub uvs: Vec<[f32; 2]>,
    /// A list of indices for a mesh.
    pub indices: Vec<u16>,
    /// The color tint of the mesh.
    pub color: Color,
    /// The dark color tint of the mesh.
    /// See the [Spine User Guide](http://en.esotericsoftware.com/spine-slots#Tint-black).
    pub dark_color: Color,
    /// The blend mode to use when drawing this mesh.
    pub blend_mode: BlendMode,
    /// The attachment's renderer object as a raw pointer. Usually represents the texture created
    /// from [`extension::set_create_texture_cb`].
    pub attachment_renderer_object: Option<*const c_void>,
}

/// A simple drawer with no optimizations.
///
/// Assumes use of the default atlas attachment loader.
///
/// See [`SimpleDrawer::draw`]
pub struct SimpleDrawer {
    /// The cull direction to use for the vertices.
    pub cull_direction: CullDirection,
    /// Set to `true` if the textures are expected to have premultiplied alpha.
    pub premultiplied_alpha: bool,
    /// The color space to use for the colors returned in [`SimpleRenderable`].
    pub color_space: ColorSpace,
}

impl SimpleDrawer {
    /// This function returns a list of [`SimpleRenderable`] structs containing all the necessary
    /// data to create and render meshes. One renderable is created for each visible attachment on
    /// the skeleton. If a [`SkeletonClipping`] is provided, meshes will be properly clipped. The
    /// renderables are expected to be rendered in the order provided with the first renderable
    /// being drawn behind all the others.
    ///
    /// # Panics
    ///
    /// Panics if not using the default attachment loader with valid atlas regions.
    pub fn draw(
        &self,
        skeleton: &mut Skeleton,
        mut clipper: Option<&mut SkeletonClipping>,
    ) -> Vec<SimpleRenderable> {
        let mut renderables = vec![];
        let mut world_vertices = vec![];
        world_vertices.resize(1000, 0.);
        for slot_index in 0..skeleton.slots_count() {
            let Some(slot) = skeleton.draw_order_at_index(slot_index) else {
                continue;
            };
            if !slot.bone().active() {
                if let Some(clipper) = clipper.as_deref_mut() {
                    clipper.clip_end(&slot);
                }
                continue;
            }

            let mut vertices = vec![];
            let mut indices = vec![];
            let mut uvs = vec![];
            let mut color;

            if let Some(mesh_attachment) = slot.attachment().and_then(|a| a.as_mesh()) {
                unsafe {
                    spMeshAttachment_updateUVs(mesh_attachment.c_ptr());
                };
                color = mesh_attachment.color();

                unsafe {
                    mesh_attachment.compute_world_vertices(
                        &slot,
                        0,
                        mesh_attachment.world_vertices_length(),
                        &mut world_vertices,
                        0,
                        2,
                    );
                }

                vertices.reserve(mesh_attachment.world_vertices_length() as usize);
                uvs.reserve(mesh_attachment.world_vertices_length() as usize);
                uvs.resize(mesh_attachment.world_vertices_length() as usize, [0., 0.]);
                for i in 0..mesh_attachment.world_vertices_length() {
                    vertices.push([
                        world_vertices[i as usize * 2],
                        world_vertices[i as usize * 2 + 1],
                    ]);
                }

                // UVs need to be copied from the indices. I'm not entirely sure why, but it can lead to crashes otherwise.
                macro_rules! copy_uvs {
                    ($i:ident) => {
                        let index = *mesh_attachment.triangles().offset($i);
                        uvs[index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                        let index = *mesh_attachment.triangles().offset($i + 1);
                        uvs[index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                        let index = *mesh_attachment.triangles().offset($i + 2);
                        uvs[index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                    };
                }

                indices.reserve(mesh_attachment.triangles_count() as usize);
                if matches!(self.cull_direction, CullDirection::CounterClockwise) {
                    for i in (0..mesh_attachment.triangles_count() as isize).step_by(3) {
                        unsafe {
                            indices.push(*mesh_attachment.triangles().offset(i + 2));
                            indices.push(*mesh_attachment.triangles().offset(i + 1));
                            indices.push(*mesh_attachment.triangles().offset(i));
                            copy_uvs!(i);
                        }
                    }
                } else {
                    for i in (0..mesh_attachment.triangles_count() as isize).step_by(3) {
                        unsafe {
                            indices.push(*mesh_attachment.triangles().offset(i));
                            indices.push(*mesh_attachment.triangles().offset(i + 1));
                            indices.push(*mesh_attachment.triangles().offset(i + 2));
                            copy_uvs!(i);
                        }
                    }
                }
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                color = region_attachment.color();

                let mut world_vertices = vec![];
                world_vertices.resize(1000, 0.);
                unsafe {
                    region_attachment.compute_world_vertices(
                        &slot.bone(),
                        &mut world_vertices,
                        0,
                        2,
                    );
                }

                vertices.reserve(4);
                uvs.reserve(4);
                for i in 0..4 {
                    vertices.push([
                        world_vertices[i as usize * 2],
                        world_vertices[i as usize * 2 + 1],
                    ]);

                    uvs.push([
                        region_attachment.uvs()[i as usize * 2],
                        region_attachment.uvs()[i as usize * 2 + 1],
                    ]);
                }

                indices.reserve(6);
                if matches!(self.cull_direction, CullDirection::CounterClockwise) {
                    indices.push(2);
                    indices.push(1);
                    indices.push(0);
                    indices.push(0);
                    indices.push(3);
                    indices.push(2);
                } else {
                    indices.push(0);
                    indices.push(1);
                    indices.push(2);
                    indices.push(2);
                    indices.push(3);
                    indices.push(0);
                }
            } else if let Some(clipping_attachment) =
                slot.attachment().and_then(|a| a.as_clipping())
            {
                if let Some(clipper) = clipper.as_deref_mut() {
                    clipper.clip_start(&slot, &clipping_attachment);
                }
                continue;
            } else {
                if let Some(clipper) = clipper.as_deref_mut() {
                    clipper.clip_end(&slot);
                }
                continue;
            }

            if let Some(clipper) = clipper.as_deref_mut() {
                if clipper.is_clipping() {
                    unsafe {
                        clipper.clip_triangles(
                            vertices.as_mut_slice(),
                            indices.as_mut_slice(),
                            uvs.as_mut_slice(),
                            2,
                        );
                        let clipped_vertices_size =
                            (*clipper.c_ptr_ref().clippedVertices).size as usize;
                        vertices.resize(clipped_vertices_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedVertices).items,
                            vertices.as_mut_ptr().cast::<f32>(),
                            clipped_vertices_size,
                        );
                        let clipped_triangles_size =
                            (*clipper.c_ptr_ref().clippedTriangles).size as usize;
                        indices.resize(clipped_triangles_size, 0);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedTriangles).items,
                            indices.as_mut_ptr(),
                            clipped_triangles_size,
                        );
                        let clipped_uvs_size = (*clipper.c_ptr_ref().clippedUVs).size as usize;
                        uvs.resize(clipped_uvs_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedUVs).items,
                            uvs.as_mut_ptr().cast::<f32>(),
                            clipped_uvs_size,
                        );
                    }
                }
            }

            let attachment_renderer_object =
                slot.attachment().and_then(|a| a.as_mesh()).map_or_else(
                    || {
                        slot.attachment().and_then(|a| a.as_region()).and_then(
                            |region_attachment| unsafe {
                                let attachment_renderer_object = region_attachment
                                    .renderer_object()
                                    .get_atlas_region()
                                    .unwrap()
                                    .page()
                                    .c_ptr_ref()
                                    .rendererObject
                                    .cast_const();
                                if attachment_renderer_object.is_null() {
                                    None
                                } else {
                                    Some(attachment_renderer_object)
                                }
                            },
                        )
                    },
                    |mesh_attachment| unsafe {
                        let attachment_renderer_object = mesh_attachment
                            .renderer_object()
                            .get_atlas_region()
                            .unwrap()
                            .page()
                            .c_ptr_ref()
                            .rendererObject
                            .cast_const();
                        if attachment_renderer_object.is_null() {
                            None
                        } else {
                            Some(attachment_renderer_object)
                        }
                    },
                );

            color *= slot.color() * skeleton.color();
            let mut dark_color = slot.dark_color().unwrap_or_default();
            if self.premultiplied_alpha {
                color.premultiply_alpha();
                dark_color *= color.a;
                dark_color.a = 1.0;
            } else {
                dark_color.a = 0.;
            }
            color = match self.color_space {
                ColorSpace::SRGB => color,
                ColorSpace::Linear => color.nonlinear_to_linear(),
            };

            dark_color = match self.color_space {
                ColorSpace::SRGB => dark_color,
                ColorSpace::Linear => dark_color.nonlinear_to_linear(),
            };

            renderables.push(SimpleRenderable {
                slot_index,
                vertices,
                uvs,
                indices,
                color,
                dark_color,
                blend_mode: slot.data().blend_mode(),
                attachment_renderer_object,
            });
            if let Some(clipper) = clipper.as_deref_mut() {
                clipper.clip_end(&slot);
            }
        }

        if let Some(clipper) = clipper {
            clipper.clip_end2();
        }
        renderables
    }
}

#[cfg(test)]
mod test {
    use crate::test::TestAsset;

    use super::*;

    /// Ensure all the example assets draw without error.
    #[test]
    fn simple_drawer() {
        for json in [true, false] {
            for example_asset in TestAsset::all() {
                let (mut skeleton, _) = example_asset.instance(json);
                let drawer = SimpleDrawer {
                    cull_direction: CullDirection::Clockwise,
                    premultiplied_alpha: false,
                    color_space: ColorSpace::Linear,
                };
                let mut clipper = SkeletonClipping::new();
                let renderables = drawer.draw(&mut skeleton, Some(&mut clipper));
                assert!(!renderables.is_empty());
            }
        }
    }
}
