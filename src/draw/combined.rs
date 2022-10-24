use crate::{
    c::{c_void, spSkeletonClipping_clipTriangles},
    Color, Skeleton, SkeletonClipping,
};

use super::CullDirection;

pub struct CombinedRenderable {
    pub vertices: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub dark_colors: Vec<[f32; 4]>,
    pub indices: Vec<u16>,
    pub attachment_renderer_object: Option<*const c_void>,
}

pub struct CombinedDrawer {
    pub cull_direction: CullDirection,
    pub premultiplied_alpha: bool,
}

/// A combined drawer with a mesh combining optimization.
///
/// Assumes use of the default atlas attachment loader.
impl CombinedDrawer {
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
        let mut attachment_renderer_object = None;
        let mut world_vertices = vec![];
        world_vertices.resize(1000, 0.);
        let mut vertex_base: u16 = 0;
        let mut index_base: u16 = 0;
        for slot_index in 0..skeleton.slots_count() {
            let slot = skeleton.draw_order_at_index(slot_index).unwrap();
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
                #[cfg(feature = "spine38")]
                unsafe {
                    region_attachment.compute_world_vertices(
                        &slot.bone(),
                        &mut world_vertices,
                        0,
                        2,
                    );
                }
                #[cfg(not(feature = "spine38"))]
                unsafe {
                    region_attachment.compute_world_vertices(&slot, &mut world_vertices, 0, 2);
                }
            }

            let next_attachment_renderer_object = if let Some(mesh_attachment) =
                slot.attachment().and_then(|a| a.as_mesh())
            {
                let next_attachment_renderer_object = Some(unsafe {
                    mesh_attachment
                        .renderer_object()
                        .get_atlas_region()
                        .unwrap()
                        .page()
                        .c_ptr_ref()
                        .rendererObject as *const c_void
                });
                next_attachment_renderer_object
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                let next_attachment_renderer_object = Some(unsafe {
                    region_attachment
                        .renderer_object()
                        .get_atlas_region()
                        .unwrap()
                        .page()
                        .c_ptr_ref()
                        .rendererObject as *const c_void
                });
                next_attachment_renderer_object
            } else {
                unreachable!();
            };

            if attachment_renderer_object != next_attachment_renderer_object {
                renderables.push(CombinedRenderable {
                    vertices,
                    uvs,
                    indices,
                    colors,
                    dark_colors,
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
            attachment_renderer_object = next_attachment_renderer_object;

            let (color, dark_color) = if let Some(mesh_attachment) =
                slot.attachment().and_then(|a| a.as_mesh())
            {
                let mut color = mesh_attachment.color() * slot.color() * skeleton.color();
                let mut dark_color = Color::new_rgba(0., 0., 0., 0.);
                if self.premultiplied_alpha {
                    color.premultiply_alpha();
                    dark_color.premultiply_alpha();
                    dark_color.a = 1.;
                }

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
                let mut dark_color = Color::new_rgba(0., 0., 0., 0.);
                if self.premultiplied_alpha {
                    color.premultiply_alpha();
                    dark_color.a = 1.;
                    dark_color.premultiply_alpha();
                }

                for i in 0..4 {
                    vertices.push([
                        world_vertices[i as usize * 2],
                        world_vertices[i as usize * 2 + 1],
                    ]);

                    unsafe {
                        uvs.push([
                            region_attachment.uvs()[i as usize * 2],
                            region_attachment.uvs()[i as usize * 2 + 1],
                        ]);
                    }

                    colors.push([color.r, color.g, color.b, color.a]);
                    dark_colors.push([dark_color.r, dark_color.g, dark_color.b, dark_color.a]);
                }

                if matches!(self.cull_direction, CullDirection::CounterClockwise) {
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 1);
                    indices.push(vertex_base + 0);
                    indices.push(vertex_base + 0);
                    indices.push(vertex_base + 3);
                    indices.push(vertex_base + 2);
                } else {
                    indices.push(vertex_base + 0);
                    indices.push(vertex_base + 1);
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 2);
                    indices.push(vertex_base + 3);
                    indices.push(vertex_base + 0);
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
                        spSkeletonClipping_clipTriangles(
                            clipper.c_ptr(),
                            &mut vertices[vertex_base as usize] as *mut f32,
                            vertices.len() as i32 - vertex_base as i32,
                            &mut indices[index_base as usize],
                            indices.len() as i32 - index_base as i32,
                            &mut uvs[vertex_base as usize] as *mut f32,
                            2,
                        );
                    }
                    unsafe {
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
                            indices.as_mut_ptr().offset(index_base as isize) as *mut u16,
                            clipped_triangles_size,
                        );
                        vertices.resize(vertex_base as usize + clipped_vertices_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedVertices).items,
                            vertices.as_mut_ptr().offset(vertex_base as isize) as *mut f32,
                            clipped_vertices_size,
                        );
                        uvs.resize(vertex_base as usize + clipped_uvs_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedUVs).items,
                            uvs.as_mut_ptr().offset(vertex_base as isize) as *mut f32,
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

        if indices.len() > 0 {
            renderables.push(CombinedRenderable {
                vertices,
                uvs,
                indices,
                colors,
                dark_colors,
                attachment_renderer_object,
            });
        }

        if let Some(clipper) = clipper.as_deref_mut() {
            clipper.clip_end2();
        }
        renderables
    }
}
