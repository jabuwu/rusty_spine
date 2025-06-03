use super::{
    c::{
        spBone, spPhysics, spPhysicsConstraint, spPhysicsConstraintData, spPhysicsConstraint_reset,
        spPhysicsConstraint_rotate, spPhysicsConstraint_setToSetupPose,
        spPhysicsConstraint_translate, spPhysicsConstraint_update,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Bone, Physics, PhysicsConstraintData,
};

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#PhysicsConstraint)
#[derive(Debug)]
pub struct PhysicsConstraint {
    c_physics_constraint: SyncPtr<spPhysicsConstraint>,
}

impl NewFromPtr<spPhysicsConstraint> for PhysicsConstraint {
    unsafe fn new_from_ptr(c_slot: *mut spPhysicsConstraint) -> Self {
        Self {
            c_physics_constraint: SyncPtr(c_slot),
        }
    }
}

impl PhysicsConstraint {
    pub fn reset(&self) {
        unsafe {
            spPhysicsConstraint_reset(self.c_physics_constraint.0);
        }
    }

    /// Applies the constraint to the constrained bones.
    pub fn update(&self, physics: Physics) {
        unsafe {
            spPhysicsConstraint_update(self.c_physics_constraint.0, physics as spPhysics);
        }
    }

    pub fn set_to_setup_pose(&self) {
        unsafe {
            spPhysicsConstraint_setToSetupPose(self.c_physics_constraint.0);
        }
    }

    /// Translates the physics constraint so next update forces are applied as if the bone moved an
    /// additional amount in world space.
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe {
            spPhysicsConstraint_translate(self.c_physics_constraint.0, x, y);
        }
    }

    /// Rotates the physics constraint so next update forces are applied as if the bone rotated
    /// around the specified point in world space.
    pub fn rotate(&mut self, x: f32, y: f32, degrees: f32) {
        unsafe {
            spPhysicsConstraint_rotate(self.c_physics_constraint.0, x, y, degrees);
        }
    }

    c_accessor_tmp_ptr_mut!(
        /// The physics constraint's setup pose data.
        data,
        data_mut,
        data,
        PhysicsConstraintData,
        spPhysicsConstraintData
    );

    c_accessor_bool!(active, active);
    c_accessor_mut!(damping, set_damping, damping, f32);
    c_accessor_mut!(gravity, set_gravity, gravity, f32);
    c_accessor_mut!(inertia, set_intertia, inertia, f32);
    c_accessor_mut!(mass_inverse, set_mass_inverse, massInverse, f32);
    c_accessor_mut!(
        /// A percentage (0-1) that controls the mix between the constrained and unconstrained poses.
        mix,
        set_mix,
        mix,
        f32
    );
    c_accessor_mut!(strength, set_strength, strength, f32);
    c_accessor_mut!(wind, set_wind, wind, f32);

    c_accessor_tmp_ptr_mut!(
        /// The bone constrained by this physics constraint.
        bone,
        bone_mut,
        bone,
        Bone,
        spBone
    );

    c_ptr!(c_physics_constraint, spPhysicsConstraint);
}
