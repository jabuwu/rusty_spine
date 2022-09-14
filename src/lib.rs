macro_rules! c_ptr {
    ($member:ident, $c_type:ident) => {
        pub fn c_ptr(&self) -> *mut $c_type {
            self.$member.0
        }

        pub fn c_ptr_ref(&self) -> &$c_type {
            unsafe { &*self.$member.0 }
        }

        pub fn c_ptr_mut(&self) -> &mut $c_type {
            unsafe { &mut *self.$member.0 }
        }
    };
}

macro_rules! c_accessor {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ident) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c
        }
        pub fn $rust_mut(&mut self) -> &mut $type {
            &mut self.c_ptr_mut().$c
        }
    };
}

macro_rules! c_accessor_bool {
    ($rust:ident, $rust_set:ident, $c:ident) => {
        pub fn $rust(&self) -> bool {
            self.c_ptr_ref().$c != 0
        }
        pub fn $rust_set(&mut self, value: bool) {
            self.c_ptr_mut().$c = if value { 1 } else { 0 };
        }
    };
}

pub mod animation_state;
pub mod animation_state_data;
pub mod atlas;
pub mod bone;
pub mod c;
pub mod error;
pub mod skeleton;
pub mod skeleton_data;
pub mod skeleton_json;
pub mod sync_ptr;
