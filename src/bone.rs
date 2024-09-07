use crate::{
    c::{
        spBone, spBoneData, spBone_getWorldRotationX, spBone_getWorldRotationY,
        spBone_getWorldScaleX, spBone_getWorldScaleY, spBone_isYDown, spBone_localToWorld,
        spBone_rotateWorld, spBone_setToSetupPose, spBone_setYDown, spBone_updateAppliedTransform,
        spBone_updateWorldTransform, spBone_updateWorldTransformWith, spBone_worldToLocal,
        spBone_worldToLocalRotation, spSkeleton,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Skeleton,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// A bone within the [`Skeleton`] hierarchy.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#Bone)
///
/// Bones can be acquired from a [`Skeleton`] and a safe [`BoneHandle`] can be obtained using the
/// [`Bone::handle`] method to store long-term references to a specific
/// bone.
///
/// The hierarchy can be traversed using [`Bone::parent`] and [`Bone::children`], and specific
/// bones can be located using [`Skeleton::find_bone`].
#[derive(Debug)]
pub struct Bone {
    c_bone: SyncPtr<spBone>,
}

impl NewFromPtr<spBone> for Bone {
    unsafe fn new_from_ptr(c_bone: *mut spBone) -> Self {
        Self {
            c_bone: SyncPtr(c_bone),
        }
    }
}

impl Bone {
    /// Sets this bone's local transform to the setup pose.
    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spBone_setToSetupPose(self.c_ptr());
        }
    }

    /// Computes the world transform using the parent bone and this bone's local transform.
    ///
    /// See [`update_world_transform_with`](`Self::update_world_transform_with`).
    pub fn update_world_transform(&mut self) {
        unsafe {
            spBone_updateWorldTransform(self.c_ptr());
        }
    }

    /// Computes the world transform using the parent bone and the specified local transform. The
    /// applied transform is set to the specified local transform. Child bones are not updated.
    ///
    /// See
    /// [`World transforms`](http://esotericsoftware.com/spine-runtime-skeletons#World-transforms)
    /// in the Spine Runtimes Guide.
    #[allow(clippy::too_many_arguments)]
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

    /// Computes the applied transform values from the world transform.
    ///
    /// If the world transform is modified (by a constraint, [`rotate_world`](`Self::rotate_world`),
    /// etc) then this method should be called so the applied transform matches the world transform.
    /// The applied transform may be needed by other code (eg to apply another constraint).
    ///
    /// Some information is ambiguous in the world transform, such as -1,-1 scale versus 180
    /// rotation. The applied transform after calling this method is equivalent to the local
    /// transform used to compute the world transform, but may not be identical.
    pub fn update_applied_transform(&mut self) {
        unsafe {
            spBone_updateAppliedTransform(self.c_ptr());
        }
    }

    /// The world rotation for the X axis, calculated using [`a`](`Self::a`) and [`c`](`Self::c`).
    #[must_use]
    pub fn world_rotation_x(&self) -> f32 {
        unsafe { spBone_getWorldRotationX(self.c_ptr()) }
    }

    /// The world rotation for the Y axis, calculated using [`b`](`Self::b`) and [`d`](`Self::d`).
    #[must_use]
    pub fn world_rotation_y(&self) -> f32 {
        unsafe { spBone_getWorldRotationY(self.c_ptr()) }
    }

    /// The magnitude (always positive) of the world scale X, calculated using [`a`](`Self::a`) and
    /// [`c`](`Self::c`).
    #[must_use]
    pub fn world_scale_x(&self) -> f32 {
        unsafe { spBone_getWorldScaleX(self.c_ptr()) }
    }

    /// The magnitude (always positive) of the world scale Y, calculated using [`b`](`Self::b`) and
    /// [`d`](`Self::d`).
    #[must_use]
    pub fn world_scale_y(&self) -> f32 {
        unsafe { spBone_getWorldScaleY(self.c_ptr()) }
    }

    /// Transforms a point from world coordinates to the bone's local coordinates.
    #[must_use]
    pub fn world_to_local(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        let mut local_x: f32 = 0.;
        let mut local_y: f32 = 0.;
        unsafe {
            spBone_worldToLocal(self.c_ptr(), world_x, world_y, &mut local_x, &mut local_y);
        }
        (local_x, local_y)
    }

    /// Transforms a point from the bone's local coordinates to world coordinates.
    #[must_use]
    pub fn local_to_world(&self, local_x: f32, local_y: f32) -> (f32, f32) {
        let mut world_x: f32 = 0.;
        let mut world_y: f32 = 0.;
        unsafe {
            spBone_localToWorld(self.c_ptr(), local_x, local_y, &mut world_x, &mut world_y);
        }
        (world_x, world_y)
    }

    /// Transforms a world rotation to a local rotation.
    #[must_use]
    pub fn world_to_local_rotation(&self, world_rotation: f32) -> f32 {
        unsafe { spBone_worldToLocalRotation(self.c_ptr(), world_rotation) }
    }

    /// Transforms a local rotation to a world rotation.
    #[must_use]
    pub fn local_to_world_rotation(&self, local_rotation: f32) -> f32 {
        unsafe { spBone_worldToLocalRotation(self.c_ptr(), local_rotation) }
    }

    /// Rotates the world transform the specified amount.
    ///
    /// After changes are made to the world transform,
    /// [`update_applied_transform`](`Self::update_applied_transform`) should be called and update
    /// will need to be called on any child bones, recursively.
    pub fn rotate_world(&self, degrees: f32) {
        unsafe {
            spBone_rotateWorld(self.c_ptr(), degrees);
        }
    }

    /// Create a persistent [`BoneHandle`] to this [`Bone`].
    #[must_use]
    pub fn handle(&self) -> BoneHandle {
        BoneHandle::new(self.c_ptr(), unsafe { self.c_ptr_mut().skeleton })
    }

    c_accessor_mut!(
        /// The local x translation.
        x,
        /// Set the local x translation.
        set_x,
        x,
        f32
    );
    c_accessor_mut!(
        /// The local y translation.
        y,
        /// Set the local y translation.
        set_y,
        y,
        f32
    );
    c_accessor_mut!(
        /// The local rotation in degrees, counter clockwise.
        rotation,
        /// Set the local rotation in degrees, counter clockwise.
        set_rotation,
        rotation,
        f32
    );
    c_accessor_mut!(
        /// The local scaleX.
        scale_x,
        /// Set the local scaleX.
        set_scale_x,
        scaleX,
        f32
    );
    c_accessor_mut!(
        /// The local scaleY.
        scale_y,
        /// Set the local scaleY.
        set_scale_y,
        scaleY,
        f32
    );
    c_accessor_mut!(
        /// The local shearX.
        shear_x,
        /// Set the local shearX.
        set_shear_x,
        shearX,
        f32
    );
    c_accessor_mut!(
        /// The local shearY.
        shear_y,
        /// Set the local shearY.
        set_shear_y,
        shearY,
        f32
    );
    c_accessor_mut!(
        /// The applied local x translation.
        applied_x,
        /// Set the applied local x translation.
        set_applied_x,
        ax,
        f32
    );
    c_accessor_mut!(
        /// The applied local y translation.
        applied_y,
        /// Set the applied local y translation.
        set_applied_y,
        ay,
        f32
    );
    c_accessor_mut!(
        /// The applied local rotation in degrees, counter clockwise.
        applied_rotation,
        /// Set the applied local rotation in degrees, counter clockwise.
        set_applied_rotation,
        arotation,
        f32
    );
    c_accessor_mut!(
        /// The applied local scaleX.
        applied_scale_x,
        /// Set the applied local scaleX.
        set_applied_scale_x,
        ascaleX,
        f32
    );
    c_accessor_mut!(
        /// The applied local scaleY.
        applied_scale_y,
        /// Set the applied local scaleY.
        set_applied_scale_y,
        ascaleY,
        f32
    );
    c_accessor_mut!(
        /// The applied local shearX.
        applied_shear_x,
        /// Set the applied local shearX.
        set_applied_shear_x,
        ashearX,
        f32
    );
    c_accessor_mut!(
        /// The applied local shearY.
        applied_shear_y,
        /// Set the applied local shearY.
        set_applied_shear_y,
        ashearY,
        f32
    );
    c_accessor_mut!(
        /// Part of the world transform matrix for the X axis.
        a,
        /// Set part of the world transform matrix for the X axis. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_a,
        a,
        f32
    );
    c_accessor_mut!(
        /// Part of the world transform matrix for the Y axis.
        b,
        /// Set part of the world transform matrix for the Y axis. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_b,
        b,
        f32
    );
    c_accessor_mut!(
        /// Part of the world transform matrix for the X axis.
        c,
        /// Set part of the world transform matrix for the X axis. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_c,
        c,
        f32
    );
    c_accessor_mut!(
        /// Part of the world transform matrix for the Y axis.
        d,
        /// Set part of the world transform matrix for the Y axis. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_d,
        d,
        f32
    );
    c_accessor_mut!(
        /// Set the world X translation.
        world_x,
        /// The world X translation. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_world_x,
        worldX,
        f32
    );
    c_accessor_mut!(
        /// The world Y translation.
        world_y,
        /// Set the world Y translation. If changed,
        /// [`update_applied_transform`](`Self::update_applied_transform`) should be called.
        set_world_y,
        worldY,
        f32
    );
    c_accessor_bool!(sorted, sorted);
    c_accessor_bool!(active, active);
    c_accessor_tmp_ptr_mut!(
        /// The bone's setup pose data.
        data,
        /// The bone's mutable setup pose data.
        data_mut,
        data,
        BoneData,
        spBoneData
    );
    c_accessor_tmp_ptr_optional_mut!(parent, parent_mut, parent, Bone, spBone);
    c_accessor!(children_count, childrenCount, usize);
    c_accessor_array_mut!(
        /// An iterator over the children of this bone.
        ///
        /// ```
        /// # #[path="./test.rs"]
        /// # mod test;
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
        /// # let (skeleton, animation_state) = test::TestAsset::spineboy().instance(true);
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
            spBone_setYDown(i32::from(y_down));
        }
    }

    #[must_use]
    pub fn is_y_down() -> bool {
        unsafe { spBone_isYDown() != 0 }
    }

    c_ptr!(c_bone, spBone);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl Bone {
    /// The local translation.
    #[must_use]
    pub fn translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    /// Set the local translation.
    pub fn set_translation(&mut self, translation: impl Into<Vector2<f32>>) {
        let translation: Vector2<f32> = translation.into();
        self.set_x(translation.x);
        self.set_y(translation.y);
    }

    /// The world translation.
    #[must_use]
    pub fn world_translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    /// Set the world translation.
    pub fn set_world_translation(&mut self, translation: impl Into<Vector2<f32>>) {
        let translation: Vector2<f32> = translation.into();
        self.set_world_x(translation.x);
        self.set_world_y(translation.y);
    }

    /// The applied translation.
    #[must_use]
    pub fn applied_translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    /// Set the applied translation.
    pub fn set_applied_translation(&mut self, translation: impl Into<Vector2<f32>>) {
        let translation: Vector2<f32> = translation.into();
        self.set_applied_x(translation.x);
        self.set_applied_y(translation.y);
    }

    /// The local scale.
    #[must_use]
    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    /// Set the local scale.
    pub fn set_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_scale_x(scale.x);
        self.set_scale_y(scale.y);
    }

    /// The world scale.
    #[must_use]
    pub fn world_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_x(),
            y: self.world_y(),
        }
    }

    /// The applied scale.
    #[must_use]
    pub fn applied_scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.applied_scale_x(),
            y: self.applied_scale_y(),
        }
    }

    /// Set the applied scale.
    pub fn set_applied_scale(&mut self, scale: impl Into<Vector2<f32>>) {
        let scale: Vector2<f32> = scale.into();
        self.set_applied_scale_x(scale.x);
        self.set_applied_scale_y(scale.y);
    }

    /// The local shear.
    #[must_use]
    pub fn shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.shear_x(),
            y: self.shear_y(),
        }
    }

    /// Set the local shear.
    pub fn set_shear(&mut self, shear: impl Into<Vector2<f32>>) {
        let shear: Vector2<f32> = shear.into();
        self.set_shear_x(shear.x);
        self.set_shear_y(shear.y);
    }

    /// The applied shear.
    #[must_use]
    pub fn applied_shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.applied_shear_x(),
            y: self.applied_shear_y(),
        }
    }

    /// Set the applied shear.
    pub fn set_applied_shear(&mut self, shear: impl Into<Vector2<f32>>) {
        let shear: Vector2<f32> = shear.into();
        self.set_applied_shear_x(shear.x);
        self.set_applied_shear_y(shear.y);
    }

    #[must_use]
    pub fn world_rotation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.world_rotation_x(),
            y: self.world_rotation_y(),
        }
    }

    pub fn update_world_transform_with2(
        &mut self,
        translation: mint::Vector2<f32>,
        rotation: f32,
        scale: mint::Vector2<f32>,
        shear: mint::Vector2<f32>,
    ) {
        unsafe {
            spBone_updateWorldTransformWith(
                self.c_ptr(),
                translation.x,
                translation.y,
                rotation,
                scale.x,
                scale.y,
                shear.x,
                shear.y,
            );
        }
    }
}

c_handle_decl!(
    /// A storeable reference to a [`Bone`].
    ///
    /// Can be acquired from any instance of [`Bone`].
    ///
    /// ```
    /// # #[path="./test.rs"]
    /// # mod test;
    /// # use rusty_spine::{AnimationState, EventType, BoneHandle};
    /// # let (skeleton, _) = test::TestAsset::spineboy().instance(true);
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
    unsafe fn new_from_ptr(c_bone_data: *mut spBoneData) -> Self {
        Self {
            c_bone_data: SyncPtr(c_bone_data),
        }
    }
}

impl BoneData {
    c_ptr!(c_bone_data, spBoneData);
    c_accessor_string!(name, name);
    c_accessor!(index, index, usize);
    c_accessor!(length, length, f32);
    c_accessor!(x, x, f32);
    c_accessor!(y, y, f32);
    c_accessor!(rotation, rotation, f32);
    c_accessor!(scale_x, scaleX, f32);
    c_accessor!(scale_y, scaleY, f32);
    c_accessor!(shear_x, shearX, f32);
    c_accessor!(shear_y, shearY, f32);
    c_accessor_bool!(skin_required, skinRequired);
    c_accessor_tmp_ptr_optional!(parent, parent, BoneData, spBoneData);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl BoneData {
    #[must_use]
    pub fn translation(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    #[must_use]
    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    #[must_use]
    pub fn shear(&self) -> Vector2<f32> {
        Vector2 {
            x: self.shear_x(),
            y: self.shear_y(),
        }
    }
}
