//! Demonstrates a complete Spine integration using `miniquad`.
//!
//! Below is a list of all features that should be considered when integrating Spine into a project.
//!
//! # Texture Creation & Disposal
//!
//! Callbacks must first be set to handle texture loading upon loading a [`rusty_spine::Atlas`].
//! See [`SpineTexture`].
//!
//! # Texture Runtime Settings
//!
//! Defined in the [`rusty_spine::atlas::AtlasPage`], atlases contain runtime configuration settings
//! for each texture. This includes the min filter, mag filter, uv wrapping, and texture format.
//! This example only supports a few texture formats, see [`SpineTextureToLoad::load`].
//!
//! # Blend Modes
//!
//! Slots within Spine can be assigned a blend mode, and this value is exposed on the renderable
//! object returned from [`rusty_spine::controller::SkeletonController::renderables`] or
//! [`rusty_spine::controller::SkeletonController::combined_renderables`]. See [`BlendStates`] for
//! how to handle these blend modes.
//!
//! # Premultiplied Alpha
//!
//! An export option within Spine allows textures to use premultiplied alpha. To support this
//! feature, additional blend states are required. See [`BlendStates`]. To detect if a skeleton was
//! exported with this option, iterate over [`rusty_spine::Atlas::pages`] and check if any page has
//! [`rusty_spine::atlas::AtlasPage::pma`] set to true.
//!
//! # Backface Culling
//!
//! Spine animations may or may not rely on backface culling in their animations. This information
//! is not exposed by skeleton files and must be coordinated manually. For an example of where
//! backface culling is problematic, see the coin in this example. Enabling
//! [`SpineDemo::backface_culling`] for that skeleton will cause it to not render correctly.
//!
//! # Dark Colors
//!
//! In addition to the usual vertex data (specifically: position, uv, and color - see [`Vertex`]), a
//! dark color needs to be sent to the fragment shader (see [`shader::FRAGMENT`]). Dark colors can
//! be animated and allow changing how darkened shades of a texture are lit. To see it in action,
//! see the coin in this example.
//!
//! # Audio Events
//!
//! Events can have audio files associated with them (along with a volume and balance) that can be
//! hooked up to the audio system to play sounds. This example does not support playing audio
//! events, however, the events will be printed to console.
//!
//! # Y Down
//!
//! Game frameworks differ on whether +Y is up or down. If your app uses +Y as down, pass `true` to
//! [`Bone::set_y_down`]. This is a global value.

use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
};

use egui_miniquad::EguiMq;

use image::GenericImageView;
use rusty_spine::{
    atlas::{AtlasFilter, AtlasFormat, AtlasWrap},
    controller::{SkeletonController, SkeletonControllerSettings},
    draw::{ColorSpace, CullDirection},
    AnimationEvent, AnimationStateData, Atlas, BlendMode, Bone, Color, Physics, SkeletonBinary,
    SkeletonJson,
};

const MAX_MESH_VERTICES: usize = 10000;
const MAX_MESH_INDICES: usize = 5000;

/// Holds all data related to load and demonstrate a particular Spine skeleton.
#[derive(Clone, Copy)]
struct SpineDemo {
    pub atlas_path: &'static str,
    pub skeleton_path: SpineSkeletonPath,
    pub animation: &'static str,
    pub position: glam::Vec2,
    pub scale: f32,
    pub skin: Option<&'static str>,
    pub backface_culling: bool,
}

#[derive(Clone, Copy)]
enum SpineSkeletonPath {
    Binary(&'static str),
    Json(&'static str),
}

const SPINE_DEMOS: &[SpineDemo] = &[
    SpineDemo {
        atlas_path: "assets/spineboy/export/spineboy-pma.atlas",
        skeleton_path: SpineSkeletonPath::Binary("assets/spineboy/export/spineboy-pro.skel"),
        animation: "portal",
        position: glam::Vec2::new(0.0, -220.0),
        scale: 0.5,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/windmill/export/windmill-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/windmill/export/windmill-ess.json"),
        animation: "animation",
        position: glam::Vec2::new(0.0, -80.0),
        scale: 0.5,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/alien/export/alien-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/alien/export/alien-pro.json"),
        animation: "death",
        position: glam::Vec2::new(0.0, -260.0),
        scale: 0.3,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/celestial-circus/export/celestial-circus-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json(
            "assets/celestial-circus/export/celestial-circus-pro.json",
        ),
        animation: "swing",
        position: glam::Vec2::new(0.0, -120.0),
        scale: 0.2,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/cloud-pot/export/cloud-pot-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/cloud-pot/export/cloud-pot.json"),
        animation: "playing-in-the-rain",
        position: glam::Vec2::new(0.0, -220.0),
        scale: 0.4,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/dragon/export/dragon-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/dragon/export/dragon-ess.json"),
        animation: "flying",
        position: glam::Vec2::new(0.0, -50.0),
        scale: 0.7,
        skin: None,
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/goblins/export/goblins-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/goblins/export/goblins-pro.json"),
        animation: "walk",
        position: glam::Vec2::new(0.0, -200.0),
        scale: 1.0,
        skin: Some("goblingirl"),
        backface_culling: true,
    },
    SpineDemo {
        atlas_path: "assets/coin/export/coin-pma.atlas",
        skeleton_path: SpineSkeletonPath::Json("assets/coin/export/coin-pro.json"),
        animation: "animation",
        position: glam::Vec2::ZERO,
        scale: 1.0,
        skin: None,
        backface_culling: false,
    },
];

/// An instance of this enum is created for each loaded [`rusty_spine::atlas::AtlasPage`] upon
/// loading a [`rusty_spine::Atlas`]. To see how this is done, see the [`main`] function of this
/// example. It utilizes the following callbacks which must be set only once in an application:
/// - [`rusty_spine::extension::set_create_texture_cb`]
/// - [`rusty_spine::extension::set_dispose_texture_cb`]
///
/// The implementation in this example defers loading by setting the texture to
/// [`SpineTexture::NeedsToBeLoaded`] and handling it later, but in other applications, it may be
/// possible to load the textures immediately, or on another thread.
#[derive(Debug)]
enum SpineTexture {
    NeedsToBeLoaded(SpineTextureToLoad),
    Loaded(miniquad::TextureId),
}

#[derive(Debug)]
struct SpineTextureToLoad {
    pub path: String,
    pub min_filter: AtlasFilter,
    pub mag_filter: AtlasFilter,
    pub u_wrap: AtlasWrap,
    pub v_wrap: AtlasWrap,
    pub format: AtlasFormat,
}

impl SpineTextureToLoad {
    pub fn load(&self, ctx: &mut dyn miniquad::RenderingBackend) -> miniquad::TextureId {
        #[allow(clippy::needless_borrows_for_generic_args)]
        let image = image::ImageReader::open(&self.path)
            .unwrap_or_else(|_| panic!("failed to open image: {}", &self.path))
            .decode()
            .unwrap_or_else(|_| panic!("failed to decode image: {}", &self.path));
        let (width, height) = image.dimensions();

        // The runtime texture format is set during export of the atlas and is meant to
        // control the format of the texture uploaded to the GPU. However, miniquad
        // supports a very limited set of texture formats, so unsupported texture
        // formats fall back to a different format.
        let (image_format, image_bytes) = match self.format {
            AtlasFormat::Alpha => (
                miniquad::TextureFormat::Alpha,
                image.into_rgba8().pixels().map(|pixel| pixel[3]).collect(),
            ),
            AtlasFormat::Intensity => (miniquad::TextureFormat::Alpha, image.into_luma8().to_vec()),
            format @ AtlasFormat::LuminanceAlpha => {
                // miniquad does not support textures with only two channels
                // we take some creative liberties here and store:
                // (luma, luma, luma, alpha) as RGBA8
                // because it looks better in the example
                println!("WARNING: Atlas used an unsupported runtime texture format ({format:?}), using RGBA8 instead as (Luma, Luma, Luma, Alpha)");
                (
                    miniquad::TextureFormat::RGBA8,
                    image
                        .into_luma_alpha8()
                        .pixels()
                        .flat_map(|pixel| [pixel[0], pixel[0], pixel[0], pixel[1]])
                        .collect(),
                )
            }
            AtlasFormat::RGB888 => (miniquad::TextureFormat::RGB8, image.into_rgb8().to_vec()),
            AtlasFormat::RGBA8888 => (miniquad::TextureFormat::RGBA8, image.into_rgba8().to_vec()),
            format => {
                let fallback_format = match format {
                    AtlasFormat::RGB565 => miniquad::TextureFormat::RGB8,
                    _ => miniquad::TextureFormat::RGBA8,
                };
                println!("WARNING: Atlas used an unsupported runtime texture format ({format:?}), using {fallback_format:?} instead");
                let image_bytes = match fallback_format {
                    miniquad::TextureFormat::RGB8 => image.to_rgb8().to_vec(),
                    miniquad::TextureFormat::RGBA8 => image.to_rgba8().to_vec(),
                    _ => unreachable!(),
                };
                (fallback_format, image_bytes)
            }
        };

        // Convert texture runtime settings to miniquad values
        let (min_filter, mipmap_filter) = Self::convert_filter(self.min_filter);
        let (mag_filter, _) = Self::convert_filter(self.mag_filter);
        let allocate_mipmaps = mipmap_filter != miniquad::MipmapFilterMode::None;
        let wrap_x = Self::convert_wrap(self.u_wrap);
        let wrap_y = Self::convert_wrap(self.v_wrap);

        // Create and setup the texture
        let texture_params = miniquad::TextureParams {
            width,
            height,
            format: image_format,
            allocate_mipmaps,
            ..Default::default()
        };
        let texture = ctx.new_texture(
            miniquad::TextureAccess::Static,
            miniquad::TextureSource::Bytes(&image_bytes),
            texture_params,
        );
        ctx.texture_set_min_filter(texture, min_filter, mipmap_filter);
        ctx.texture_set_mag_filter(texture, mag_filter);
        ctx.texture_set_wrap(texture, wrap_x, wrap_y);
        if allocate_mipmaps {
            ctx.texture_generate_mipmaps(texture);
        }
        texture
    }

    fn convert_filter(filter: AtlasFilter) -> (miniquad::FilterMode, miniquad::MipmapFilterMode) {
        match filter {
            AtlasFilter::Linear => (
                miniquad::FilterMode::Linear,
                miniquad::MipmapFilterMode::None,
            ),
            AtlasFilter::Nearest => (
                miniquad::FilterMode::Nearest,
                miniquad::MipmapFilterMode::None,
            ),
            AtlasFilter::Mipmap => (
                miniquad::FilterMode::Linear,
                miniquad::MipmapFilterMode::Linear,
            ),
            AtlasFilter::MipmapNearestNearest => (
                miniquad::FilterMode::Nearest,
                miniquad::MipmapFilterMode::Nearest,
            ),
            AtlasFilter::MipmapLinearNearest => (
                miniquad::FilterMode::Linear,
                miniquad::MipmapFilterMode::Nearest,
            ),
            AtlasFilter::MipmapNearestLinear => (
                miniquad::FilterMode::Nearest,
                miniquad::MipmapFilterMode::Linear,
            ),
            AtlasFilter::MipmapLinearLinear => (
                miniquad::FilterMode::Linear,
                miniquad::MipmapFilterMode::Linear,
            ),
            _ => unreachable!(),
        }
    }

    fn convert_wrap(wrap: AtlasWrap) -> miniquad::TextureWrap {
        match wrap {
            AtlasWrap::ClampToEdge => miniquad::TextureWrap::Clamp,
            AtlasWrap::MirroredRepeat => miniquad::TextureWrap::Mirror,
            AtlasWrap::Repeat => miniquad::TextureWrap::Repeat,
            _ => unreachable!(),
        }
    }
}

/// Holds all data related to rendering Spine skeletons in this example.
struct Spine {
    pub controller: SkeletonController,
    pub world: glam::Mat4,
    pub cull_face: miniquad::CullFace,
}

impl Spine {
    pub fn load(info: SpineDemo) -> Self {
        // Load atlas and auto-detect if the textures are premultiplied
        let atlas = Arc::new(
            Atlas::new_from_file(info.atlas_path)
                .unwrap_or_else(|_| panic!("failed to load atlas file: {}", info.atlas_path)),
        );
        let premultiplied_alpha = atlas.pages().any(|page| page.pma());

        // Load either binary or json skeleton files
        let skeleton_data = Arc::new(match info.skeleton_path {
            SpineSkeletonPath::Binary(path) => {
                let skeleton_binary = SkeletonBinary::new(atlas);
                skeleton_binary
                    .read_skeleton_data_file(path)
                    .unwrap_or_else(|_| panic!("failed to load binary skeleton file: {path}"))
            }
            SpineSkeletonPath::Json(path) => {
                let skeleton_json = SkeletonJson::new(atlas);
                skeleton_json
                    .read_skeleton_data_file(path)
                    .unwrap_or_else(|_| panic!("failed to load json skeleton file: {path}"))
            }
        });

        // Create animation state data from a skeleton
        // If desired, set crossfades at this point
        // See [`rusty_spine::AnimationStateData::set_mix_by_name`]
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));

        // Instantiate the [`rusty_spine::controller::SkeletonController`] helper class which
        // handles creating the live data ([`rusty_spine::Skeleton`] and
        // [`rusty_spine::AnimationState`] and capable of generating mesh render data.
        // Use of this helper is not required but it does handle a lot of little things for you.
        let mut controller = SkeletonController::new(skeleton_data, animation_state_data)
            .with_settings(SkeletonControllerSettings {
                premultiplied_alpha,
                cull_direction: CullDirection::CounterClockwise,
                color_space: ColorSpace::SRGB,
            });

        // Listen for animation events
        controller
            .animation_state
            .set_listener(|_, animation_event| match animation_event {
                AnimationEvent::Start { track_entry } => {
                    println!("Animation {} started!", track_entry.track_index());
                }
                AnimationEvent::Interrupt { track_entry } => {
                    println!("Animation {} interrupted!", track_entry.track_index());
                }
                AnimationEvent::End { track_entry } => {
                    println!("Animation {} ended!", track_entry.track_index());
                }
                AnimationEvent::Complete { track_entry } => {
                    println!("Animation {} completed!", track_entry.track_index());
                }
                AnimationEvent::Dispose { track_entry } => {
                    println!("Animation {} disposed!", track_entry.track_index());
                }
                AnimationEvent::Event {
                    track_entry,
                    name,
                    int,
                    float,
                    string,
                    audio_path,
                    volume,
                    balance,
                    ..
                } => {
                    println!("Animation {} event!", track_entry.track_index());
                    println!("  Name: {name}");
                    println!("  Integer: {int}");
                    println!("  Float: {float}");
                    if !string.is_empty() {
                        println!("  String: \"{string}\"");
                    }
                    if !audio_path.is_empty() {
                        println!("  Audio: \"{audio_path}\"");
                        println!("    Volume: {volume}");
                        println!("    Balance: {balance}");
                    }
                }
            });

        // Start the animation on track 0 and loop
        controller
            .animation_state
            .set_animation_by_name(0, info.animation, true)
            .unwrap_or_else(|_| panic!("failed to start animation: {}", info.animation));

        // If a skin was provided, set it
        if let Some(skin) = info.skin {
            controller
                .skeleton
                .set_skin_by_name(skin)
                .unwrap_or_else(|_| panic!("failed to set skin: {skin}"));
        }

        controller.settings.premultiplied_alpha = premultiplied_alpha;
        Self {
            controller,
            world: glam::Mat4::from_translation(info.position.extend(0.0))
                * glam::Mat4::from_scale(glam::Vec2::splat(info.scale).extend(1.0)),
            cull_face: match info.backface_culling {
                false => miniquad::CullFace::Nothing,
                true => miniquad::CullFace::Back,
            },
        }
    }
}

#[repr(C)]
struct Vertex {
    pub position: glam::Vec2,
    pub uv: glam::Vec2,
    pub color: Color,
    pub dark_color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineKey {
    pub premultiplied_alpha: bool,
    pub blend_mode: BlendMode,
    pub cull_face: miniquad::CullFace,
}

impl std::hash::Hash for PipelineKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(if self.premultiplied_alpha { 1 } else { 0 });
        state.write_u8(self.blend_mode as u8);
        state.write_u8(self.cull_face as u8);
    }
}

/// Convert a [`rusty_spine::BlendMode`] to a pair of [`miniquad::BlendState`]s. One for alpha, one
/// for color.
///
/// Spine supports 4 different blend modes:
/// - [`rusty_spine::BlendMode::Additive`]
/// - [`rusty_spine::BlendMode::Multiply`]
/// - [`rusty_spine::BlendMode::Normal`]
/// - [`rusty_spine::BlendMode::Screen`]
///
/// And blend states are different depending on if the texture has premultiplied alpha values.
///
/// So, 8 blend states must be supported. See [`GetBlendStates::get_blend_states`] below.
struct BlendStates {
    pub alpha_blend: miniquad::BlendState,
    pub color_blend: miniquad::BlendState,
}

trait GetBlendStates {
    fn get_blend_states(&self, premultiplied_alpha: bool) -> BlendStates;
}

impl GetBlendStates for BlendMode {
    fn get_blend_states(&self, premultiplied_alpha: bool) -> BlendStates {
        match self {
            Self::Additive => match premultiplied_alpha {
                // Case 1: Additive Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::One,
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::One,
                    ),
                },
                // Case 2: Additive Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::One,
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::One,
                    ),
                },
            },
            Self::Multiply => match premultiplied_alpha {
                // Case 3: Multiply Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::DestinationColor),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
                // Case 4: Multiply Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::DestinationColor),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
            },
            Self::Normal => match premultiplied_alpha {
                // Case 5: Normal Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
                // Case 6: Normal Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
            },
            Self::Screen => match premultiplied_alpha {
                // Case 7: Screen Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceColor),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
                // Case 8: Screen Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceColor),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                    color_blend: miniquad::BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::One,
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    ),
                },
            },
        }
    }
}

struct Stage {
    ctx: Box<dyn miniquad::RenderingBackend>,
    egui: EguiMq,
    pipelines: HashMap<PipelineKey, miniquad::Pipeline>,
    default_texture: miniquad::TextureId,
    bindings: Vec<miniquad::Bindings>,
    spine: Spine,
    current_spine_demo: usize,
    last_frame_time: f64,
    screen_size: glam::Vec2,
    texture_delete_receiver: mpsc::Receiver<miniquad::TextureId>,
}

impl Stage {
    fn new(texture_delete_receiver: mpsc::Receiver<miniquad::TextureId>) -> Self {
        let mut ctx = miniquad::window::new_rendering_backend();

        // Setup egui
        let egui = EguiMq::new(&mut *ctx);
        egui.egui_ctx()
            .set_zoom_factor(miniquad::window::dpi_scale());

        // Create the Spine shader
        let shader = shader::create(&mut *ctx);

        // Create pipelines for rendering all variations of Spine meshes
        let mut pipelines = HashMap::new();
        for premultiplied_alpha in [false, true] {
            for blend_mode in [
                BlendMode::Normal,
                BlendMode::Additive,
                BlendMode::Multiply,
                BlendMode::Screen,
            ] {
                let BlendStates {
                    alpha_blend,
                    color_blend,
                } = blend_mode.get_blend_states(premultiplied_alpha);
                for cull_face in [miniquad::CullFace::Nothing, miniquad::CullFace::Back] {
                    let pipeline_key = PipelineKey {
                        premultiplied_alpha,
                        blend_mode,
                        cull_face,
                    };

                    pipelines.insert(
                        pipeline_key,
                        ctx.new_pipeline(
                            &[miniquad::BufferLayout::default()],
                            &[
                                miniquad::VertexAttribute::new(
                                    "position",
                                    miniquad::VertexFormat::Float2,
                                ),
                                miniquad::VertexAttribute::new(
                                    "uv",
                                    miniquad::VertexFormat::Float2,
                                ),
                                miniquad::VertexAttribute::new(
                                    "color",
                                    miniquad::VertexFormat::Float4,
                                ),
                                miniquad::VertexAttribute::new(
                                    "dark_color",
                                    miniquad::VertexFormat::Float4,
                                ),
                            ],
                            shader,
                            miniquad::PipelineParams {
                                cull_face,
                                alpha_blend: Some(alpha_blend),
                                color_blend: Some(color_blend),
                                ..Default::default()
                            },
                        ),
                    );
                }
            }
        }

        // Create a default texture to use before textures are loaded
        let default_texture = ctx.new_texture_from_rgba8(
            2,
            2,
            &[
                0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                0x00, 0xFF,
            ],
        );

        // Load the first spine demo
        let current_spine_demo = 0;
        let spine = Spine::load(SPINE_DEMOS[current_spine_demo]);

        Self {
            ctx,
            egui,
            pipelines,
            default_texture,
            bindings: vec![],
            spine,
            current_spine_demo,
            last_frame_time: miniquad::date::now(),
            screen_size: glam::Vec2::new(800., 600.),
            texture_delete_receiver,
        }
    }

    fn view(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh_gl(
            self.screen_size.x * -0.5,
            self.screen_size.x * 0.5,
            self.screen_size.y * -0.5,
            self.screen_size.y * 0.5,
            -1.0,
            1.0,
        )
    }
}

impl miniquad::EventHandler for Stage {
    fn update(&mut self) {
        let now = miniquad::date::now();
        let dt = ((now - self.last_frame_time) as f32).max(0.001);
        self.spine.controller.update(dt, Physics::Update);
        self.last_frame_time = now;
    }

    fn draw(&mut self) {
        let renderables = self.spine.controller.combined_renderables();

        // Create bindings that can be re-used for rendering Spine meshes
        while renderables.len() > self.bindings.len() {
            let vertex_buffer = self.ctx.new_buffer(
                miniquad::BufferType::VertexBuffer,
                miniquad::BufferUsage::Stream,
                miniquad::BufferSource::empty::<Vertex>(MAX_MESH_VERTICES),
            );
            let index_buffer = self.ctx.new_buffer(
                miniquad::BufferType::IndexBuffer,
                miniquad::BufferUsage::Stream,
                miniquad::BufferSource::empty::<u16>(MAX_MESH_INDICES),
            );
            self.bindings.push(miniquad::Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![self.default_texture],
            });
        }

        // Delete textures that are no longer used.
        loop {
            match self.texture_delete_receiver.try_recv() {
                Ok(texture) => {
                    self.ctx.delete_texture(texture);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    break;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Texture delete receiver closed unexpectedly.");
                }
            }
        }

        // Update egui
        self.egui.run(&mut *self.ctx, |_, egui_ctx| {
            ui(egui_ctx);
        });

        let view = self.view();
        for (renderable, bindings) in renderables.iter().zip(self.bindings.iter_mut()) {
            // Create the vertex and index buffers for miniquad
            let mut vertices = vec![];
            for vertex_index in 0..renderable.vertices.len() {
                vertices.push(Vertex {
                    position: glam::Vec2::new(
                        renderable.vertices[vertex_index][0],
                        renderable.vertices[vertex_index][1],
                    ),
                    uv: glam::Vec2::new(
                        renderable.uvs[vertex_index][0],
                        renderable.uvs[vertex_index][1],
                    ),
                    color: Color::from(renderable.colors[vertex_index]),
                    dark_color: Color::from(renderable.dark_colors[vertex_index]),
                });
            }
            self.ctx.buffer_update(
                bindings.vertex_buffers[0],
                miniquad::BufferSource::slice(&vertices),
            );
            self.ctx.buffer_update(
                bindings.index_buffer,
                miniquad::BufferSource::slice(&renderable.indices),
            );

            // If there is no attachment (and therefore no texture), skip rendering this renderable
            // May also be None if a create texture callback was never set.
            let Some(attachment_renderer_object) = renderable.attachment_renderer_object else {
                continue;
            };

            // Load textures if they haven't been loaded already
            let spine_texture = unsafe { &mut *(attachment_renderer_object as *mut SpineTexture) };
            let texture = match spine_texture {
                SpineTexture::NeedsToBeLoaded(texture_to_load) => {
                    let texture = texture_to_load.load(&mut *self.ctx);
                    *spine_texture = SpineTexture::Loaded(texture);
                    texture
                }
                SpineTexture::Loaded(texture) => *texture,
            };
            bindings.images = vec![texture];
        }

        // Begin frame
        self.ctx
            .begin_default_pass(miniquad::PassAction::clear_color(0.1, 0.1, 0.1, 1.0));

        for (renderable, bindings) in renderables.into_iter().zip(self.bindings.iter_mut()) {
            // Apply the proper pipeline for this renderable
            let pipeline_key = PipelineKey {
                premultiplied_alpha: renderable.premultiplied_alpha,
                blend_mode: renderable.blend_mode,
                cull_face: self.spine.cull_face,
            };
            let pipeline = self
                .pipelines
                .get(&pipeline_key)
                .expect("Pipeline key not supported.");
            self.ctx.apply_pipeline(pipeline);

            // Draw this renderable
            self.ctx.apply_bindings(bindings);
            self.ctx
                .apply_uniforms(miniquad::UniformsSource::table(&shader::Uniforms {
                    mvp: view * self.spine.world,
                }));
            self.ctx.draw(0, renderable.indices.len() as i32, 1);
        }

        // End frame
        self.ctx.end_render_pass();
        self.egui.draw(&mut *self.ctx);
        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.screen_size = glam::Vec2::new(width, height) / miniquad::window::dpi_scale();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        self.egui.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        self.egui.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: miniquad::KeyMods, _repeat: bool) {
        self.egui.char_event(character);
    }

    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        repeat: bool,
    ) {
        if !repeat && keycode == miniquad::KeyCode::Space {
            self.current_spine_demo = (self.current_spine_demo + 1) % SPINE_DEMOS.len();
            self.spine = Spine::load(SPINE_DEMOS[self.current_spine_demo]);
        }
        self.egui.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.egui.key_up_event(keycode, keymods);
    }
}

fn main() {
    // These texture callbacks should be set before loading an atlas.
    rusty_spine::extension::set_create_texture_cb(|atlas_page, path| {
        atlas_page
            .renderer_object()
            .set(SpineTexture::NeedsToBeLoaded(SpineTextureToLoad {
                path: path.to_owned(),
                min_filter: atlas_page.min_filter(),
                mag_filter: atlas_page.mag_filter(),
                u_wrap: atlas_page.u_wrap(),
                v_wrap: atlas_page.v_wrap(),
                format: atlas_page.format(),
            }));
    });
    let (texture_delete_sender, texture_delete_receiver) = mpsc::channel();
    rusty_spine::extension::set_dispose_texture_cb(move |atlas_page| unsafe {
        if let Some(SpineTexture::Loaded(texture)) =
            atlas_page.renderer_object().get::<SpineTexture>()
        {
            _ = texture_delete_sender.send(*texture);
        }
        atlas_page.renderer_object().dispose::<SpineTexture>();
    });

    // This is the default behavior (y_down = false), it's only here for demonstration.
    // In this example, +Y is up. If your app has +Y as down, set this to true instead.
    Bone::set_y_down(false);

    let conf = miniquad::conf::Conf {
        window_title: "rusty_spine".to_owned(),
        high_dpi: true,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    };
    miniquad::start(conf, || Box::new(Stage::new(texture_delete_receiver)));
}

fn ui(egui_ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new())
        .show(egui_ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("press space for next demo")
                            .size(30.0)
                            .color(egui::Color32::WHITE),
                    )
                    .wrap_mode(egui::TextWrapMode::Extend)
                    .selectable(false),
                );
            });
        });
}

mod shader {
    pub const VERTEX: &str = r#"
        #version 100
        attribute vec2 position;
        attribute vec2 uv;
        attribute vec4 color;
        attribute vec4 dark_color;

        uniform mat4 mvp;

        varying lowp vec2 f_uv;
        varying lowp vec4 f_color;
        varying lowp vec4 f_dark_color;

        void main() {
            gl_Position = mvp * vec4(position, 0, 1);
            f_uv = uv;
            f_color = color;
            f_dark_color = dark_color;
        }
    "#;

    pub const FRAGMENT: &str = r#"
        #version 100
        varying lowp vec2 f_uv;
        varying lowp vec4 f_color;
        varying lowp vec4 f_dark_color;

        uniform sampler2D tex;

        void main() {
            lowp vec4 tex_color = texture2D(tex, f_uv);
            gl_FragColor = vec4(
                ((tex_color.a - 1.0) * f_dark_color.a + 1.0 - tex_color.rgb) * f_dark_color.rgb + tex_color.rgb * f_color.rgb,
                tex_color.a * f_color.a
            );
        }
    "#;

    pub const METAL: &str = r#"
        #include <metal_stdlib>

        using namespace metal;

        struct Uniforms
        {
            float4x4 mvp;
        };

        struct Vertex
        {
            float2 position   [[attribute(0)]];
            float2 uv         [[attribute(1)]];
            float4 color      [[attribute(2)]];
            float4 dark_color [[attribute(3)]];
        };

        struct RasterizerData
        {
            float4 position   [[position]];
            float2 uv         [[user(locn0)]];
            float4 color      [[user(locn1)]];
            float4 dark_color [[user(locn2)]];
        };

        vertex RasterizerData vertexShader(
          Vertex v [[stage_in]], 
          constant Uniforms& uniforms [[buffer(0)]])
        {
            RasterizerData out;

            out.position = uniforms.mvp * float4(v.position, 0.0, 1.0);
            out.uv = v.uv;
            out.color = v.color;
            out.dark_color = v.dark_color;

            return out;
        }

        fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
        {
            float4 tex_color = tex.sample(texSmplr, in.uv);
            return float4(
                ((tex_color.a - 1.0) * in.dark_color.a + 1.0 - tex_color.rgb) * in.dark_color.rgb + tex_color.rgb * in.color.rgb,
                tex_color.a * in.color.a
            );
        }
    "#;

    pub fn meta() -> miniquad::ShaderMeta {
        miniquad::ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: miniquad::UniformBlockLayout {
                uniforms: vec![miniquad::UniformDesc::new(
                    "mvp",
                    miniquad::UniformType::Mat4,
                )],
            },
        }
    }

    pub fn create(ctx: &mut dyn miniquad::RenderingBackend) -> miniquad::ShaderId {
        ctx.new_shader(
            match ctx.info().backend {
                miniquad::Backend::OpenGl => miniquad::ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                miniquad::Backend::Metal => miniquad::ShaderSource::Msl { program: METAL },
            },
            meta(),
        )
        .expect("Failed to build shader.")
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
