//! Demonstrates a complete Spine integration using `miniquad`.
//!
//! To integrate Spine into a project, the following features must be supported:
//!
//! # Texture Creation & Disposal
//!
//! Callbacks must be set to handle texture loading upon loading a [`rusty_spine::Atlas`].
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

use std::{fs::read, sync::Arc};

use glam::{Mat4, Vec2, Vec3};
use miniquad::*;
use rusty_spine::{
    atlas::{AtlasFilter, AtlasFormat, AtlasWrap},
    controller::{SkeletonController, SkeletonControllerSettings},
    draw::{ColorSpace, CullDirection},
    AnimationStateData, Atlas, BlendMode, Color, SkeletonBinary, SkeletonJson,
};

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
    last_frame_time: f64,
    screen_size: Vec2,
    demo_text: text::TextMesh,
}

impl Stage {
    fn new(ctx: &mut Context) -> Stage {
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
        let font_file = "assets/FiraMono-Medium.ttf";

        Stage {
            spine,
            spine_demos,
            current_spine_demo,
            pipeline: create_pipeline(ctx),
            last_frame_time: date::now(),
            screen_size: Vec2::new(800., 600.),
            demo_text: text::TextMesh::new(
                ctx,
                "Press space for next demo",
                &read(font_file).unwrap_or_else(|_| panic!("failed to load font: {font_file}")),
            ),
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

        // Begin frame
        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0.1, 0.1, 0.1, 1.)), None, None);
        ctx.apply_pipeline(&self.pipeline);

        // Apply backface culling only if this skeleton needs it
        ctx.set_cull_face(self.spine.cull_face);

        for renderable in renderables.into_iter() {
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
            let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
            let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &renderable.indices);

            // If there is no attachment (and therefore no texture), skip rendering this renderable
            let Some(attachment_renderer_object) = renderable.attachment_renderer_object else { continue };

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
                        TextureFormat::LuminanceAlpha => Texture::from_data_and_format(
                            ctx,
                            &image.to_luma_alpha8(),
                            texture_params,
                        ),
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

            // Draw this renderable
            ctx.apply_bindings(&Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![texture],
            });
            ctx.apply_uniforms(&shader::Uniforms {
                world: self.spine.world,
                view: self.view(),
            });
            ctx.draw(0, renderable.indices.len() as i32, 1);
        }

        // Draw demo text
        ctx.apply_pipeline(&self.pipeline);
        self.demo_text.draw(
            ctx,
            shader::Uniforms {
                world: Mat4::from_translation(Vec3::new(0., self.screen_size.y * 0.5 - 30., 0.)),
                view: self.view(),
            },
        );

        // End frame
        ctx.end_render_pass();
        ctx.commit_frame();
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.screen_size = Vec2::new(width, height);
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
                AtlasFormat::LuminanceAlpha => TextureFormat::LuminanceAlpha,
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
    rusty_spine::extension::set_dispose_texture_cb(|atlas_page| unsafe {
        atlas_page.renderer_object().dispose::<SpineTexture>();
    });
    let conf = conf::Conf {
        window_title: "rusty_spine".to_owned(),
        ..Default::default()
    };
    miniquad::start(conf, |ctx| Box::new(Stage::new(ctx)));
}

/// Not part of the demo, just necessary to render some text. Can probably be simplified.
mod text {
    use fontdue::{
        layout::{CoordinateSystem, Layout, TextStyle},
        Font,
    };
    use glam::Vec2;
    use miniquad::*;
    use rusty_spine::Color;

    use super::{shader, Vertex};

    pub struct TextMesh {
        pub bindings: Vec<Bindings>,
    }

    impl TextMesh {
        pub fn new(ctx: &mut Context, text: &'static str, ttf: &[u8]) -> Self {
            let font = Font::from_bytes(ttf, fontdue::FontSettings::default())
                .expect("failed to parse font");
            let mut layout = Layout::new(CoordinateSystem::PositiveYUp);
            layout.append(&[&font], &TextStyle::new(text, 25.0, 0));
            let mut bindings = vec![];
            let total_width = if let Some(last_glyph) = layout.glyphs().iter().last() {
                last_glyph.x + last_glyph.width as f32
            } else {
                0.
            };
            for glyph in layout.glyphs() {
                let (metrics, bitmap) = font.rasterize(glyph.parent, 25.0);
                if metrics.width * metrics.height > 0 {
                    let mut rgba: Vec<u8> = vec![];
                    for coverage in bitmap.into_iter() {
                        for _ in 0..4 {
                            rgba.push(coverage);
                        }
                    }
                    let texture = Texture::from_rgba8(
                        ctx,
                        metrics.width as u16,
                        metrics.height as u16,
                        &rgba,
                    );
                    let position = Vec2::new(glyph.x, glyph.y) - Vec2::new(total_width * 0.5, 0.);
                    let size = Vec2::new(glyph.width as f32, glyph.height as f32);
                    let vertices = [
                        Vertex {
                            position: position + Vec2::new(0., 0.) * size,
                            uv: Vec2::new(0., 1.),
                            color: Color::new_rgba(1., 1., 1., 1.),
                            dark_color: Color::new_rgba(0., 0., 0., 1.),
                        },
                        Vertex {
                            position: position + Vec2::new(0., 1.) * size,
                            uv: Vec2::new(0., 0.),
                            color: Color::new_rgba(1., 1., 1., 1.),
                            dark_color: Color::new_rgba(0., 0., 0., 1.),
                        },
                        Vertex {
                            position: position + Vec2::new(1., 1.) * size,
                            uv: Vec2::new(1., 0.),
                            color: Color::new_rgba(1., 1., 1., 1.),
                            dark_color: Color::new_rgba(0., 0., 0., 1.),
                        },
                        Vertex {
                            position: position + Vec2::new(1., 0.) * size,
                            uv: Vec2::new(1., 1.),
                            color: Color::new_rgba(1., 1., 1., 1.),
                            dark_color: Color::new_rgba(0., 0., 0., 1.),
                        },
                    ];
                    let indices = vec![0, 2, 1, 0, 3, 2];
                    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
                    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
                    bindings.push(Bindings {
                        vertex_buffers: vec![vertex_buffer],
                        index_buffer,
                        images: vec![texture],
                    });
                }
            }
            Self { bindings }
        }

        pub fn draw(&self, ctx: &mut Context, uniforms: shader::Uniforms) {
            ctx.set_blend(
                Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
            );
            for bindings in self.bindings.iter() {
                ctx.apply_bindings(bindings);
                ctx.apply_uniforms(&uniforms);
                ctx.draw(0, 6, 1);
            }
        }
    }
}
