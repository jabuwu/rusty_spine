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
    c::{spSkeletonClipping_clipTriangles, spSkeletonClipping_isClipping},
    error::Error,
    skeleton::Skeleton,
    skeleton_clipping::SkeletonClipping,
    skeleton_json::SkeletonJson,
};

#[derive(Component)]
pub struct Spine {
    skeleton: Skeleton,
    animation_state: AnimationState,
    slots: HashMap<String, Entity>,
    clipper: SkeletonClipping,
}

#[derive(Debug)]
struct SpineTexture {
    _path: String,
}

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
            _path: path.to_owned(),
        });
    });
    rusty_spine::extension::set_dispose_texture_cb(|page| unsafe {
        page.renderer_object().dispose::<SpineTexture>();
    });
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(star)
        .add_system(star_update)
        .run();
}

fn star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let (skeleton, mut animation_state, _) = load_skeleton().unwrap();
    animation_state.set_animation_by_name(0, "portal", true);
    let mut slots = HashMap::new();
    commands
        .spawn_bundle((
            Transform::default(),
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
                            Transform::from_xyz(0., -200., 0.),
                            GlobalTransform::default(),
                            Visibility::default(),
                            ComputedVisibility::default(),
                            materials.add(ColorMaterial {
                                color: Color::WHITE,
                                texture: Some(asset_server.load("spineboy-pro.png")),
                            }),
                        ))
                        .id(),
                );
            }
        })
        .insert(Spine {
            skeleton,
            animation_state,
            slots,
            clipper: SkeletonClipping::new(),
        });
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn star_update(
    mut spine_query: Query<&mut Spine>,
    colored_mesh2d: Query<&Mesh2dHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    for mut spine in spine_query.iter_mut() {
        let Spine {
            animation_state,
            skeleton,
            slots,
            clipper,
        } = spine.as_mut();
        animation_state.update(time.delta_seconds());
        animation_state.apply(skeleton);
        skeleton.update_world_transform();
        for slot in skeleton.slots().iter() {
            let slot_entity = slots.get(slot.data().name()).unwrap();
            if let Ok(mesh_handle) = colored_mesh2d.get(*slot_entity) {
                let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
                let mut clip_vertices = vec![];
                let mut clip_indices = vec![];
                let mut clip_uvs = vec![];
                if let Some(mesh_attachment) = slot.attachment().and_then(|a| a.as_mesh()) {
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
                            unsafe { *mesh_attachment.triangles().offset(i as isize) } as i32
                        );
                    }
                } else if let Some(region_attachment) =
                    slot.attachment().and_then(|a| a.as_region())
                {
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
                                0.,
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
                            v_pos.push([clip_vertices[i * 2], clip_vertices[i * 2 + 1], 0.]);
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
            }
        }

        clipper.clip_end2();
    }
}

fn load_skeleton() -> Result<(Skeleton, AnimationState, Arc<Atlas>), Error> {
    let file = include_bytes!("../spineboy/spineboy-pro.atlas");
    let dir = "";
    let atlas = Arc::new(Atlas::new(file, dir)?);
    let mut skeleton_json = SkeletonJson::new(atlas.clone());
    *skeleton_json.scale_mut() = 0.4;
    let skeleton_data =
        Arc::new(skeleton_json.read_skeleton_data(include_str!("../spineboy/spineboy-pro.json"))?);
    let animation_state_data = AnimationStateData::new(skeleton_data.clone());
    let skeleton = Skeleton::new(skeleton_data)?;
    let animation_state = AnimationState::new(Arc::new(animation_state_data));
    Ok((skeleton, animation_state, atlas))
}
