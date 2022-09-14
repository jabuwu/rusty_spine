use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

use bevy::prelude::*;
use rusty_spine::c::*;

pub struct SyncPtr<T>(*mut T);
unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

#[derive(Component)]
struct Spine {
    skeleton: SyncPtr<spSkeleton>,
    animation_state: SyncPtr<spAnimationState>,
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
    let (c_skeleton, c_animation_state) = unsafe { load_skeleton() };
    unsafe {
        let c_animation_name = CString::new("walk").unwrap();
        spAnimationState_setAnimationByName(c_animation_state, 0, c_animation_name.as_ptr(), 1);
    }
    let mut bones = HashMap::new();
    let bone_count = unsafe { (*c_skeleton).bonesCount } as isize;
    for i in 0..bone_count {
        let bone = unsafe { *(*c_skeleton).bones.offset(i) };
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
        let bone_name =
            String::from(unsafe { CStr::from_ptr((*(*bone).data).name).to_str().unwrap() });
        bones.insert(bone_name, entity);
    }
    commands.spawn().insert(Spine {
        skeleton: SyncPtr(c_skeleton),
        animation_state: SyncPtr(c_animation_state),
        bones,
    });
}

fn spine_update(
    mut spine_query: Query<&mut Spine>,
    mut transform_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    let scale = 0.5;
    let offset = Vec2::new(0., -200.);
    for mut spine in spine_query.iter_mut() {
        let Spine {
            animation_state,
            skeleton,
            bones,
        } = spine.as_mut();
        unsafe {
            spAnimationState_update(animation_state.0, time.delta_seconds());
            spAnimationState_apply(animation_state.0, skeleton.0);
            spSkeleton_updateWorldTransform(skeleton.0);
        }
        let bone_count = unsafe { (*skeleton.0).bonesCount } as isize;
        for i in 0..bone_count {
            let bone = unsafe { *(*skeleton.0).bones.offset(i) };
            let bone_name =
                String::from(unsafe { CStr::from_ptr((*(*bone).data).name).to_str().unwrap() });
            let bone_entity = bones.get(&bone_name).unwrap();
            let mut bone_transform = transform_query.get_mut(*bone_entity).unwrap();
            unsafe {
                bone_transform.translation =
                    Vec3::new((*bone).worldX * scale, (*bone).worldY * scale, 0.)
                        + offset.extend(0.);
            }
        }
    }
}

unsafe fn load_skeleton() -> (*mut spSkeleton, *mut spAnimationState) {
    let file = include_str!("../spineboy/spineboy-pro.atlas");
    let c_file = CString::new(file).unwrap();
    let c_dir = CString::new("./").unwrap();
    let c_atlas = spAtlas_create(
        c_file.as_ptr(),
        file.len() as c_int,
        c_dir.as_ptr(),
        std::ptr::null_mut(),
    );
    let json = include_str!("../spineboy/spineboy-pro.json");
    let c_json = CString::new(json).unwrap();
    let c_skeleton_json = spSkeletonJson_create(c_atlas);
    let c_skeleton_data = spSkeletonJson_readSkeletonData(c_skeleton_json, c_json.as_ptr());
    let c_animation_state_data = spAnimationStateData_create(c_skeleton_data);
    let c_skeleton = spSkeleton_create(c_skeleton_data);
    let c_animation_state = spAnimationState_create(c_animation_state_data);
    (c_skeleton, c_animation_state)
}
