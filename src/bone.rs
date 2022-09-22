use crate::{
    c::{
        spBone, spBoneData, spBone_getWorldRotationX, spBone_getWorldRotationY,
        spBone_getWorldScaleX, spBone_getWorldScaleY, spBone_isYDown, spBone_localToWorld,
        spBone_rotateWorld, spBone_setToSetupPose, spBone_setYDown, spBone_update,
        spBone_updateAppliedTransform, spBone_updateWorldTransform,
        spBone_updateWorldTransformWith, spBone_worldToLocal, spBone_worldToLocalRotation,
        spSkeleton, spTransformMode,
    },
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
    Skeleton,
};

#[derive(Debug)]
pub struct Bone {
    c_bone: SyncPtr<spBone>,
}

impl NewFromPtr<spBone> for Bone {
    unsafe fn new_from_ptr(c_bone: *const spBone) -> Self {
        Self {
            c_bone: SyncPtr(c_bone as *mut spBone),
        }
    }
}

impl Bone {
    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spBone_setToSetupPose(self.c_ptr());
        }
    }

    pub fn update(&mut self) {
        unsafe {
            spBone_update(self.c_ptr());
        }
    }

    pub fn update_world_transform(&mut self) {
        unsafe {
            spBone_updateWorldTransform(self.c_ptr());
        }
    }

    pub fn update_world_transform_with(
        &mut self,
        x: f32,
        y: f32,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
        shear_x: f32,
        shear_y: f32,
    ) {
        unsafe {
            spBone_updateWorldTransformWith(
                self.c_ptr(),
                x,
                y,
                rotation,
                scale_x,
                scale_y,
                shear_x,
                shear_y,
            );
        }
    }

    pub fn get_world_rotation_x(&self) -> f32 {
        unsafe { spBone_getWorldRotationX(self.c_ptr()) }
    }

    pub fn get_world_rotation_y(&self) -> f32 {
        unsafe { spBone_getWorldRotationY(self.c_ptr()) }
    }

    pub fn get_world_scale_x(&self) -> f32 {
        unsafe { spBone_getWorldScaleX(self.c_ptr()) }
    }

    pub fn get_world_scale_y(&self) -> f32 {
        unsafe { spBone_getWorldScaleY(self.c_ptr()) }
    }

    pub fn update_applied_transform(&mut self) {
        unsafe {
            spBone_updateAppliedTransform(self.c_ptr());
        }
    }

    pub fn world_to_local(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        let mut local_x: f32 = 0.;
        let mut local_y: f32 = 0.;
        unsafe {
            spBone_worldToLocal(self.c_ptr(), world_x, world_y, &mut local_x, &mut local_y);
        }
        (local_x, local_y)
    }

    pub fn local_to_world(&self, local_x: f32, local_y: f32) -> (f32, f32) {
        let mut world_x: f32 = 0.;
        let mut world_y: f32 = 0.;
        unsafe {
            spBone_localToWorld(self.c_ptr(), local_x, local_y, &mut world_x, &mut world_y);
        }
        (world_x, world_y)
    }

    pub fn world_to_local_rotation(&self, world_rotation: f32) -> f32 {
        unsafe { spBone_worldToLocalRotation(self.c_ptr(), world_rotation) }
    }

    pub fn local_to_world_rotation(&self, local_rotation: f32) -> f32 {
        unsafe { spBone_worldToLocalRotation(self.c_ptr(), local_rotation) }
    }

    pub fn rotate_world(&self, degrees: f32) {
        unsafe {
            spBone_rotateWorld(self.c_ptr(), degrees);
        }
    }

    pub fn handle(&self) -> BoneHandle {
        BoneHandle::new(self.c_ptr(), unsafe { self.c_ptr_mut().skeleton })
    }

    c_accessor_mut!(x, set_x, x, f32);
    c_accessor_mut!(y, set_y, y, f32);
    c_accessor_mut!(rotation, set_rotation, rotation, f32);
    c_accessor_mut!(scale_x, set_scale_x, scaleX, f32);
    c_accessor_mut!(scale_y, set_scale_y, scaleY, f32);
    c_accessor_mut!(shear_x, set_shear_x, shearX, f32);
    c_accessor_mut!(shear_y, set_shear_y, shearY, f32);
    c_accessor_mut!(applied_x, set_applied_x, ax, f32);
    c_accessor_mut!(applied_y, set_applied_y, ay, f32);
    c_accessor_mut!(applied_rotation, set_applied_rotation, arotation, f32);
    c_accessor_mut!(applied_scale_x, set_applied_scale_x, ascaleX, f32);
    c_accessor_mut!(applied_scale_y, set_applied_scale_y, ascaleY, f32);
    c_accessor_mut!(applied_shear_x, set_applied_shear_x, ashearX, f32);
    c_accessor_mut!(applied_shear_y, set_applied_shear_y, ashearY, f32);
    c_accessor_mut!(a, set_a, a, f32);
    c_accessor_mut!(b, set_b, b, f32);
    c_accessor_mut!(c, set_c, c, f32);
    c_accessor_mut!(d, set_d, d, f32);
    c_accessor_mut!(world_x, set_world_x, worldX, f32);
    c_accessor_mut!(world_y, set_world_y, worldY, f32);
    c_accessor_bool_mut!(sorted, set_sorted, sorted);
    c_accessor_bool_mut!(active, set_active, active);
    c_accessor_tmp_ptr!(data, data_mut, data, BoneData, spBoneData);
    c_accessor_tmp_ptr!(parent, parent_mut, parent, Bone, spBone);
    c_accessor!(children_count, childrenCount, i32);
    c_accessor_array!(
        children,
        children_mut,
        childrenat_index,
        childrenat_index_mut,
        Bone,
        Bone,
        spBone,
        children,
        children_count
    );

    pub fn set_y_down(y_down: bool) {
        unsafe {
            spBone_setYDown(y_down as i32);
        }
    }

    pub fn is_y_down() -> bool {
        unsafe { spBone_isYDown() != 0 }
    }

    c_ptr!(c_bone, spBone);
}

c_handle_decl!(BoneHandle, Bone, Skeleton, spBone, spSkeleton);

#[derive(Debug)]
pub struct BoneData {
    c_bone_data: SyncPtr<spBoneData>,
}

impl NewFromPtr<spBoneData> for BoneData {
    unsafe fn new_from_ptr(c_bone_data: *const spBoneData) -> Self {
        Self {
            c_bone_data: SyncPtr(c_bone_data as *mut spBoneData),
        }
    }
}

impl BoneData {
    c_ptr!(c_bone_data, spBoneData);
    c_accessor_string!(name, name);
    c_accessor!(index, index, i32);
    c_accessor!(length, length, f32);
    c_accessor!(x, x, f32);
    c_accessor!(y, y, f32);
    c_accessor!(rotation, rotation, f32);
    c_accessor!(scale_x, scaleX, f32);
    c_accessor!(scale_y, scaleY, f32);
    c_accessor!(shear_x, shearX, f32);
    c_accessor!(shear_y, shearY, f32);
    c_accessor_color!(color, color);
    c_accessor_bool!(skin_required, skinRequired);
    c_accessor_enum!(transform_mode, transformMode, TransformMode);
    c_accessor_tmp_ptr_optional!(parent, parent_mut, parent, BoneData, spBoneData);
}

pub enum TransformMode {
    Normal = 0,
    OnlyTranslation = 1,
    NoRotationOrReflection = 2,
    NoScale = 3,
    NoScaleOrReflection = 4,
    Unknown = 99,
}

impl From<spTransformMode> for TransformMode {
    fn from(mode: spTransformMode) -> Self {
        match mode {
            0 => Self::Normal,
            1 => Self::OnlyTranslation,
            2 => Self::NoRotationOrReflection,
            3 => Self::NoScale,
            4 => Self::NoScaleOrReflection,
            _ => Self::Unknown,
        }
    }
}
