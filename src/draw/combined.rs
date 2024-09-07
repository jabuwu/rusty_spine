use crate::{c::c_void, BlendMode, Skeleton, SkeletonClipping};

use super::{ColorSpace, CullDirection};

#[allow(unused_imports)]
use crate::{draw::SimpleDrawer, extension};

/// Renderables generated from [`CombinedDrawer::draw`].
pub struct CombinedRenderable {
    /// A list of vertex attributes for a mesh.
    pub vertices: Vec<[f32; 2]>,
    /// A list of UV attributes for a mesh.
    pub uvs: Vec<[f32; 2]>,
    /// A list of color attributes for a mesh.
    pub colors: Vec<[f32; 4]>,
    /// A list of dark color attributes for a mesh.
    /// See the [Spine User Guide](http://en.esotericsoftware.com/spine-slots#Tint-black).
    pub dark_colors: Vec<[f32; 4]>,
    /// A list of indices for a mesh.
    pub indices: Vec<u16>,
    /// The blend mode to use when drawing this mesh.
    pub blend_mode: BlendMode,
    /// The attachment's renderer object as a raw pointer. Usually represents the texture created
    /// from [`extension::set_create_texture_cb`].
    pub attachment_renderer_object: Option<*const c_void>,
}

/// A combined drawer with a mesh combining optimization.
///
/// Assumes use of the default atlas attachment loader.
///
/// See [`CombinedDrawer::draw`]
pub struct CombinedDrawer {
    pub cull_direction: CullDirection,
    pub premultiplied_alpha: bool,
    pub color_space: ColorSpace,
}

impl CombinedDrawer {
    /// This function returns a list of [`CombinedRenderable`] structs containing all the necessary
    /// data to create and render meshes. Attachments are batched together into a single renderable
    /// so long as their blend mode or renderer object is not different from the previous
    /// attachment. If a [`SkeletonClipping`] is provided, meshes will be properly clipped. The
    /// renderables are expected to be rendered in the order provided with the first renderable
    /// being drawn behind all the others.
    ///
    /// This drawer can provide a significant performance advantage over the [`SimpleDrawer`] in
    /// most cases.
    ///
    /// # Panics
    ///
    /// Panics if not using the default attachment loader with valid atlas regions.
    pub fn draw(
        &self,
        skeleton: &mut Skeleton,
        mut clipper: Option<&mut SkeletonClipping>,
    ) -> Vec<CombinedRenderable> {
        let mut renderables = vec![];
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];
        let mut dark_colors = vec![];
        let mut indices = vec![];
        let mut blend_mode = BlendMode::Normal;
        let mut attachment_renderer_object = None;
        let mut world_vertices = vec![];
        world_vertices.resize(1000, 0.);
        let mut vertex_base: u16 = 0;
        let mut index_base: u16 = 0;
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

            if let Some(clipping_attachment) = slot.attachment().and_then(|a| a.as_clipping()) {
                if let Some(clipper) = clipper.as_deref_mut() {
                    clipper.clip_start(&slot, &clipping_attachment);
                }
                continue;
            } else if slot.attachment().and_then(|a| a.as_mesh()).is_none()
                && slot.attachment().and_then(|a| a.as_region()).is_none()
            {
                if let Some(clipper) = clipper.as_deref_mut() {
                    clipper.clip_end(&slot);
                }
                continue;
            }

            if let Some(mesh_attachment) = slot.attachment().and_then(|a| a.as_mesh()) {
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
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                unsafe {
                    region_attachment.compute_world_vertices(
                        &slot.bone(),
                        &mut world_vertices,
                        0,
                        2,
                    );
                }
            }

            let next_blend_mode = slot.data().blend_mode();
            let next_attachment_renderer_object =
                slot.attachment().and_then(|a| a.as_mesh()).map_or_else(
                    || {
                        slot.attachment().and_then(|a| a.as_region()).map_or_else(
                            || {
                                unreachable!();
                            },
                            |region_attachment| {
                                let next_attachment_renderer_object = unsafe {
                                    region_attachment
                                        .renderer_object()
                                        .get_atlas_region()
                                        .unwrap()
                                        .page()
                                        .c_ptr_ref()
                                        .rendererObject
                                        .cast_const()
                                };
                                if next_attachment_renderer_object.is_null() {
                                    None
                                } else {
                                    Some(next_attachment_renderer_object)
                                }
                            },
                        )
                    },
                    |mesh_attachment| {
                        let next_attachment_renderer_object = unsafe {
                            mesh_attachment
                                .renderer_object()
                                .get_atlas_region()
                                .unwrap()
                                .page()
                                .c_ptr_ref()
                                .rendererObject
                                .cast_const()
                        };
                        if next_attachment_renderer_object.is_null() {
                            None
                        } else {
                            Some(next_attachment_renderer_object)
                        }
                    },
                );

            if slot_index == 0 {
                blend_mode = next_blend_mode;
                attachment_renderer_object = next_attachment_renderer_object;
            }
            if blend_mode != next_blend_mode
                || attachment_renderer_object != next_attachment_renderer_object
            {
                renderables.push(CombinedRenderable {
                    vertices,
                    uvs,
                    indices,
                    colors,
                    dark_colors,
                    blend_mode,
                    attachment_renderer_object,
                });
                vertices = vec![];
                uvs = vec![];
                colors = vec![];
                dark_colors = vec![];
                indices = vec![];
                vertex_base = 0;
                index_base = 0;
            }
            blend_mode = next_blend_mode;
            attachment_renderer_object = next_attachment_renderer_object;

            let (color, dark_color) = if let Some(mesh_attachment) =
                slot.attachment().and_then(|a| a.as_mesh())
            {
                let mut color = mesh_attachment.color() * slot.color() * skeleton.color();
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

                uvs.resize(
                    vertex_base as usize + mesh_attachment.world_vertices_length() as usize,
                    [0., 0.],
                );
                for i in 0..mesh_attachment.world_vertices_length() {
                    vertices.push([
                        world_vertices[i as usize * 2],
                        world_vertices[i as usize * 2 + 1],
                    ]);

                    colors.push([color.r, color.g, color.b, color.a]);
                    dark_colors.push([dark_color.r, dark_color.g, dark_color.b, dark_color.a]);
                }

                // UVs need to be copied from the indices. I'm not entirely sure why, but it can lead to crashes otherwise.
                macro_rules! copy_uvs {
                    ($i:ident) => {
                        let index = *mesh_attachment.triangles().offset($i);
                        uvs[vertex_base as usize + index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                        let index = *mesh_attachment.triangles().offset($i + 1);
                        uvs[vertex_base as usize + index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                        let index = *mesh_attachment.triangles().offset($i + 2);
                        uvs[vertex_base as usize + index as usize] = [
                            *mesh_attachment.c_ptr_mut().uvs.offset(index as isize * 2),
                            *mesh_attachment
                                .c_ptr_mut()
                                .uvs
                                .offset(index as isize * 2 + 1),
                        ];
                    };
                }

                if matches!(self.cull_direction, CullDirection::CounterClockwise) {
                    for i in (0..mesh_attachment.triangles_count() as isize).step_by(3) {
                        unsafe {
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i + 2));
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i + 1));
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i));
                            copy_uvs!(i);
                        }
                    }
                } else {
                    for i in (0..mesh_attachment.triangles_count() as isize).step_by(3) {
                        unsafe {
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i));
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i + 1));
                            indices.push(vertex_base + *mesh_attachment.triangles().offset(i + 2));
                            copy_uvs!(i);
                        }
                    }
                }

                (color, dark_color)
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                let mut color = region_attachment.color() * slot.color() * skeleton.color();
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

                for i in 0..4 {
                    vertices.push([
                        world_vertices[i as usize * 2],
                        world_vertices[i as usize * 2 + 1],
                    ]);

                    uvs.push([
                        region_attachment.uvs()[i as usize * 2],
                        region_attachment.uvs()[i as usize * 2 + 1],
                    ]);

                    colors.push([color.r, color.g, color.b, color.a]);
                    dark_colors.push([dark_color.r, dark_color.g, dark_color.b, dark_color.a]);
                }

                if matches!(self.cull_direction, CullDirection::CounterClockwise) {
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 1);
                    indices.push(vertex_base);
                    indices.push(vertex_base);
                    indices.push(vertex_base + 3);
                    indices.push(vertex_base + 2);
                } else {
                    indices.push(vertex_base);
                    indices.push(vertex_base + 1);
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 3);
                    indices.push(vertex_base);
                }

                (color, dark_color)
            } else {
                unreachable!();
            };

            if let Some(clipper) = clipper.as_deref_mut() {
                if clipper.is_clipping() {
                    for i in index_base..indices.len() as u16 {
                        indices[i as usize] -= vertex_base;
                    }
                    unsafe {
                        clipper.clip_triangles(
                            &mut vertices.as_mut_slice()[(vertex_base as usize)..],
                            &mut indices.as_mut_slice()[(index_base as usize)..],
                            &mut uvs.as_mut_slice()[(vertex_base as usize)..],
                            2,
                        );
                        let clipped_triangles_size =
                            (*clipper.c_ptr_ref().clippedTriangles).size as usize;
                        let clipped_vertices_size =
                            (*clipper.c_ptr_ref().clippedVertices).size as usize;
                        let clipped_uvs_size = (*clipper.c_ptr_ref().clippedUVs).size as usize;
                        colors.resize(
                            vertex_base as usize + (clipped_vertices_size / 2),
                            [color.r, color.g, color.b, color.a],
                        );
                        dark_colors.resize(
                            vertex_base as usize + (clipped_vertices_size / 2),
                            [dark_color.r, dark_color.g, dark_color.b, dark_color.a],
                        );
                        indices.resize(index_base as usize + clipped_triangles_size, 0);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedTriangles).items,
                            indices.as_mut_ptr().offset(index_base as isize),
                            clipped_triangles_size,
                        );
                        vertices.resize(vertex_base as usize + clipped_vertices_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedVertices).items,
                            vertices
                                .as_mut_ptr()
                                .offset(vertex_base as isize)
                                .cast::<f32>(),
                            clipped_vertices_size,
                        );
                        uvs.resize(vertex_base as usize + clipped_uvs_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedUVs).items,
                            uvs.as_mut_ptr().offset(vertex_base as isize).cast::<f32>(),
                            clipped_uvs_size,
                        );
                    }
                    for i in index_base..indices.len() as u16 {
                        indices[i as usize] += vertex_base;
                    }
                }
            }

            vertex_base = vertices.len() as u16;
            index_base = indices.len() as u16;

            if let Some(clipper) = clipper.as_deref_mut() {
                clipper.clip_end(&slot);
            }
        }

        if !indices.is_empty() {
            renderables.push(CombinedRenderable {
                vertices,
                uvs,
                indices,
                colors,
                dark_colors,
                blend_mode,
                attachment_renderer_object,
            });
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
    fn combined_drawer() {
        for json in [true, false] {
            for example_asset in TestAsset::all() {
                let (mut skeleton, _) = example_asset.instance(json);
                let drawer = CombinedDrawer {
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
