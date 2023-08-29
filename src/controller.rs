//! Provides [`SkeletonController`], a helper struct for updating and drawing Spine skeletons.
//!
//! ```
//! use std::sync::Arc;
//!
//! use rusty_spine::{controller::*, *};
//!
//! // Load and initialize the atlas and skeleton
//! let atlas_path = "assets/spineboy/export/spineboy.atlas";
//! let json_path = "assets/spineboy/export/spineboy-pro.json";
//! let atlas = Arc::new(Atlas::new_from_file(atlas_path).unwrap());
//! let skeleton_json = SkeletonJson::new(atlas);
//! let skeleton_data = Arc::new(skeleton_json.read_skeleton_data_file(json_path).unwrap());
//! let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
//!
//! // Create the controller
//! let mut skeleton_controller =
//!     SkeletonController::new(skeleton_data.clone(), animation_state_data);
//!
//! // Update for one frame
//! skeleton_controller.update(0.016);
//!
//! // Get renderable mesh data
//! // Note: if slot_index is not important, use the much faster
//! // `combined_renderables()` method instead
//! let renderables = skeleton_controller.renderables();
//! println!("Skeleton:");
//! println!("");
//! println!("  Atlas: {}", atlas_path);
//! println!("  JSON: {}", json_path);
//! println!("  Version: {}", skeleton_data.version().unwrap_or("??"));
//! println!("  Hash: {}", skeleton_data.hash());
//! println!("");
//! println!("Renderables:");
//! println!("");
//! for renderable in renderables.iter() {
//!     let slot = skeleton_controller
//!         .skeleton
//!         .slot_at_index(renderable.slot_index)
//!         .unwrap();
//!     println!("  {}", slot.data().name());
//!     println!("    {} Vertices / UVs", renderable.vertices.len());
//!     println!("    {} Indices", renderable.indices.len());
//!     println!("    {:?} Blend Mode", renderable.blend_mode);
//!     println!("    {:?}", renderable.color);
//!     println!(
//!         "    {}",
//!         if renderable.premultiplied_alpha {
//!             "Premultiplied Alpha"
//!         } else {
//!             "No Premultiplied Alpha"
//!         }
//!     );
//!     println!("");
//! }
//! ```

use std::{mem::take, sync::Arc};

use crate::{
    animation_state::AnimationState,
    animation_state_data::AnimationStateData,
    c::c_void,
    color::Color,
    draw::{ColorSpace, CombinedDrawer, CullDirection, SimpleDrawer},
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
    /// Set to `true` if the textures are expected to have premultiplied alpha.
    pub premultiplied_alpha: bool,
    /// The cull direction to use for the vertices.
    pub cull_direction: CullDirection,
    /// The color space to use for the colors returned in [`SkeletonRenderable`] or  [`SkeletonCombinedRenderable`].
    pub color_space: ColorSpace,
}

impl Default for SkeletonControllerSettings {
    fn default() -> Self {
        Self {
            premultiplied_alpha: false,
            cull_direction: CullDirection::Clockwise,
            color_space: ColorSpace::SRGB,
        }
    }
}

impl SkeletonControllerSettings {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_premultiplied_alpha(self, premultiplied_alpha: bool) -> Self {
        Self {
            premultiplied_alpha,
            ..self
        }
    }

    #[must_use]
    pub const fn with_cull_direction(self, cull_direction: CullDirection) -> Self {
        Self {
            cull_direction,
            ..self
        }
    }

    #[must_use]
    pub const fn with_color_space(self, color_space: ColorSpace) -> Self {
        Self {
            color_space,
            ..self
        }
    }
}

impl SkeletonController {
    /// Creates a new skeleton and animation state instance with the given data.
    #[must_use]
    pub fn new(
        skeleton_data: Arc<SkeletonData>,
        animation_state_data: Arc<AnimationStateData>,
    ) -> Self {
        let mut skeleton = Skeleton::new(skeleton_data);
        skeleton.set_to_setup_pose();
        skeleton.update_world_transform();
        Self {
            skeleton,
            animation_state: AnimationState::new(animation_state_data),
            clipper: SkeletonClipping::new(),
            settings: SkeletonControllerSettings::default(),
        }
    }

    #[must_use]
    pub fn with_settings(self, settings: SkeletonControllerSettings) -> Self {
        Self { settings, ..self }
    }

    /// Updates the animation state, applies to the skeleton, and updates world transforms.
    pub fn update(&mut self, delta_seconds: f32) {
        self.animation_state.update(delta_seconds);
        self.animation_state.apply(&mut self.skeleton);
        self.skeleton.update_world_transform();
    }

    /// Render the skeleton using the [`SimpleDrawer`] and returns renderable mesh information.
    ///
    /// In most cases, it is preferable to use [`SkeletonController::combined_renderables`] which
    /// is significantly faster for complex rigs.
    pub fn renderables(&mut self) -> Vec<SkeletonRenderable> {
        let renderables = SimpleDrawer {
            cull_direction: self.settings.cull_direction,
            premultiplied_alpha: self.settings.premultiplied_alpha,
            color_space: self.settings.color_space,
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

    /// Render the skeleton using the [`CombinedDrawer`] and returns renderable mesh information.
    pub fn combined_renderables(&mut self) -> Vec<SkeletonCombinedRenderable> {
        let renderables = CombinedDrawer {
            cull_direction: self.settings.cull_direction,
            premultiplied_alpha: self.settings.premultiplied_alpha,
            color_space: self.settings.color_space,
        }
        .draw(&mut self.skeleton, Some(&mut self.clipper));
        renderables
            .into_iter()
            .map(|mut renderable| SkeletonCombinedRenderable {
                vertices: take(&mut renderable.vertices),
                uvs: take(&mut renderable.uvs),
                indices: take(&mut renderable.indices),
                colors: renderable.colors,
                dark_colors: renderable.dark_colors,
                blend_mode: renderable.blend_mode,
                premultiplied_alpha: self.settings.premultiplied_alpha,
                attachment_renderer_object: renderable.attachment_renderer_object,
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct SkeletonRenderable {
    pub slot_index: usize,
    pub vertices: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub color: Color,
    pub dark_color: Color,
    pub blend_mode: BlendMode,
    pub premultiplied_alpha: bool,
    pub attachment_renderer_object: Option<*const c_void>,
}

#[derive(Debug, Clone)]
pub struct SkeletonCombinedRenderable {
    pub vertices: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub colors: Vec<[f32; 4]>,
    pub dark_colors: Vec<[f32; 4]>,
    pub blend_mode: BlendMode,
    pub premultiplied_alpha: bool,
    pub attachment_renderer_object: Option<*const c_void>,
}
