use std::{collections::HashMap, sync::Arc};

use bevy::prelude::*;
use rusty_spine::{
    animation_state::AnimationState, animation_state_data::AnimationStateData, atlas::Atlas,
    error::Error, skeleton::Skeleton, skeleton_json::SkeletonJson,
};

#[derive(Component)]
struct Spine {
    skeleton: Skeleton,
    animation_state: AnimationState,
    bones: HashMap<String, Entity>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(spine_update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    match load_skeleton() {
        Ok((skeleton, mut animation_state)) => {
            animation_state.set_animation_by_name(0, "run", true);
            let mut bones = HashMap::new();
            for bone in skeleton.bones().iter() {
                let entity = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::ONE * 16.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .id();
                bones.insert(bone.data().name().to_owned(), entity);
            }
            commands.spawn().insert(Spine {
                skeleton,
                animation_state,
                bones,
            });
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn spine_update(mut spine_query: Query<&mut Spine>, mut transform_query: Query<&mut Transform>) {
    let scale = 0.5;
    let offset = Vec2::new(0., -200.);
    for mut spine in spine_query.iter_mut() {
        let Spine {
            animation_state,
            skeleton,
            bones,
        } = spine.as_mut();
        animation_state.update(0.016);
        animation_state.apply(skeleton);
        skeleton.update_world_transform();
        for bone in skeleton.bones_mut().iter_mut() {
            let bone_entity = bones.get(bone.data().name()).unwrap();
            let mut bone_transform = transform_query.get_mut(*bone_entity).unwrap();
            bone_transform.translation =
                Vec3::new(bone.world_x() * scale, bone.world_y() * scale, 0.) + offset.extend(0.);
        }
    }
}

fn load_skeleton() -> Result<(Skeleton, AnimationState), Error> {
    let file = include_bytes!("../spineboy/spineboy-pro.atlas");
    let dir = "./";
    let atlas = Atlas::new(file, dir)?;
    let skeleton_json = SkeletonJson::new(Arc::new(atlas));
    let skeleton_data =
        Arc::new(skeleton_json.read_skeleton_data(include_str!("../spineboy/spineboy-pro.json"))?);
    let animation_state_data = AnimationStateData::new(skeleton_data.clone());
    let skeleton = Skeleton::new(skeleton_data)?;
    let animation_state = AnimationState::new(Arc::new(animation_state_data));
    Ok((skeleton, animation_state))
}
