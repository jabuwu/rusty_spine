use std::sync::Arc;

use enum_map::{enum_map, Enum, EnumMap};
use glam::{Mat4, Vec2};
use miniquad::*;
use rusty_spine::{controller::*, *};

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
    color: Color,
    dark_color: Color,
}

#[derive(Debug)]
enum SpineTexture {
    NeedsToBeLoaded(String),
    Loaded(Texture),
}

#[derive(Clone, Copy)]
struct SpineDemo {
    atlas_path: &'static str,
    json_path: &'static str,
    animation: &'static str,
    position: Vec2,
    scale: f32,
    skin: Option<&'static str>,
}

struct Spine {
    controller: SkeletonController,
    world: Mat4,
    premultiplied_alpha: bool,
}

impl Spine {
    pub fn load(info: SpineDemo) -> Self {
        let atlas = Arc::new(Atlas::new_from_file(info.atlas_path).unwrap());
        let mut premultiplied_alpha = false;
        for page in atlas.pages() {
            if page.pma() {
                premultiplied_alpha = true;
            }
        }
        let skeleton_json = SkeletonJson::new(atlas);
        let skeleton_data = Arc::new(
            skeleton_json
                .read_skeleton_data_file(info.json_path)
                .unwrap(),
        );
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        let mut controller = SkeletonController::new(skeleton_data.clone(), animation_state_data);
        controller
            .animation_state
            .set_animation_by_name(0, info.animation, true)
            .expect(&format!("failed to start animation: {}", info.animation));
        if let Some(skin) = info.skin {
            controller
                .skeleton
                .set_skin_by_name(skin)
                .expect(&format!("failed to set skin: {}", skin));
        }
        Self {
            controller,
            world: Mat4::from_translation(info.position.extend(0.))
                * Mat4::from_scale(Vec2::splat(info.scale).extend(1.)),
            premultiplied_alpha,
        }
    }
}

#[derive(Enum)]
enum Pipelines {
    Additive,
    Multiply,
    Normal,
    Screen,
    AdditivePma,
    MultiplyPma,
    NormalPma,
    ScreenPma,
}

impl Pipelines {
    fn from_pma_blend_mode(premultiplied_alpha: bool, blend_mode: BlendMode) -> Self {
        match premultiplied_alpha {
            false => match blend_mode {
                BlendMode::Additive => Self::Additive,
                BlendMode::Multiply => Self::Multiply,
                BlendMode::Normal => Self::Normal,
                BlendMode::Screen => Self::Screen,
            },
            true => match blend_mode {
                BlendMode::Additive => Self::AdditivePma,
                BlendMode::Multiply => Self::MultiplyPma,
                BlendMode::Normal => Self::NormalPma,
                BlendMode::Screen => Self::ScreenPma,
            },
        }
    }
}

struct Stage {
    spine: Spine,
    spine_demos: Vec<SpineDemo>,
    current_spine_demo: usize,
    pipelines: EnumMap<Pipelines, Pipeline>,
    last_frame: f64,
    screen_size: Vec2,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let params_map = enum_map! {
            Pipelines::Additive => (
                BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::One,
                ),
            ),
            Pipelines::Multiply => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::DestinationColor),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            ),
            Pipelines::Normal => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            ),
            Pipelines::Screen => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::OneMinusValue(BlendValue::SourceColor),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            ),
            Pipelines::AdditivePma => (
                BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
                BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One),
            ),
            Pipelines::MultiplyPma => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::DestinationColor),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            ),
            Pipelines::NormalPma => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            ),
            Pipelines::ScreenPma => (
                BlendState::new(
                    Equation::Add,
                    BlendFactor::OneMinusValue(BlendValue::SourceColor),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
                BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                ),
            )
        };

        let pipelines = params_map.map(|_, (alpha_blend, color_blend)| {
            let shader =
                Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();
            Pipeline::with_params(
                ctx,
                &[BufferLayout::default()],
                &[
                    VertexAttribute::new("pos", VertexFormat::Float2),
                    VertexAttribute::new("uv", VertexFormat::Float2),
                    VertexAttribute::new("color", VertexFormat::Float4),
                    VertexAttribute::new("dark_color", VertexFormat::Float4),
                ],
                shader,
                PipelineParams {
                    alpha_blend: Some(alpha_blend),
                    color_blend: Some(color_blend),
                    ..Default::default()
                },
            )
        });

        let spine_demos = vec![
            SpineDemo {
                atlas_path: "assets/spineboy/export/spineboy-pma.atlas",
                json_path: "assets/spineboy/export/spineboy-pro.json",
                animation: "portal",
                position: Vec2::new(0., -220.),
                scale: 0.5,
                skin: None,
            },
            SpineDemo {
                atlas_path: "assets/windmill/export/windmill.atlas",
                json_path: "assets/windmill/export/windmill-ess.json",
                animation: "animation",
                position: Vec2::new(0., -80.),
                scale: 0.5,
                skin: None,
            },
            SpineDemo {
                atlas_path: "assets/alien/export/alien.atlas",
                json_path: "assets/alien/export/alien-pro.json",
                animation: "death",
                position: Vec2::new(0., -260.),
                scale: 0.3,
                skin: None,
            },
            SpineDemo {
                atlas_path: "assets/dragon/export/dragon.atlas",
                json_path: "assets/dragon/export/dragon-ess.json",
                animation: "flying",
                position: Vec2::new(0., -50.),
                scale: 0.7,
                skin: None,
            },
            SpineDemo {
                atlas_path: "assets/goblins/export/goblins.atlas",
                json_path: "assets/goblins/export/goblins-pro.json",
                animation: "walk",
                position: Vec2::new(0., -200.),
                scale: 1.,
                skin: Some("goblingirl"),
            },
            SpineDemo {
                atlas_path: "assets/coin/export/coin.atlas",
                json_path: "assets/coin/export/coin-pro.json",
                animation: "animation",
                position: Vec2::ZERO,
                scale: 1.,
                skin: None,
            },
        ];
        let current_spine_demo = 0;
        let spine = Spine::load(spine_demos[current_spine_demo]);

        Stage {
            spine,
            spine_demos,
            current_spine_demo,
            pipelines,
            last_frame: date::now(),
            screen_size: Vec2::new(800., 600.),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        let now = date::now();
        let dt = ((now - self.last_frame) as f32).max(0.001);
        self.spine.controller.update(dt);
        self.last_frame = now;
    }

    fn draw(&mut self, ctx: &mut Context) {
        let renderables = self.spine.controller.combined_renderables();

        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0., 0., 0., 1.)), None, None);

        for renderable in renderables.into_iter() {
            ctx.apply_pipeline(
                &self.pipelines[Pipelines::from_pma_blend_mode(
                    self.spine.premultiplied_alpha,
                    renderable.blend_mode,
                )],
            );
            let mut vertices = vec![];
            for vertex_index in 0..renderable.vertices.len() {
                vertices.push(Vertex {
                    pos: Vec2 {
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

            let Some(attachment_renderer_object) = renderable.attachment_renderer_object else { continue };
            let spine_texture = unsafe { &mut *(attachment_renderer_object as *mut SpineTexture) };
            let texture = match spine_texture {
                SpineTexture::NeedsToBeLoaded(path) => {
                    use image::io::Reader as ImageReader;
                    let image = ImageReader::open(path)
                        .unwrap()
                        .decode()
                        .unwrap()
                        .to_rgba8();
                    let texture = Texture::from_rgba8(
                        ctx,
                        image.width() as u16,
                        image.height() as u16,
                        &image.to_vec(),
                    );
                    *spine_texture = SpineTexture::Loaded(texture.clone());
                    texture
                }
                SpineTexture::Loaded(texture) => texture.clone(),
            };
            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![texture],
            };

            ctx.apply_bindings(&bindings);
            ctx.apply_uniforms(&shader::Uniforms {
                world: self.spine.world,
                view: Mat4::orthographic_rh_gl(
                    self.screen_size.x * -0.5,
                    self.screen_size.x * 0.5,
                    self.screen_size.y * -0.5,
                    self.screen_size.y * 0.5,
                    0.,
                    1.,
                ),
            });
            ctx.draw(0, renderable.indices.len() as i32, 1);
        }
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
        atlas_page
            .renderer_object()
            .set(SpineTexture::NeedsToBeLoaded(path.to_owned()));
    });
    rusty_spine::extension::set_dispose_texture_cb(|atlas_page| unsafe {
        atlas_page.renderer_object().dispose::<SpineTexture>();
    });
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}

mod shader {
    use glam::Mat4;
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    attribute vec4 color;
    attribute vec4 dark_color;

    uniform mat4 world;
    uniform mat4 view;

    varying lowp vec2 f_texcoord;
    varying lowp vec4 f_color;
    varying lowp vec4 f_dark_color;

    void main() {
        gl_Position = view * world * vec4(pos, 0, 1);
        f_texcoord = uv;
        f_color = color;
        f_dark_color = dark_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
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
    }"#;

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
