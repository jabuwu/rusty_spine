use std::sync::Arc;

use crate::{
    animation_state::AnimationState,
    animation_state_data::AnimationStateData,
    c::{
        c_void, spSkeletonClipping_clipTriangles, spSkeletonClipping_isClipping,
        spSkeleton_setToSetupPose,
    },
    color::Color,
    draw::CullDirection,
    skeleton::Skeleton,
    skeleton_clipping::SkeletonClipping,
    skeleton_data::SkeletonData,
};

#[derive(Debug)]
pub struct SkeletonController {
    pub skeleton: Skeleton,
    pub animation_state: AnimationState,
    pub clipper: SkeletonClipping,
    pub settings: SkeletonControllerSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonControllerSettings {
    pub premultiplied_alpha: bool,
    pub cull_direction: CullDirection,
}

impl Default for SkeletonControllerSettings {
    fn default() -> Self {
        Self {
            premultiplied_alpha: false,
            cull_direction: CullDirection::Clockwise,
        }
    }
}

impl SkeletonControllerSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_premultiplied_alpha(self, premultiplied_alpha: bool) -> Self {
        Self {
            premultiplied_alpha,
            ..self
        }
    }

    pub fn with_cull_direction(self, cull_direction: CullDirection) -> Self {
        Self {
            cull_direction,
            ..self
        }
    }
}

impl SkeletonController {
    pub fn new(
        skeleton_data: Arc<SkeletonData>,
        animation_state_data: Arc<AnimationStateData>,
    ) -> Self {
        let mut skeleton = Skeleton::new(skeleton_data).unwrap();
        unsafe {
            spSkeleton_setToSetupPose(skeleton.c_ptr());
        }
        skeleton.update_world_transform();
        Self {
            skeleton,
            animation_state: AnimationState::new(animation_state_data),
            clipper: SkeletonClipping::new(),
            settings: SkeletonControllerSettings::default(),
        }
    }

    pub fn with_settings(self, settings: SkeletonControllerSettings) -> Self {
        Self { settings, ..self }
    }

    pub fn update(&mut self, delta_seconds: f32) {
        self.animation_state.update(delta_seconds);
        self.animation_state.apply(&mut self.skeleton);
        self.skeleton.update_world_transform();
    }

    pub fn renderables(&mut self) -> Vec<SkeletonRenderable> {
        let mut renderables = vec![];
        for slot_index in 0..self.skeleton.slots_count() {
            let slot = self
                .skeleton
                .draw_order_at_index(slot_index as usize)
                .unwrap();
            if !slot.bone().active() {
                self.clipper.clip_end(&slot);
                continue;
            }

            let mut vertices_raw = vec![];
            let mut indices_raw = vec![];
            let mut uvs_raw = vec![];
            let mut color;

            if let Some(mesh_attachment) = slot.attachment().and_then(|a| a.as_mesh()) {
                color = mesh_attachment.color();

                let mut world_vertices = vec![];
                world_vertices.resize(1000, 0.);
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

                for i in 0..mesh_attachment.world_vertices_length() {
                    vertices_raw.push(world_vertices[i as usize * 2]);
                    vertices_raw.push(world_vertices[i as usize * 2 + 1]);

                    unsafe {
                        uvs_raw.push(*mesh_attachment.c_ptr_mut().uvs.offset(i as isize * 2));
                        uvs_raw.push(*mesh_attachment.c_ptr_mut().uvs.offset(i as isize * 2 + 1));
                    }
                }

                for i in 0..mesh_attachment.triangles_count() {
                    indices_raw
                        .push(unsafe { *mesh_attachment.triangles().offset(i as isize) } as u16);
                }
            } else if let Some(region_attachment) = slot.attachment().and_then(|a| a.as_region()) {
                color = region_attachment.color();

                let mut world_vertices = vec![];
                world_vertices.resize(1000, 0.);
                unsafe {
                    region_attachment.compute_world_vertices(&slot, &mut world_vertices, 0, 2);
                }

                for i in 0..4 {
                    vertices_raw.push(world_vertices[i as usize * 2]);
                    vertices_raw.push(world_vertices[i as usize * 2 + 1]);

                    uvs_raw.push(region_attachment.c_ptr_mut().uvs[i as usize * 2]);
                    uvs_raw.push(region_attachment.c_ptr_mut().uvs[i as usize * 2 + 1]);
                }

                indices_raw = vec![0, 1, 2, 2, 3, 0];
            } else if let Some(clipping_attachment) =
                slot.attachment().and_then(|a| a.as_clipping())
            {
                self.clipper.clip_start(&slot, &clipping_attachment);
                continue;
            } else {
                self.clipper.clip_end(&slot);
                continue;
            }

            let mut vertices = vec![];
            let mut uvs = vec![];
            let mut indices = vec![];
            unsafe {
                if spSkeletonClipping_isClipping(self.clipper.c_ptr()) != 0 {
                    spSkeletonClipping_clipTriangles(
                        self.clipper.c_ptr(),
                        vertices_raw.as_mut_ptr(),
                        vertices_raw.len() as i32 / 2,
                        indices_raw.as_mut_ptr(),
                        indices_raw.len() as i32,
                        uvs_raw.as_mut_ptr(),
                        2,
                    );
                    let clipped_vertices_size = (*self.clipper.c_ptr_ref().clippedVertices).size;
                    for i in 0..(clipped_vertices_size / 2) {
                        vertices.push([
                            *(*self.clipper.c_ptr_ref().clippedVertices)
                                .items
                                .offset(i as isize * 2),
                            *(*self.clipper.c_ptr_ref().clippedVertices)
                                .items
                                .offset(i as isize * 2 + 1),
                            0.,
                        ]);
                    }
                    let clipped_triangles_size = (*self.clipper.c_ptr_ref().clippedTriangles).size;
                    for i in 0..clipped_triangles_size {
                        indices.push(
                            *(*self.clipper.c_ptr_ref().clippedTriangles)
                                .items
                                .offset(i as isize) as u32,
                        );
                    }
                    let clipped_uvs_size = (*self.clipper.c_ptr_ref().clippedUVs).size;
                    for i in 0..(clipped_uvs_size / 2) {
                        uvs.push([
                            *(*self.clipper.c_ptr_ref().clippedUVs)
                                .items
                                .offset(i as isize * 2),
                            *(*self.clipper.c_ptr_ref().clippedUVs)
                                .items
                                .offset(i as isize * 2 + 1),
                        ]);
                    }
                } else {
                    for i in 0..(vertices_raw.len() / 2) {
                        vertices.push([vertices_raw[i * 2], vertices_raw[i * 2 + 1], 0.]);
                    }
                    for index in indices_raw.iter() {
                        indices.push(*index as u32);
                    }
                    for i in 0..vertices.len() {
                        uvs.push([uvs_raw[i * 2], uvs_raw[i * 2 + 1]]);
                    }
                }
            }

            if self.settings.cull_direction == CullDirection::CounterClockwise {
                for i in 0..indices.len() / 3 {
                    let a = indices[i * 3 + 1];
                    indices[i * 3] = indices[i * 3];
                    indices[i * 3 + 1] = indices[i * 3 + 2];
                    indices[i * 3 + 2] = a;
                }
            }

            self.clipper.clip_end(&slot);

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

            color.r *= slot.c_ptr_mut().color.r * self.skeleton.c_ptr_mut().color.r;
            color.g *= slot.c_ptr_mut().color.g * self.skeleton.c_ptr_mut().color.g;
            color.b *= slot.c_ptr_mut().color.b * self.skeleton.c_ptr_mut().color.b;
            color.a *= slot.c_ptr_mut().color.a * self.skeleton.c_ptr_mut().color.a;

            renderables.push(SkeletonRenderable {
                slot_index: slot_index as usize,
                vertices,
                uvs,
                indices,
                color,
                blend_mode: BlendMode::Normal,
                premultiplied_alpha: self.settings.premultiplied_alpha,
                attachment_renderer_object,
            });
        }
        self.clipper.clip_end2();
        renderables
    }
}

#[derive(Debug, Clone)]
pub struct SkeletonRenderable {
    pub slot_index: usize,
    pub vertices: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub color: Color,
    pub blend_mode: BlendMode,
    pub premultiplied_alpha: bool,
    pub attachment_renderer_object: Option<*const c_void>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Additive,
    Multiply,
    Screen,
}
