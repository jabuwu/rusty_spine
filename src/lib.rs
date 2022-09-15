macro_rules! c_ptr {
    ($member:ident, $c_type:ty) => {
        #[inline]
        pub fn c_ptr(&self) -> *mut $c_type {
            self.$member.0
        }

        #[inline]
        pub fn c_ptr_ref(&self) -> &$c_type {
            unsafe { &*self.$member.0 }
        }

        #[inline]
        pub fn c_ptr_mut(&self) -> &mut $c_type {
            unsafe { &mut *self.$member.0 }
        }
    };
}

macro_rules! c_accessor {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty) => {
        #[inline]
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c
        }
        #[inline]
        pub fn $rust_mut(&mut self) -> &mut $type {
            &mut self.c_ptr_mut().$c
        }
    };
}

macro_rules! c_accessor_string {
    ($rust:ident, $c:ident) => {
        #[inline]
        pub fn $rust(&self) -> &str {
            unsafe {
                std::ffi::CStr::from_ptr(self.c_ptr_ref().$c)
                    .to_str()
                    .unwrap()
            }
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

macro_rules! c_accessor_color {
    ($rust:ident, $rust_mut:ident, $c:ident) => {
        pub fn $rust(&self) -> &crate::color::Color {
            unsafe {
                &*(&self.c_ptr_ref().$c as *const crate::c::spColor as *const crate::color::Color)
            }
        }
        pub fn $rust_mut(&self) -> &mut crate::color::Color {
            unsafe {
                &mut *(&self.c_ptr_ref().$c as *const crate::c::spColor as *mut crate::color::Color)
            }
        }
    };
}

macro_rules! c_accessor_enum_no_set {
    ($rust:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c.into()
        }
    };
}

macro_rules! c_accessor_enum {
    ($rust:ident, $rust_set:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c.into()
        }
        pub fn $rust_set(&mut self, value: $type) {
            self.c_ptr_mut().$c = value as u32;
        }
    };
}

macro_rules! c_accessor_void_ptr {
    ($rust:ident, $rust_mut:ident, $c:ident) => {
        pub fn $rust(&self) -> *const crate::c::c_void {
            self.c_ptr_ref().$c
        }
        pub fn $rust_mut(&mut self) -> &mut *mut crate::c::c_void {
            &mut self.c_ptr_mut().$c
        }
    };
}

macro_rules! c_accessor_tmp_ptr {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty) => {
        pub fn $rust(&self) -> crate::tmp_ref::TmpRef<Self, $type> {
            crate::tmp_ref::TmpRef::new(self, <$type>::new_from_ptr(self.c_ptr_ref().$c))
        }
        pub fn $rust_mut(&mut self) -> crate::tmp_ref::TmpRefMut<Self, $type> {
            crate::tmp_ref::TmpRefMut::new(self, <$type>::new_from_ptr(self.c_ptr_ref().$c))
        }
    };
}

macro_rules! c_accessor_super {
    ($rust:ident, $rust_mut:ident, $type:ty) => {
        pub fn $rust(&self) -> crate::tmp_ref::TmpRef<Self, $type> {
            crate::tmp_ref::TmpRef::new(
                self,
                TextureRegion::new_from_ptr(&mut self.c_ptr_mut().super_0),
            )
        }
        pub fn $rust_mut(&mut self) -> crate::tmp_ref::TmpRefMut<Self, $type> {
            crate::tmp_ref::TmpRefMut::new(
                self,
                TextureRegion::new_from_ptr(&mut self.c_ptr_mut().super_0),
            )
        }
    };
}

macro_rules! c_accessor_passthrough {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $type_mut:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().splits
        }

        pub fn $rust_mut(&mut self) -> $type_mut {
            self.c_ptr_mut().splits
        }
    };
}

macro_rules! c_attachment_accessors {
    ($c:ident) => {
        #[inline]
        pub fn name(&self) -> &str {
            unsafe {
                std::ffi::CStr::from_ptr(self.c_ptr_ref().super_0.name)
                    .to_str()
                    .unwrap()
            }
        }

        #[inline]
        pub fn attachment_type(&self) -> crate::attachment::AttachmentType {
            self.c_ptr_ref().super_0.type_0.into()
        }
    };
}

pub mod animation_state;
pub mod animation_state_data;
pub mod atlas;
pub mod attachment;
pub mod bone;
pub mod c;
pub mod color;
pub mod error;
pub mod extension;
pub mod mesh_attachment;
pub mod region_attachment;
pub mod skeleton;
pub mod skeleton_data;
pub mod skeleton_json;
pub mod slot;
pub mod sync_ptr;
pub mod texture_region;
pub mod tmp_ref;
