use crate::{
    c::{spBoneData, spPhysicsConstraintData},
    c_interface::{NewFromPtr, SyncPtr},
    BoneData,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// [Spine API Reference](https://esotericsoftware.com/spine-api-reference#PhysicsConstraintData)
#[derive(Debug)]
pub struct PhysicsConstraintData {
    c_physics_constraint_data: SyncPtr<spPhysicsConstraintData>,
}

impl NewFromPtr<spPhysicsConstraintData> for PhysicsConstraintData {
    unsafe fn new_from_ptr(c_slot: *mut spPhysicsConstraintData) -> Self {
        Self {
            c_physics_constraint_data: SyncPtr(c_slot),
        }
    }
}

impl PhysicsConstraintData {
    c_accessor_string!(name, name);
    c_accessor!(order, order, i32);
    c_accessor_bool!(skin_required, skinRequired);

    c_accessor!(damping, damping, f32);
    c_accessor_bool!(damping_global, dampingGlobal);
    c_accessor!(gravity, gravity, f32);
    c_accessor_bool!(gravity_global, gravityGlobal);
    c_accessor!(inertia, gravity, f32);
    c_accessor_bool!(inertia_global, inertiaGlobal);
    c_accessor!(limit, limit, f32);
    c_accessor_bool!(mass_global, massGlobal);
    c_accessor!(mass_inverse, massInverse, f32);
    c_accessor!(mix, mix, f32);
    c_accessor_bool!(mix_global, mixGlobal);
    c_accessor!(rotate, rotate, f32);
    c_accessor!(scale_x, scaleX, f32);
    c_accessor!(shear_x, shearX, f32);
    c_accessor!(step, step, f32);
    c_accessor!(strength, strength, f32);
    c_accessor_bool!(strength_global, strengthGlobal);
    c_accessor!(wind, wind, f32);
    c_accessor_bool!(wind_global, windGlobal);
    c_accessor!(x, x, f32);
    c_accessor!(y, y, f32);

    c_accessor_tmp_ptr!(bone, bone, BoneData, spBoneData);

    c_ptr!(c_physics_constraint_data, spPhysicsConstraintData);
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl PhysicsConstraintData {
    #[must_use]
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }
}
