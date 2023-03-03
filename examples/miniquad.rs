use miniquad::*;
use rusty_spine::{controller::*, *};
use std::sync::Arc;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    last_frame: f64,
    pipeline: Pipeline,
    skeleton_controller: SkeletonController,
}

#[derive(Debug)]
enum SpineTexture {
    NeedsToBeLoaded(String),
    Loaded(Texture),
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        rusty_spine::extension::set_create_texture_cb(|atlas_page, path| {
            atlas_page
                .renderer_object()
                .set(SpineTexture::NeedsToBeLoaded(path.to_owned()));
        });
        rusty_spine::extension::set_dispose_texture_cb(|atlas_page| unsafe {
            atlas_page.renderer_object().dispose::<SpineTexture>();
        });

        let atlas_path = "assets/spineboy/export/spineboy-pma.atlas";
        let json_path = "assets/spineboy/export/spineboy-pro.json";
        let atlas = Arc::new(Atlas::new_from_file(atlas_path).unwrap());
        let skeleton_json = SkeletonJson::new(atlas);
        let skeleton_data = Arc::new(skeleton_json.read_skeleton_data_file(json_path).unwrap());
        let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
        let mut skeleton_controller =
            SkeletonController::new(skeleton_data.clone(), animation_state_data);
        let _ = skeleton_controller
            .animation_state
            .set_animation_by_name(0, "portal", true);

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );

        Stage {
            last_frame: date::now(),
            skeleton_controller,
            pipeline,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        let now = date::now();
        let dt = ((now - self.last_frame) as f32).max(0.001);
        self.skeleton_controller.update(dt);
        self.last_frame = now;
    }

    fn draw(&mut self, ctx: &mut Context) {
        let renderables = self.skeleton_controller.renderables();

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        for renderable in renderables.into_iter() {
            let mut vertices = vec![];
            for vertex_index in 0..renderable.vertices.len() {
                vertices.push(Vertex {
                    pos: Vec2 {
                        x: renderable.vertices[vertex_index][0] * 0.0015,
                        y: renderable.vertices[vertex_index][1] * 0.0015,
                    },
                    uv: Vec2 {
                        x: renderable.uvs[vertex_index][0],
                        y: renderable.uvs[vertex_index][1],
                    },
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
                offset: (0.1, -0.5),
            });
            ctx.draw(0, renderable.indices.len() as i32, 1);
        }
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}
