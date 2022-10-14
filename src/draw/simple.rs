use crate::{
    c::{c_void, spSkeletonClipping_clipTriangles},
    BlendMode, Color, Skeleton, SkeletonClipping,
};

#[cfg(feature="spine38")]
use crate::c::spMeshAttachment_updateUVs;

#[cfg(not(feature="spine38"))]
use crate::c::spMeshAttachment_updateRegion;

use super::CullDirection;

pub struct SimpleRenderable {
    pub slot_index: i32,
    pub vertices: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub color: Color,
    pub dark_color: Color,
    pub blend_mode: BlendMode,
    pub attachment_renderer_object: Option<*const c_void>,
}

pub struct SimpleDrawer {
    pub cull_direction: CullDirection,
    pub premultiplied_alpha: bool,
}

/// A simple drawer with no optimizations.
///
/// Assumes use of the default atlas attachment loader.
impl SimpleDrawer {
    pub fn draw(
        &self,
        skeleton: &mut Skeleton,
        mut clipper: Option<&mut SkeletonClipping>,
    ) -> Vec<SimpleRenderable> {
        let mut renderables = vec![];
        let mut world_vertices = vec![];
        world_vertices.resize(1000, 0.);
        for slot_index in 0..skeleton.slots_count() {
            let slot = skeleton.draw_order_at_index(slot_index).unwrap();
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
                #[cfg(feature="spine38")]
                unsafe {
                    spMeshAttachment_updateUVs(mesh_attachment.c_ptr());
                };

                #[cfg(not(feature="spine38"))]
                unsafe {
                    spMeshAttachment_updateRegion(mesh_attachment.c_ptr());
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
                #[cfg(not(feature="spine38"))]
                unsafe {
                    region_attachment.compute_world_vertices(&slot, &mut world_vertices, 0, 2);
                }
                #[cfg(feature="spine38")]
                unsafe {
                    region_attachment.compute_world_vertices(&slot.bone(), &mut world_vertices, 0, 2);
                }

                vertices.reserve(4);
                uvs.reserve(4);
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
                        spSkeletonClipping_clipTriangles(
                            clipper.c_ptr(),
                            vertices.as_mut_ptr() as *mut f32,
                            vertices.len() as i32,
                            indices.as_mut_ptr(),
                            indices.len() as i32,
                            uvs.as_mut_ptr() as *mut f32,
                            2,
                        );
                    }
                    unsafe {
                        let clipped_vertices_size =
                            (*clipper.c_ptr_ref().clippedVertices).size as usize;
                        vertices.resize(clipped_vertices_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedVertices).items,
                            vertices.as_mut_ptr() as *mut f32,
                            clipped_vertices_size,
                        );
                        let clipped_triangles_size =
                            (*clipper.c_ptr_ref().clippedTriangles).size as usize;
                        indices.resize(clipped_triangles_size, 0);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedTriangles).items,
                            indices.as_mut_ptr() as *mut u16,
                            clipped_triangles_size,
                        );
                        let clipped_uvs_size = (*clipper.c_ptr_ref().clippedUVs).size as usize;
                        uvs.resize(clipped_uvs_size / 2, [0., 0.]);
                        std::ptr::copy_nonoverlapping(
                            (*clipper.c_ptr_ref().clippedUVs).items,
                            uvs.as_mut_ptr() as *mut f32,
                            clipped_uvs_size,
                        );
                    }
                }
            }

            let attachment_renderer_object = if let Some(mesh_attachment) =
                slot.attachment().and_then(|a| a.as_mesh())
            {
                Some(unsafe {
                    mesh_attachment
                        .renderer_object()
                        .get_atlas_region()
                        .unwrap()
                        .page()
                        .c_ptr_ref()
                        .rendererObject as *const c_void
                })
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                Some(unsafe {
                    region_attachment
                        .renderer_object()
                        .get_atlas_region()
                        .unwrap()
                        .page()
                        .c_ptr_ref()
                        .rendererObject as *const c_void
                })
            } else {
                None
            };

            color *= slot.color() * skeleton.color();
            if self.premultiplied_alpha {
                color.premultiply_alpha();
            }

            let mut dark_color = slot
                .dark_color()
                .unwrap_or(Color::new_rgba(0.0, 0.0, 0.0, 0.0));
            if self.premultiplied_alpha {
                dark_color.a = 1.0;
                dark_color.premultiply_alpha();
            }

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

        if let Some(clipper) = clipper.as_deref_mut() {
            clipper.clip_end2();
        }
        renderables
    }
}
