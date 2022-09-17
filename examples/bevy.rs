use std::{collections::HashMap, ffi::CString, mem::take, sync::Arc};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use rusty_spine::{
    animation_state_data::AnimationStateData,
    atlas::Atlas,
    c::spSkeleton_setSkinByName,
    error::Error,
    skeleton_controller::{CullDirection, SkeletonController, SkeletonControllerSettings},
    skeleton_json::SkeletonJson,
};

#[derive(Component)]
pub struct Spine {
    controller: SkeletonController,
}

#[derive(Debug)]
struct SpineTexture {
    path: String,
}

struct Demo {
    atlas: Vec<u8>,
    json: Vec<u8>,
    dir: String,
    animation: String,
    position: Vec2,
    scale: f32,
    skin: Option<String>,
}

struct Demos(Vec<Demo>);

#[derive(Clone)]
struct DemoLoad(usize);

fn make_cube(mesh: &mut Mesh) {
    let indices = Indices::U32(vec![]);

    let positions: Vec<[f32; 3]> = vec![];
    let normals: Vec<[f32; 3]> = vec![];
    let uvs: Vec<[f32; 2]> = vec![];

    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
}

fn main() {
    rusty_spine::extension::set_create_texture_cb(|page, path| {
        page.renderer_object().set(SpineTexture {
            path: path.to_owned(),
        });
    });
    rusty_spine::extension::set_dispose_texture_cb(|page| unsafe {
        page.renderer_object().dispose::<SpineTexture>();
    });
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Demos(vec![
            Demo {
                atlas: include_bytes!("../assets/spineboy/export/spineboy.atlas").to_vec(),
                json: include_bytes!("../assets/spineboy/export/spineboy-pro.json").to_vec(),
                dir: "spineboy/export/".to_owned(),
                animation: "portal".to_owned(),
                position: Vec2::new(0., -400.),
                scale: 0.5,
                skin: None,
            },
            Demo {
                atlas: include_bytes!("../assets/spineboy/export/spineboy.atlas").to_vec(),
                json: include_bytes!("../assets/spineboy/export/spineboy-pro.json").to_vec(),
                dir: "spineboy/export/".to_owned(),
                animation: "hoverboard".to_owned(),
                position: Vec2::new(0., -400.),
                scale: 0.5,
                skin: None,
            },
            Demo {
                atlas: include_bytes!("../assets/windmill/export/windmill.atlas").to_vec(),
                json: include_bytes!("../assets/windmill/export/windmill-ess.json").to_vec(),
                dir: "windmill/export/".to_owned(),
                animation: "animation".to_owned(),
                position: Vec2::new(0., -150.),
                scale: 0.5,
                skin: None,
            },
            Demo {
                atlas: include_bytes!("../assets/alien/export/alien.atlas").to_vec(),
                json: include_bytes!("../assets/alien/export/alien-pro.json").to_vec(),
                dir: "alien/export/".to_owned(),
                animation: "death".to_owned(),
                position: Vec2::new(0., -600.),
                scale: 0.25,
                skin: None,
            },
            Demo {
                atlas: include_bytes!("../assets/coin/export/coin.atlas").to_vec(),
                json: include_bytes!("../assets/coin/export/coin-pro.json").to_vec(),
                dir: "coin/export/".to_owned(),
                animation: "animation".to_owned(),
                position: Vec2::new(0., 0.),
                scale: 0.75,
                skin: None,
            },
            /*TODO: figure out why dragon crashes - Demo {
                atlas: "assets/dragon/export/dragon.atlas".to_owned(),
                json: "assets/dragon/export/dragon-ess.json".to_owned(),
                dir: "dragon/export/".to_owned(),
                animation: "flying".to_owned(),
                position: Vec2::new(0., 0.),
                scale: 0.75,
            },*/
            Demo {
                atlas: include_bytes!("../assets/goblins/export/goblins.atlas").to_vec(),
                json: include_bytes!("../assets/goblins/export/goblins-pro.json").to_vec(),
                dir: "goblins/export/".to_owned(),
                animation: "walk".to_owned(),
                position: Vec2::new(0., -200.),
                scale: 1.,
                skin: Some("goblingirl".to_owned()),
            },
            Demo {
                atlas: include_bytes!("../assets/stretchyman/export/stretchyman.atlas").to_vec(),
                json: include_bytes!("../assets/stretchyman/export/stretchyman-pro.json").to_vec(),
                dir: "stretchyman/export/".to_owned(),
                animation: "sneak".to_owned(),
                position: Vec2::new(-700., -250.),
                scale: 0.75,
                skin: None,
            },
            Demo {
                atlas: include_bytes!("../assets/tank/export/tank.atlas").to_vec(),
                json: include_bytes!("../assets/tank/export/tank-pro.json").to_vec(),
                dir: "tank/export/".to_owned(),
                animation: "drive".to_owned(),
                position: Vec2::new(3500., -850.),
                scale: 0.3,
                skin: None,
            },
        ]))
        .add_event::<DemoLoad>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(demo_load)
        .add_system(demo_next)
        .add_system(spine_update)
        .run();
}

fn startup(mut commands: Commands, mut ev_demo_load: EventWriter<DemoLoad>) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_demo_load.send(DemoLoad(0));
}

fn demo_load(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_demo_load: EventReader<DemoLoad>,
    entity_query: Query<Entity, Without<Camera>>,
    demos: Res<Demos>,
) {
    for event in ev_demo_load.iter() {
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }
        let demo = &demos.0[event.0];
        let mut controller = load_skeleton(&demo.atlas, &demo.json, &demo.dir).unwrap();
        controller
            .animation_state
            .set_animation_by_name(0, &demo.animation, true);
        if let Some(skin) = &demo.skin {
            unsafe {
                let c_skin = CString::new(skin.as_str()).unwrap();
                spSkeleton_setSkinByName(controller.skeleton.c_ptr(), c_skin.as_ptr());
            }
        }
        let mut slots = HashMap::new();
        commands
            .spawn_bundle((
                Transform::from_scale(Vec3::ONE * demo.scale),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .with_children(|parent| {
                for slot in controller.skeleton.slots() {
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                    make_cube(&mut mesh);
                    let mesh = meshes.add(mesh);
                    slots.insert(
                        slot.data().name().to_owned(),
                        parent
                            .spawn_bundle((
                                Mesh2dHandle(mesh.clone()),
                                Transform::from_xyz(demo.position.x, demo.position.y, 0.),
                                GlobalTransform::default(),
                                Visibility::default(),
                                ComputedVisibility::default(),
                                materials.add(ColorMaterial {
                                    color: Color::NONE,
                                    texture: None,
                                }),
                            ))
                            .id(),
                    );
                }
            })
            .insert(Spine { controller });
    }
}

#[derive(Default)]
struct DemoNextLocal {
    current_index: usize,
}

fn demo_next(
    mut ev_demo_load: EventWriter<DemoLoad>,
    mut local: Local<DemoNextLocal>,
    keys: Res<Input<KeyCode>>,
    demos: Res<Demos>,
) {
    if keys.just_pressed(KeyCode::Space) {
        local.current_index = (local.current_index + 1) % demos.0.len();
        ev_demo_load.send(DemoLoad(local.current_index));
    }
}

pub fn spine_update(
    mut spine_query: Query<(&mut Spine, &Children)>,
    colored_mesh2d: Query<(&Mesh2dHandle, &Handle<ColorMaterial>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (mut spine, spine_children) in spine_query.iter_mut() {
        let Spine { controller } = spine.as_mut();
        controller.update(time.delta_seconds());
        let mut renderables = controller.renderables();
        for (renderable_index, child) in spine_children.iter().enumerate() {
            if let Ok((mesh_handle, color_material_handle)) = colored_mesh2d.get(*child) {
                if let Some(renderable) = renderables.get_mut(renderable_index) {
                    let mut normals = vec![];
                    for _ in 0..renderable.vertices.len() {
                        normals.push([0., 0., 0.]);
                    }
                    let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
                    mesh.set_indices(Some(Indices::U32(take(&mut renderable.indices))));
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, take(&mut renderable.vertices));
                    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, take(&mut renderable.uvs));
                    if let Some(color_material) = color_materials.get_mut(color_material_handle) {
                        color_material.color.set_r(renderable.color.r);
                        color_material.color.set_g(renderable.color.g);
                        color_material.color.set_b(renderable.color.b);
                        color_material.color.set_a(renderable.color.a);
                        let texture_path = if let Some(attachment_render_object) =
                            renderable.attachment_renderer_object
                        {
                            let spine_texture =
                                unsafe { &mut *(attachment_render_object as *mut SpineTexture) };
                            Some(spine_texture.path.clone())
                        } else {
                            None
                        };
                        color_material.texture =
                            texture_path.map(|p| asset_server.load(p.as_str()));
                    }
                } else {
                    if let Some(color_material) = color_materials.get_mut(color_material_handle) {
                        color_material.color = Color::NONE;
                    }
                }
            }
        }
    }
}

fn load_skeleton(atlas: &Vec<u8>, json: &Vec<u8>, dir: &str) -> Result<SkeletonController, Error> {
    let atlas = Arc::new(Atlas::new(atlas, dir)?);
    let skeleton_json = SkeletonJson::new(atlas.clone());
    let skeleton_data = Arc::new(skeleton_json.read_skeleton_data(json)?);
    let animation_state_data = Arc::new(AnimationStateData::new(skeleton_data.clone()));
    Ok(
        SkeletonController::new(skeleton_data, animation_state_data).with_settings(
            SkeletonControllerSettings::new().with_cull_direction(CullDirection::CounterClockwise),
        ),
    )
}
