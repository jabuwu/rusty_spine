use crate::{
    c::{
        spBone, spBoneData, spBone_getWorldRotationX, spBone_getWorldRotationY,
        spBone_getWorldScaleX, spBone_getWorldScaleY, spBone_isYDown, spBone_localToWorld,
        spBone_rotateWorld, spBone_setToSetupPose, spBone_setYDown,
        spBone_updateAppliedTransform, spBone_updateWorldTransform,
        spBone_updateWorldTransformWith, spBone_worldToLocal, spBone_worldToLocalRotation,
        spSkeleton, spTransformMode,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Skeleton,
};

#[cfg(not(feature="spine38"))]
use crate::{
    c::{
        spBone_update
    }
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// A bone within the [Skeleton](struct.Skeleton.html) hierarchy.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Bone)
///
/// Bones can be acquired from a [Skeleton](struct.Skeleton.html) and a safe
/// [BoneHandle](struct.BoneHandle.html) can be obtained using the
/// [handle](struct.Bone.html#method.handle) method to store long-term references to a specific
/// bone.
///
/// The hierarchy can be traversed using [parent](struct.Bone.html#method.parent) and
/// [children](struct.Bone.html#method.children), and specific bones can be located using
/// [Skeleton::find_bone](struct.Skeleton.html#method.find_bone).
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

    #[cfg(not(feature="spine38"))]
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

    pub fn update_applied_transform(&mut self) {
        unsafe {
            spBone_updateAppliedTransform(self.c_ptr());
        }
    }

    pub fn world_rotation_x(&self) -> f32 {
        unsafe { spBone_getWorldRotationX(self.c_ptr()) }
    }

    pub fn world_rotation_y(&self) -> f32 {
        unsafe { spBone_getWorldRotationY(self.c_ptr()) }
    }

    pub fn world_scale_x(&self) -> f32 {
        unsafe { spBone_getWorldScaleX(self.c_ptr()) }
    }

    pub fn world_scale_y(&self) -> f32 {
        unsafe { spBone_getWorldScaleY(self.c_ptr()) }
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
    c_accessor_bool!(sorted, sorted);
    c_accessor_bool!(active, active);
    c_accessor_tmp_ptr!(data, data_mut, data, BoneData, spBoneData);
    c_accessor_tmp_ptr_optional!(parent, parent_mut, parent, Bone, spBone);
    c_accessor!(children_count, childrenCount, i32);
    c_accessor_array!(
        /// An iterator over the children of this bone.
        ///
        /// ```
        /// # #[path="./doctests.rs"]
        /// # mod doctests;
        /// use rusty_spine::{BoneHandle, Skeleton};
        ///
        /// fn traverse_bones(
        ///     bone_handle: BoneHandle,
        ///     skeleton: &Skeleton,
        ///     ident: usize,
        /// ) {
        ///     if let Some(bone) = bone_handle.get(skeleton) {
        ///         println!(
        ///             "{:ident$}{name}",
        ///             "",
        ///             ident = ident,
        ///             name = bone.data().name()
        ///         );
        ///         for child in bone.children() {
        ///             traverse_bones(child.handle(), skeleton, ident + 2);
        ///         }
        ///     }
        /// }
        ///
        /// // Traverse all bones in a skeleton
        /// # let (skeleton, animation_state) = doctests::test_spineboy_instance();
        /// let root_bone = skeleton.bone_root().handle();
        /// traverse_bones(root_bone, &skeleton, 0);
        /// ```
        children,
        children_mut,
        children_at_index,
        children_at_index_mut,
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

#[cfg(feature = "mint")]
impl Bone {
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn set_position(&mut self, position: impl Into<Vector2<f32>>) {
        let position: Vector2<f32> = position.into();
        self.set_x(position.x);
        self.set_y(position.y);
    }

    pub fn world_position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    pub fn set_world_position(&mut self, position: impl Into<Vector2<f32>>) {
        let position: Vector2<f32> = position.into();
        self.set_world_x(position.x);
        self.set_world_y(position.y);
    }

    pub fn applied_position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    pub fn set_applied_position(&mut self, position: impl Into<Vector2<f32>>) {
        let position: Vector2<f32> = position.into();
        self.set_applied_x(position.x);
        self.set_applied_y(position.y);
    }

    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    pub fn set_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_scale_x(scale.x);
        self.set_scale_y(scale.y);
    }

    pub fn world_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    pub fn applied_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.applied_scale_x(),
            y: self.applied_scale_y(),
        }
    }

    pub fn set_applied_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_applied_scale_x(scale.x);
        self.set_applied_scale_y(scale.y);
    }

    pub fn shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.shear_x(),
            y: self.shear_y(),
        }
    }

    pub fn set_shear(&mut self, shear: impl Into<Vector2<f32>>) {
        let shear: Vector2<f32> = shear.into();
        self.set_shear_x(shear.x);
        self.set_shear_y(shear.y);
    }

    pub fn applied_shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.applied_shear_x(),
            y: self.applied_shear_y(),
        }
    }

    pub fn set_applied_shear(&mut self, shear: impl Into<Vector2<f32>>) {
        let shear: Vector2<f32> = shear.into();
        self.set_applied_shear_x(shear.x);
        self.set_applied_shear_y(shear.y);
    }

    pub fn world_rotation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_rotation_x(),
            y: self.world_rotation_y(),
        }
    }
}

c_handle_decl!(
    /// A storeable reference to a [Bone](struct.Bone.html).
    ///
    /// Can be acquired from any instance of [Bone](struct.Bone.html).
    ///
    /// ```
    /// # #[path="./doctests.rs"]
    /// # mod doctests;
    /// # use rusty_spine::{AnimationState, EventType, BoneHandle};
    /// # let (skeleton, _) = doctests::test_spineboy_instance();
    /// let bone_handles: Vec<BoneHandle> = skeleton.bones().map(|bone| bone.handle()).collect();
    /// for bone_handle in bone_handles.iter() {
    ///     let bone = bone_handle.get(&skeleton).unwrap();
    ///     println!("{}", bone.data().name());
    /// }
    /// ```
    BoneHandle,
    Bone,
    Skeleton,
    spBone,
    spSkeleton
);

/// Static bone data imported from Spine.
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
    c_accessor_mut!(length, set_length, length, f32);
    c_accessor_mut!(x, set_x, x, f32);
    c_accessor_mut!(y, set_y, y, f32);
    c_accessor_mut!(rotation, set_rotation, rotation, f32);
    c_accessor_mut!(scale_x, set_scale_x, scaleX, f32);
    c_accessor_mut!(scale_y, set_scale_y, scaleY, f32);
    c_accessor_mut!(shear_x, set_shear_x, shearX, f32);
    c_accessor_mut!(shear_y, set_shear_y, shearY, f32);
    #[cfg(not(feature="spine38"))]
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor_bool_mut!(skin_required, set_skin_required, skinRequired);
    c_accessor_enum_mut!(
        transform_mode,
        set_transform_mode,
        transformMode,
        TransformMode
    );
    c_accessor_tmp_ptr_optional!(parent, parent_mut, parent, BoneData, spBoneData);
}

#[cfg(feature = "mint")]
impl BoneData {
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn set_position(&mut self, position: impl Into<Vector2<f32>>) {
        let position: Vector2<f32> = position.into();
        self.set_x(position.x);
        self.set_y(position.y);
    }

    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    pub fn set_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_scale_x(scale.x);
        self.set_scale_y(scale.y);
    }

    pub fn shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.shear_x(),
            y: self.shear_y(),
        }
    }

    pub fn set_shear(&mut self, shear: impl Into<Vector2<f32>>) {
        let shear: Vector2<f32> = shear.into();
        self.set_shear_x(shear.x);
        self.set_shear_y(shear.y);
    }
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
