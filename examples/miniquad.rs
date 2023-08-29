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
//! This example does not handle all cases and will print a warning if encountering unsupported
//! settings. Most notably, mipmap textures are not supported.
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
//! backface culling is required, see the coin in this example. Disabling
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
//! hooked up to the audio system to play sounds. This example does not support this (although
//! events will be printed to console).

use std::sync::{Arc, Mutex};

use glam::{Mat4, Vec2, Vec3};
use miniquad::*;
use rusty_spine::{
    atlas::{AtlasFilter, AtlasFormat, AtlasWrap},
    controller::{SkeletonController, SkeletonControllerSettings},
    draw::{ColorSpace, CullDirection},
    AnimationEvent, AnimationStateData, Atlas, BlendMode, Color, SkeletonBinary, SkeletonJson,
};

const MAX_MESH_VERTICES: usize = 10000;
const MAX_MESH_INDICES: usize = 5000;

mod shader {
    use glam::Mat4;
    use miniquad::*;

    pub const VERTEX: &str = r#"
        #version 100
        attribute vec2 position;
        attribute vec2 uv;
        attribute vec4 color;
        attribute vec4 dark_color;

        uniform mat4 world;
        uniform mat4 view;

        varying lowp vec2 f_texcoord;
        varying lowp vec4 f_color;
        varying lowp vec4 f_dark_color;

        void main() {
            gl_Position = view * world * vec4(position, 0, 1);
            f_texcoord = uv;
            f_color = color;
            f_dark_color = dark_color;
        }
    "#;

    pub const FRAGMENT: &str = r#"
        #version 100
        varying lowp vec2 f_texcoord;
        varying lowp vec4 f_color;
        varying lowp vec4 f_dark_color;

        uniform sampler2D tex;

        void main() {
            lowp vec4 tex_color = texture2D(tex, f_texcoord);
            gl_FragColor = vec4(
                ((tex_color.a - 1.0) * f_dark_color.a + 1.0 - tex_color.rgb) * f_dark_color.rgb + tex_color.rgb * f_color.rgb,
                tex_color.a * f_color.a
            );
        }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("world", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub world: Mat4,
        pub view: Mat4,
    }
}

#[repr(C)]
struct Vertex {
    position: Vec2,
    uv: Vec2,
    color: Color,
    dark_color: Color,
}

fn create_pipeline(ctx: &mut Context) -> Pipeline {
    let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta())
        .expect("failed to build shader");
    Pipeline::new(
        ctx,
        &[BufferLayout::default()],
        &[
            VertexAttribute::new("position", VertexFormat::Float2),
            VertexAttribute::new("uv", VertexFormat::Float2),
            VertexAttribute::new("color", VertexFormat::Float4),
            VertexAttribute::new("dark_color", VertexFormat::Float4),
        ],
        shader,
    )
}

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
    NeedsToBeLoaded {
        path: String,
        min_filter: FilterMode,
        mag_filter: FilterMode,
        x_wrap: TextureWrap,
        y_wrap: TextureWrap,
        format: TextureFormat,
    },
    Loaded(Texture),
}

/// Holds all data related to load and demonstrate a particular Spine skeleton.
#[derive(Clone, Copy)]
struct SpineDemo {
    atlas_path: &'static str,
    skeleton_path: SpineSkeletonPath,
    animation: &'static str,
    position: Vec2,
    scale: f32,
    skin: Option<&'static str>,
    backface_culling: bool,
}

#[derive(Clone, Copy)]
enum SpineSkeletonPath {
    Binary(&'static str),
    Json(&'static str),
}

/// Holds all data related to rendering Spine skeletons in this example.
struct Spine {
    controller: SkeletonController,
    world: Mat4,
    cull_face: CullFace,
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
            world: Mat4::from_translation(info.position.extend(0.))
                * Mat4::from_scale(Vec2::splat(info.scale).extend(1.)),
            cull_face: match info.backface_culling {
                false => CullFace::Nothing,
                true => CullFace::Back,
            },
        }
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
    alpha_blend: BlendState,
    color_blend: BlendState,
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
                    alpha_blend: BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::One,
                    ),
                },
                // Case 2: Additive Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
                    color_blend: BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
                },
            },
            Self::Multiply => match premultiplied_alpha {
                // Case 3: Multiply Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::DestinationColor),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
                // Case 4: Multiply Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::DestinationColor),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
            },
            Self::Normal => match premultiplied_alpha {
                // Case 5: Normal Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
                // Case 6: Normal Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
            },
            Self::Screen => match premultiplied_alpha {
                // Case 7: Screen Blend Mode, Normal Alpha
                false => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::OneMinusValue(BlendValue::SourceColor),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
                // Case 8: Screen Blend Mode, Premultiplied Alpha
                true => BlendStates {
                    alpha_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::OneMinusValue(BlendValue::SourceColor),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                    color_blend: BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    ),
                },
            },
        }
    }
}

struct Stage {
    spine: Spine,
    spine_demos: Vec<SpineDemo>,
    current_spine_demo: usize,
    pipeline: Pipeline,
    bindings: Vec<Bindings>,
    texture_delete_queue: Arc<Mutex<Vec<Texture>>>,
    last_frame_time: f64,
    screen_size: Vec2,
    demo_text: text::Text,
}

impl Stage {
    fn new(ctx: &mut Context, texture_delete_queue: Arc<Mutex<Vec<Texture>>>) -> Stage {
        let spine_demos = vec![
            SpineDemo {
                atlas_path: "assets/spineboy/export/spineboy-pma.atlas",
                skeleton_path: SpineSkeletonPath::Binary(
                    "assets/spineboy/export/spineboy-pro.skel",
                ),
                animation: "portal",
                position: Vec2::new(0., -220.),
                scale: 0.5,
                skin: None,
                backface_culling: true,
            },
            SpineDemo {
                atlas_path: "assets/windmill/export/windmill.atlas",
                skeleton_path: SpineSkeletonPath::Json("assets/windmill/export/windmill-ess.json"),
                animation: "animation",
                position: Vec2::new(0., -80.),
                scale: 0.5,
                skin: None,
                backface_culling: true,
            },
            SpineDemo {
                atlas_path: "assets/alien/export/alien.atlas",
                skeleton_path: SpineSkeletonPath::Json("assets/alien/export/alien-pro.json"),
                animation: "death",
                position: Vec2::new(0., -260.),
                scale: 0.3,
                skin: None,
                backface_culling: true,
            },
            SpineDemo {
                atlas_path: "assets/dragon/export/dragon.atlas",
                skeleton_path: SpineSkeletonPath::Json("assets/dragon/export/dragon-ess.json"),
                animation: "flying",
                position: Vec2::new(0., -50.),
                scale: 0.7,
                skin: None,
                backface_culling: true,
            },
            SpineDemo {
                atlas_path: "assets/goblins/export/goblins.atlas",
                skeleton_path: SpineSkeletonPath::Json("assets/goblins/export/goblins-pro.json"),
                animation: "walk",
                position: Vec2::new(0., -200.),
                scale: 1.,
                skin: Some("goblingirl"),
                backface_culling: true,
            },
            SpineDemo {
                atlas_path: "assets/coin/export/coin-pma.atlas",
                skeleton_path: SpineSkeletonPath::Json("assets/coin/export/coin-pro.json"),
                animation: "animation",
                position: Vec2::ZERO,
                scale: 1.,
                skin: None,
                backface_culling: false,
            },
        ];

        let current_spine_demo = 0;
        let spine = Spine::load(spine_demos[current_spine_demo]);

        let mut text_system = text::TextSystem::new();
        let demo_text =
            text_system.create_text(ctx, "press space for next demo", 32. * ctx.dpi_scale());

        Stage {
            spine,
            spine_demos,
            current_spine_demo,
            pipeline: create_pipeline(ctx),
            bindings: vec![],
            texture_delete_queue,
            last_frame_time: date::now(),
            screen_size: Vec2::new(800., 600.),
            demo_text,
        }
    }

    fn view(&self) -> Mat4 {
        Mat4::orthographic_rh_gl(
            self.screen_size.x * -0.5,
            self.screen_size.x * 0.5,
            self.screen_size.y * -0.5,
            self.screen_size.y * 0.5,
            0.,
            1.,
        )
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        let now = date::now();
        let dt = ((now - self.last_frame_time) as f32).max(0.001);
        self.spine.controller.update(dt);
        self.last_frame_time = now;
    }

    fn draw(&mut self, ctx: &mut Context) {
        let renderables = self.spine.controller.combined_renderables();

        // Create bindings that can be re-used for rendering Spine meshes
        while renderables.len() > self.bindings.len() {
            let vertex_buffer = Buffer::stream(
                ctx,
                BufferType::VertexBuffer,
                MAX_MESH_VERTICES * std::mem::size_of::<Vertex>(),
            );
            let index_buffer = Buffer::stream(
                ctx,
                BufferType::IndexBuffer,
                MAX_MESH_INDICES * std::mem::size_of::<u16>(),
            );
            self.bindings.push(Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![Texture::empty()],
            });
        }

        // Delete textures that are no longer used. The delete call needs to happen here, before
        // rendering, or it may not actually delete the texture.
        for texture_delete in self.texture_delete_queue.lock().unwrap().drain(..) {
            texture_delete.delete();
        }

        // Begin frame
        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0.1, 0.1, 0.1, 1.)), None, None);
        ctx.apply_pipeline(&self.pipeline);

        // Apply backface culling only if this skeleton needs it
        ctx.set_cull_face(self.spine.cull_face);

        let view = self.view();
        for (renderable, bindings) in renderables.into_iter().zip(self.bindings.iter_mut()) {
            // Set blend state based on this renderable's blend mode
            let BlendStates {
                alpha_blend,
                color_blend,
            } = renderable
                .blend_mode
                .get_blend_states(self.spine.controller.settings.premultiplied_alpha);
            ctx.set_blend(Some(color_blend), Some(alpha_blend));

            // Create the vertex and index buffers for miniquad
            let mut vertices = vec![];
            for vertex_index in 0..renderable.vertices.len() {
                vertices.push(Vertex {
                    position: Vec2 {
                        x: renderable.vertices[vertex_index][0],
                        y: renderable.vertices[vertex_index][1],
                    },
                    uv: Vec2 {
                        x: renderable.uvs[vertex_index][0],
                        y: renderable.uvs[vertex_index][1],
                    },
                    color: Color::from(renderable.colors[vertex_index]),
                    dark_color: Color::from(renderable.dark_colors[vertex_index]),
                });
            }
            bindings.vertex_buffers[0].update(ctx, &vertices);
            bindings.index_buffer.update(ctx, &renderable.indices);

            // If there is no attachment (and therefore no texture), skip rendering this renderable
            // May also be None if a create texture callback was never set.
            let Some(attachment_renderer_object) = renderable.attachment_renderer_object else {
                continue;
            };

            // Load textures if they haven't been loaded already
            let spine_texture = unsafe { &mut *(attachment_renderer_object as *mut SpineTexture) };
            let texture = match spine_texture {
                SpineTexture::NeedsToBeLoaded {
                    path,
                    min_filter,
                    mag_filter,
                    x_wrap,
                    y_wrap,
                    format,
                } => {
                    use image::io::Reader as ImageReader;
                    let image = ImageReader::open(&path)
                        .unwrap_or_else(|_| panic!("failed to open image: {}", &path))
                        .decode()
                        .unwrap_or_else(|_| panic!("failed to decode image: {}", &path));
                    let texture_params = TextureParams {
                        width: image.width(),
                        height: image.height(),
                        format: *format,
                        ..Default::default()
                    };
                    let texture = match format {
                        TextureFormat::RGB8 => {
                            Texture::from_data_and_format(ctx, &image.to_rgb8(), texture_params)
                        }
                        TextureFormat::RGBA8 => {
                            Texture::from_data_and_format(ctx, &image.to_rgba8(), texture_params)
                        }
                        _ => unreachable!(),
                    };
                    texture.set_filter_min_mag(ctx, *min_filter, *mag_filter);
                    texture.set_wrap_xy(ctx, *x_wrap, *y_wrap);
                    *spine_texture = SpineTexture::Loaded(texture);
                    texture
                }
                SpineTexture::Loaded(texture) => *texture,
            };
            bindings.images = vec![texture];

            // Draw this renderable
            ctx.apply_bindings(bindings);
            ctx.apply_uniforms(&shader::Uniforms {
                world: self.spine.world,
                view,
            });
            ctx.draw(0, renderable.indices.len() as i32, 1);
        }

        // Draw demo text
        let BlendStates {
            alpha_blend,
            color_blend,
        } = BlendMode::Normal.get_blend_states(true);
        ctx.set_blend(Some(color_blend), Some(alpha_blend));
        ctx.apply_bindings(&self.demo_text.bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            world: Mat4::from_translation(Vec3::new(0., self.screen_size.y * 0.48, 0.))
                * Mat4::from_scale(self.demo_text.size.extend(0.) / ctx.dpi_scale()),
            view,
        });
        ctx.draw(0, self.demo_text.num_elements, 1);

        // End frame
        ctx.end_render_pass();
        ctx.commit_frame();
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.screen_size = Vec2::new(width, height) / ctx.dpi_scale();
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if !repeat && keycode == KeyCode::Space {
            self.current_spine_demo = (self.current_spine_demo + 1) % self.spine_demos.len();
            self.spine = Spine::load(self.spine_demos[self.current_spine_demo]);
        }
    }
}

fn main() {
    // These texture callbacks should be set before loading an atlas.
    rusty_spine::extension::set_create_texture_cb(|atlas_page, path| {
        fn convert_filter(filter: AtlasFilter) -> FilterMode {
            match filter {
                AtlasFilter::Linear => FilterMode::Linear,
                AtlasFilter::Nearest => FilterMode::Nearest,
                filter => {
                    println!("Unsupported texture filter mode: {filter:?}");
                    FilterMode::Linear
                }
            }
        }
        fn convert_wrap(wrap: AtlasWrap) -> TextureWrap {
            match wrap {
                AtlasWrap::ClampToEdge => TextureWrap::Clamp,
                AtlasWrap::MirroredRepeat => TextureWrap::Mirror,
                AtlasWrap::Repeat => TextureWrap::Repeat,
                wrap => {
                    println!("Unsupported texture wrap mode: {wrap:?}");
                    TextureWrap::Clamp
                }
            }
        }
        fn convert_format(format: AtlasFormat) -> TextureFormat {
            match format {
                AtlasFormat::RGB888 => TextureFormat::RGB8,
                AtlasFormat::RGBA8888 => TextureFormat::RGBA8,
                format => {
                    println!("Unsupported texture format: {format:?}");
                    TextureFormat::RGBA8
                }
            }
        }
        atlas_page
            .renderer_object()
            .set(SpineTexture::NeedsToBeLoaded {
                path: path.to_owned(),
                min_filter: convert_filter(atlas_page.min_filter()),
                mag_filter: convert_filter(atlas_page.mag_filter()),
                x_wrap: convert_wrap(atlas_page.u_wrap()),
                y_wrap: convert_wrap(atlas_page.v_wrap()),
                format: convert_format(atlas_page.format()),
            });
    });
    let texture_delete_queue: Arc<Mutex<Vec<Texture>>> = Arc::new(Mutex::new(vec![]));
    let texture_delete_queue_cb = texture_delete_queue.clone();
    rusty_spine::extension::set_dispose_texture_cb(move |atlas_page| unsafe {
        if let Some(SpineTexture::Loaded(texture)) =
            atlas_page.renderer_object().get::<SpineTexture>()
        {
            texture_delete_queue_cb.lock().unwrap().push(*texture);
        }
        atlas_page.renderer_object().dispose::<SpineTexture>();
    });
    let conf = conf::Conf {
        window_title: "rusty_spine".to_owned(),
        high_dpi: true,
        ..Default::default()
    };
    miniquad::start(conf, |ctx| Box::new(Stage::new(ctx, texture_delete_queue)));
}

/// Not part of the demo, just necessary to render some text.
mod text {
    use cosmic_text::{Attrs, FontSystem, Metrics, Shaping, SwashCache, Wrap};
    use glam::Vec2;
    use miniquad::*;
    use rusty_spine::Color;

    use super::Vertex;

    pub struct Text {
        pub bindings: Bindings,
        pub num_elements: i32,
        pub size: Vec2,
    }

    pub struct TextSystem {
        pub font_system: FontSystem,
        pub swash_cache: SwashCache,
    }

    impl TextSystem {
        pub fn new() -> Self {
            Self {
                font_system: FontSystem::new(),
                swash_cache: SwashCache::new(),
            }
        }

        pub fn create_text(&mut self, ctx: &mut Context, text: &str, size: f32) -> Text {
            let metrics = Metrics::new(size, size);
            let mut buffer = cosmic_text::Buffer::new(&mut self.font_system, metrics);
            {
                let mut buffer = buffer.borrow_with(&mut self.font_system);
                buffer.set_wrap(Wrap::None);
                buffer.set_size(f32::MAX, f32::MAX);
                buffer.set_text(text, Attrs::new(), Shaping::Basic);
                buffer.shape_until_scroll();
            }
            let mut width = 1_usize;
            let mut height = 1_usize;
            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    width = width.max((run.line_w + glyph.w) as usize);
                }
                height = height.max((run.line_y + size) as usize);
            }
            let mut texture_data = vec![0; width * height * 4];
            buffer.draw(
                &mut self.font_system,
                &mut self.swash_cache,
                cosmic_text::Color::rgb(255, 255, 255),
                |x, y, h, w, color| {
                    if x < 0 || y < 0 {
                        return;
                    }
                    let a = color.a() as f32 / 255.;
                    let (r, g, b) = if a != 0. {
                        (
                            (color.r() as f32 / 255.) * a,
                            (color.g() as f32 / 255.) * a,
                            (color.b() as f32 / 255.) * a,
                        )
                    } else {
                        (0., 0., 0.)
                    };
                    let (x, y, w, h) = (x as usize, y as usize, w as usize, h as usize);
                    for xx in x..(x + w) {
                        for yy in y..(y + h) {
                            if xx < width && y < height {
                                let index = xx + yy * width;
                                texture_data[index * 4] = (r * 255.) as u8;
                                texture_data[index * 4 + 1] = (g * 255.) as u8;
                                texture_data[index * 4 + 2] = (b * 255.) as u8;
                                texture_data[index * 4 + 3] = color.a();
                            }
                        }
                    }
                },
            );
            let vertex_buffers = Buffer::immutable(
                ctx,
                BufferType::VertexBuffer,
                &[
                    Vertex {
                        position: Vec2::new(-0.5, -1.),
                        uv: Vec2::new(0., 1.),
                        color: Color::new_rgba(1., 1., 1., 1.),
                        dark_color: Color::new_rgba(0., 0., 0., 1.),
                    },
                    Vertex {
                        position: Vec2::new(0.5, -1.),
                        uv: Vec2::new(1., 1.),
                        color: Color::new_rgba(1., 1., 1., 1.),
                        dark_color: Color::new_rgba(0., 0., 0., 1.),
                    },
                    Vertex {
                        position: Vec2::new(-0.5, 0.),
                        uv: Vec2::new(0., 0.),
                        color: Color::new_rgba(1., 1., 1., 1.),
                        dark_color: Color::new_rgba(0., 0., 0., 1.),
                    },
                    Vertex {
                        position: Vec2::new(0.5, 0.),
                        uv: Vec2::new(1., 0.),
                        color: Color::new_rgba(1., 1., 1., 1.),
                        dark_color: Color::new_rgba(0., 0., 0., 1.),
                    },
                ],
            );
            let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &[0, 1, 2, 1, 3, 2]);
            let texture = Texture::from_rgba8(ctx, width as u16, height as u16, &texture_data);
            Text {
                bindings: Bindings {
                    vertex_buffers: vec![vertex_buffers],
                    index_buffer,
                    images: vec![texture],
                },
                num_elements: 6,
                size: Vec2::new(width as f32, height as f32),
            }
        }
    }
}
