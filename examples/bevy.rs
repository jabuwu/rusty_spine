use std::{collections::HashMap, sync::Arc};

use bevy::{prelude::*, sprite::Rect};
use rusty_spine::{
    animation_state::AnimationState, animation_state_data::AnimationStateData, atlas::Atlas,
    c::spTextureRegion, error::Error, skeleton::Skeleton, skeleton_json::SkeletonJson,
    sync_ptr::SyncPtr,
};

#[derive(Component)]
struct Spine {
    skeleton: Skeleton,
    animation_state: AnimationState,
    slots: HashMap<String, Entity>,
}

#[derive(Default)]
struct RegionData {
    region_to_index: HashMap<SyncPtr<spTextureRegion>, usize>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<RegionData>()
        .add_startup_system(setup)
        .add_system(spine_update)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut region_data: ResMut<RegionData>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    match load_skeleton() {
        Ok((skeleton, mut animation_state, atlas)) => {
            let mut texture_atlas = TextureAtlas::new_empty(
                asset_server.load("./spineboy-pro.png"),
                Vec2::new(1534., 529.),
            );
            for (i, region) in atlas.regions().iter().enumerate() {
                let width = region.page().width() as f32;
                let height = region.page().height() as f32;
                let u = region.texture_region().u() as f32;
                let v = region.texture_region().v() as f32;
                let u2 = region.texture_region().u2() as f32;
                let v2 = region.texture_region().v2() as f32;
                texture_atlas.add_texture(Rect {
                    min: Vec2::new(width * u, height * v),
                    max: Vec2::new(width * u2, height * v2),
                });
                region_data
                    .region_to_index
                    .insert(SyncPtr(region.texture_region().c_ptr()), i);
            }
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            animation_state.set_animation_by_name(0, "run", true);
            let mut slots = HashMap::new();
            for slot in skeleton.slots().iter() {
                let entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: 0,
                            ..Default::default()
                        },
                        texture_atlas: texture_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .id();
                slots.insert(slot.data().name().to_owned(), entity);
            }
            commands.spawn().insert(Spine {
                skeleton,
                animation_state,
                slots,
            });
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn spine_update(
    mut spine_query: Query<&mut Spine>,
    mut children_query: Query<(&mut Transform, &mut TextureAtlasSprite)>,
    region_data: Res<RegionData>,
) {
    let scale = 0.5;
    let offset = Vec2::new(0., -200.);
    let mut z = 0.;
    for mut spine in spine_query.iter_mut() {
        let Spine {
            animation_state,
            skeleton,
            slots,
        } = spine.as_mut();
        animation_state.update(0.016);
        animation_state.apply(skeleton);
        skeleton.update_world_transform();
        for slot in skeleton.slots().iter() {
            if slot.attachment().is_none() {
                continue;
            }
            let slot_entity = slots.get(slot.data().name()).unwrap();
            let (mut slot_transform, mut slot_sprite) =
                children_query.get_mut(*slot_entity).unwrap();
            slot_transform.translation = Vec3::new(
                slot.bone().world_x() * scale,
                slot.bone().world_y() * scale,
                z,
            ) + offset.extend(0.);
            slot_transform.rotation =
                Quat::from_axis_angle(Vec3::Z, slot.bone().rotation().to_radians());
            slot_transform.scale = (Vec2::ONE * 0.3).extend(1.);
            if let Some(region_attachment) = slot
                .attachment()
                .and_then(|attachment| attachment.as_region())
            {
                let region = region_attachment.region();
                slot_transform.rotate_z(-(region.degrees() as f32).to_radians());
                if let Some(index) = region_data
                    .region_to_index
                    .get(&SyncPtr(region_attachment.region().c_ptr()))
                {
                    slot_sprite.index = *index;
                }
            } else if let Some(mesh_attachment) = slot
                .attachment()
                .and_then(|attachment| attachment.as_mesh())
            {
                let region = mesh_attachment.region();
                slot_transform.rotate_z(-(region.degrees() as f32).to_radians());
                if let Some(index) = region_data
                    .region_to_index
                    .get(&SyncPtr(mesh_attachment.region().c_ptr()))
                {
                    slot_sprite.index = *index;
                }
            } else {
                slot_transform.scale = Vec3::ZERO;
            }
        }
        z += 0.01;
    }
}

fn load_skeleton() -> Result<(Skeleton, AnimationState, Arc<Atlas>), Error> {
    let file = include_bytes!("../spineboy/spineboy-pro.atlas");
    let dir = "./";
    let atlas = Arc::new(Atlas::new(file, dir)?);
    let skeleton_json = SkeletonJson::new(atlas.clone());
    let skeleton_data =
        Arc::new(skeleton_json.read_skeleton_data(include_str!("../spineboy/spineboy-pro.json"))?);
    let animation_state_data = AnimationStateData::new(skeleton_data.clone());
    let skeleton = Skeleton::new(skeleton_data)?;
    let animation_state = AnimationState::new(Arc::new(animation_state_data));
    Ok((skeleton, animation_state, atlas))
}
