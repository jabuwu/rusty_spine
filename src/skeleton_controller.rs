use std::{mem::take, sync::Arc};

use crate::{
    animation_state::AnimationState,
    animation_state_data::AnimationStateData,
    c::{c_void, spSkeleton_setToSetupPose},
    color::Color,
    draw::{CullDirection, SimpleDrawer},
    skeleton::Skeleton,
    skeleton_clipping::SkeletonClipping,
    skeleton_data::SkeletonData,
    BlendMode,
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
        let mut skeleton = Skeleton::new(skeleton_data);
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
        let renderables = SimpleDrawer {
            cull_direction: self.settings.cull_direction,
        }
        .draw(&mut self.skeleton, Some(&mut self.clipper));
        renderables
            .into_iter()
            .map(|mut renderable| SkeletonRenderable {
                slot_index: renderable.slot_index,
                vertices: take(&mut renderable.vertices),
                uvs: take(&mut renderable.uvs),
                indices: take(&mut renderable.indices),
                color: renderable.color,
                dark_color: renderable.dark_color,
                blend_mode: renderable.blend_mode,
                premultiplied_alpha: self.settings.premultiplied_alpha,
                attachment_renderer_object: renderable.attachment_renderer_object,
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct SkeletonRenderable {
    pub slot_index: i32,
    pub vertices: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub color: Color,
    pub dark_color: Option<Color>,
    pub blend_mode: BlendMode,
    pub premultiplied_alpha: bool,
    pub attachment_renderer_object: Option<*const c_void>,
}
