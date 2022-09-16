use std::{collections::HashMap, sync::Arc};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use rusty_spine::{
    animation_state::AnimationState,
    animation_state_data::AnimationStateData,
    atlas::Atlas,
    c::{
        spSkeletonClipping_clipTriangles, spSkeletonClipping_isClipping, spSkeleton_setToSetupPose,
    },
    error::Error,
    skeleton::Skeleton,
    skeleton_clipping::SkeletonClipping,
    skeleton_json::SkeletonJson,
};

#[derive(Component)]
pub struct Spine {
    skeleton: Skeleton,
    animation_state: AnimationState,
    clipper: SkeletonClipping,
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
            },
            Demo {
                atlas: include_bytes!("../assets/windmill/export/windmill.atlas").to_vec(),
                json: include_bytes!("../assets/windmill/export/windmill-ess.json").to_vec(),
                dir: "windmill/export/".to_owned(),
                animation: "animation".to_owned(),
                position: Vec2::new(0., -150.),
                scale: 0.5,
            },
            Demo {
                atlas: include_bytes!("../assets/alien/export/alien.atlas").to_vec(),
                json: include_bytes!("../assets/alien/export/alien-pro.json").to_vec(),
                dir: "alien/export/".to_owned(),
                animation: "death".to_owned(),
                position: Vec2::new(0., -600.),
                scale: 0.25,
            },
            Demo {
                atlas: include_bytes!("../assets/coin/export/coin.atlas").to_vec(),
                json: include_bytes!("../assets/coin/export/coin-pro.json").to_vec(),
                dir: "coin/export/".to_owned(),
                animation: "animation".to_owned(),
                position: Vec2::new(0., 0.),
                scale: 0.75,
            },
            /*TODO: figure out why dragon crashes - Demo {
                atlas: "assets/dragon/export/dragon.atlas".to_owned(),
                json: "assets/dragon/export/dragon-ess.json".to_owned(),
                dir: "dragon/export/".to_owned(),
                animation: "flying".to_owned(),
                position: Vec2::new(0., 0.),
                scale: 0.75,
            },*/
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
        let (mut skeleton, mut animation_state, _) =
            load_skeleton(&demo.atlas, &demo.json, &demo.dir).unwrap();
        unsafe {
            spSkeleton_setToSetupPose(skeleton.c_ptr());
        }
        skeleton.update_world_transform();
        animation_state.set_animation_by_name(0, &demo.animation, true);
        let mut slots = HashMap::new();
        commands
            .spawn_bundle((
                Transform::from_scale(Vec3::ONE * demo.scale),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .with_children(|parent| {
                for slot in skeleton.slots().iter() {
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
            .insert(Spine {
                skeleton,
                animation_state,
                clipper: SkeletonClipping::new(),
            });
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
    mut colored_mesh2d: Query<(&Mesh2dHandle, &Handle<ColorMaterial>, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (mut spine, spine_children) in spine_query.iter_mut() {
        let Spine {
            animation_state,
            skeleton,
            clipper,
        } = spine.as_mut();
        animation_state.update(time.delta_seconds());
        animation_state.apply(skeleton);
        skeleton.update_world_transform();
        let draw_order = skeleton.draw_order();
        for (slot_index, child) in spine_children.iter().enumerate() {
            let slot_entity = *child;
            let slot = &draw_order[slot_index];
            if let Ok((mesh_handle, color_material_handle, mut transform)) =
                colored_mesh2d.get_mut(slot_entity)
            {
                if !slot.bone().active() {
                    continue;
                }

                let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
                let mut clip_vertices = vec![];
                let mut clip_indices = vec![];
                let mut clip_uvs = vec![];
                let mut color = rusty_spine::color::Color::default();
                if let Some(mesh_attachment) = slot.attachment().and_then(|a| a.as_mesh()) {
                    //color = *mesh_attachment.color();
                    color.r = mesh_attachment.c_ptr_mut().color.r;
                    color.g = mesh_attachment.c_ptr_mut().color.g;
                    color.b = mesh_attachment.c_ptr_mut().color.b;
                    color.a = mesh_attachment.c_ptr_mut().color.a;

                    let mut world_vertices = vec![];
                    world_vertices.resize(1000, 0.);
                    unsafe {
                        mesh_attachment.compute_world_vertices(
                            slot,
                            0,
                            mesh_attachment.world_vertices_length(),
                            &mut world_vertices,
                            0,
                            2,
                        );
                    }

                    for i in 0..mesh_attachment.world_vertices_length() {
                        clip_vertices.push(world_vertices[i as usize * 2]);
                        clip_vertices.push(world_vertices[i as usize * 2 + 1]);

                        unsafe {
                            clip_uvs.push(*mesh_attachment.c_ptr_mut().uvs.offset(i as isize * 2));
                            clip_uvs
                                .push(*mesh_attachment.c_ptr_mut().uvs.offset(i as isize * 2 + 1));
                        }
                    }

                    for i in 0..mesh_attachment.triangles_count() {
                        clip_indices.push(
                            unsafe { *mesh_attachment.triangles().offset(i as isize) } as u16
                        );
                    }
                } else if let Some(region_attachment) =
                    slot.attachment().and_then(|a| a.as_region())
                {
                    //color = *region_attachment.color();
                    color.r = region_attachment.c_ptr_mut().color.r;
                    color.g = region_attachment.c_ptr_mut().color.g;
                    color.b = region_attachment.c_ptr_mut().color.b;
                    color.a = region_attachment.c_ptr_mut().color.a;

                    let mut world_vertices = vec![];
                    world_vertices.resize(1000, 0.);
                    unsafe {
                        region_attachment.compute_world_vertices(slot, &mut world_vertices, 0, 2);
                    }

                    for i in 0..4 {
                        clip_vertices.push(world_vertices[i as usize * 2]);
                        clip_vertices.push(world_vertices[i as usize * 2 + 1]);

                        clip_uvs.push(region_attachment.c_ptr_mut().uvs[i as usize * 2]);
                        clip_uvs.push(region_attachment.c_ptr_mut().uvs[i as usize * 2 + 1]);
                    }

                    clip_indices = vec![0, 1, 2, 2, 3, 0];
                } else if let Some(clipping_attachment) =
                    slot.attachment().and_then(|a| a.as_clipping())
                {
                    clipper.clip_start(slot, &clipping_attachment);
                    continue;
                } else {
                    make_cube(mesh);
                    continue;
                }

                for _ in clip_vertices.iter() {
                    clip_uvs.push(0.);
                    clip_uvs.push(0.);
                }
                let mut v_pos = vec![];
                let mut indices = vec![];
                let mut uvs = vec![];
                let z = slot_index as f32 / 100.;
                transform.translation.z = z;
                unsafe {
                    if spSkeletonClipping_isClipping(clipper.c_ptr()) != 0 {
                        spSkeletonClipping_clipTriangles(
                            clipper.c_ptr(),
                            clip_vertices.as_mut_ptr(),
                            clip_vertices.len() as i32 / 2,
                            clip_indices.as_mut_ptr(),
                            clip_indices.len() as i32,
                            clip_uvs.as_mut_ptr(),
                            2,
                        );
                        let clipped_vertices_size = (*clipper.c_ptr_ref().clippedVertices).size;
                        for i in 0..(clipped_vertices_size / 2) {
                            v_pos.push([
                                *(*clipper.c_ptr_ref().clippedVertices)
                                    .items
                                    .offset(i as isize * 2),
                                *(*clipper.c_ptr_ref().clippedVertices)
                                    .items
                                    .offset(i as isize * 2 + 1),
                                z,
                            ]);
                        }
                        let clipped_triangles_size = (*clipper.c_ptr_ref().clippedTriangles).size;
                        for i in 0..clipped_triangles_size {
                            indices.push(
                                *(*clipper.c_ptr_ref().clippedTriangles)
                                    .items
                                    .offset(i as isize) as u32,
                            );
                        }
                        let clipped_uvs_size = (*clipper.c_ptr_ref().clippedUVs).size;
                        for i in 0..(clipped_uvs_size / 2) {
                            uvs.push([
                                *(*clipper.c_ptr_ref().clippedUVs)
                                    .items
                                    .offset(i as isize * 2),
                                *(*clipper.c_ptr_ref().clippedUVs)
                                    .items
                                    .offset(i as isize * 2 + 1),
                            ]);
                        }
                    } else {
                        for i in 0..(clip_vertices.len() / 2) {
                            v_pos.push([clip_vertices[i * 2], clip_vertices[i * 2 + 1], z]);
                        }
                        for index in clip_indices.iter() {
                            indices.push(*index as u32);
                        }
                        for i in 0..v_pos.len() {
                            uvs.push([clip_uvs[i * 2], clip_uvs[i * 2 + 1]]);
                        }
                    }
                }

                for i in 0..indices.len() / 3 {
                    let a = indices[i * 3 + 1];
                    indices[i * 3] = indices[i * 3];
                    indices[i * 3 + 1] = indices[i * 3 + 2];
                    indices[i * 3 + 2] = a;
                }

                let mut v_color: Vec<u32> = vec![];
                for _ in 0..v_pos.len() {
                    v_color.push(Color::BLACK.as_linear_rgba_u32());
                }

                let mut normals = vec![];
                for _ in 0..v_pos.len() {
                    normals.push([0., 0., 0.]);
                }

                mesh.set_indices(Some(Indices::U32(indices)));
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

                clipper.clip_end(slot);

                if let Some(color_material) = color_materials.get_mut(color_material_handle) {
                    let texture_path = if let Some(mesh_attachment) =
                        slot.attachment().and_then(|a| a.as_mesh())
                    {
                        Some(unsafe {
                            mesh_attachment
                                .renderer_object()
                                .get_atlas_region()
                                .page()
                                .renderer_object()
                                .get::<SpineTexture>()
                                .path
                                .as_str()
                                .to_owned()
                        })
                    } else if let Some(region_attachment) =
                        slot.attachment().and_then(|a| a.as_region())
                    {
                        Some(unsafe {
                            region_attachment
                                .renderer_object()
                                .get_atlas_region()
                                .page()
                                .renderer_object()
                                .get::<SpineTexture>()
                                .path
                                .as_str()
                                .to_owned()
                        })
                    } else {
                        None
                    };

                    color.r *= slot.c_ptr_mut().color.r * skeleton.c_ptr_mut().color.r;
                    color.g *= slot.c_ptr_mut().color.g * skeleton.c_ptr_mut().color.g;
                    color.b *= slot.c_ptr_mut().color.b * skeleton.c_ptr_mut().color.b;
                    color.a *= slot.c_ptr_mut().color.a * skeleton.c_ptr_mut().color.a;

                    color_material.texture = texture_path.map(|p| asset_server.load(p.as_str()));
                    // TODO: figure out why colors are broken
                    color_material.color.set_r(color.r);
                    color_material.color.set_g(color.g);
                    color_material.color.set_b(color.b);
                    color_material.color.set_a(color.a);
                }
            }
        }

        clipper.clip_end2();
    }
}

fn load_skeleton(
    atlas: &Vec<u8>,
    json: &Vec<u8>,
    dir: &str,
) -> Result<(Skeleton, AnimationState, Arc<Atlas>), Error> {
    let atlas = Arc::new(Atlas::new(atlas, dir)?);
    let skeleton_json = SkeletonJson::new(atlas.clone());
    let skeleton_data = Arc::new(skeleton_json.read_skeleton_data(json)?);
    let animation_state_data = AnimationStateData::new(skeleton_data.clone());
    let skeleton = Skeleton::new(skeleton_data)?;
    let animation_state = AnimationState::new(Arc::new(animation_state_data));
    Ok((skeleton, animation_state, atlas))
}
