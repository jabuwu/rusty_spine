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
        c_accessor_for!(c_ptr_ref, c_ptr_mut, $rust, $rust_mut, $c, $type);
    };
}

macro_rules! c_accessor_for {
    ($for:ident, $for_mut:ident, $rust:ident, $rust_mut:ident, $c:ident, $type:ty) => {
        #[inline]
        pub fn $rust(&self) -> $type {
            self.$for().$c
        }
        #[inline]
        pub fn $rust_mut(&mut self) -> &mut $type {
            &mut self.$for_mut().$c
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

macro_rules! c_accessor_renderer_object {
    () => {
        pub fn renderer_object(&self) -> crate::renderer_object::RendererObject {
            crate::renderer_object::RendererObject::new(&mut self.c_ptr_mut().rendererObject)
        }
    };
}

macro_rules! c_accessor_tmp_ptr {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::tmp_ref::TmpRef<Self, $type> {
            crate::tmp_ref::TmpRef::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
        pub fn $rust_mut(&mut self) -> crate::tmp_ref::TmpRefMut<Self, $type> {
            crate::tmp_ref::TmpRefMut::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    self.c_ptr_ref().$c,
                )
            })
        }
    };
}

macro_rules! c_accessor_tmp_ptr_optional {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> Option<crate::tmp_ref::TmpRef<Self, $type>> {
            let ptr = self.c_ptr_ref().$c;
            if !ptr.is_null() {
                Some(crate::tmp_ref::TmpRef::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                }))
            } else {
                None
            }
        }
        pub fn $rust_mut(&mut self) -> Option<crate::tmp_ref::TmpRefMut<Self, $type>> {
            let ptr = self.c_ptr_ref().$c;
            if !ptr.is_null() {
                Some(crate::tmp_ref::TmpRefMut::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(ptr)
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_accessor_super {
    ($rust:ident, $rust_mut:ident, $type:ty, $c_type:ident) => {
        pub fn $rust(&self) -> crate::tmp_ref::TmpRef<Self, $type> {
            crate::tmp_ref::TmpRef::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    &mut self.c_ptr_mut().super_0,
                )
            })
        }
        pub fn $rust_mut(&mut self) -> crate::tmp_ref::TmpRefMut<Self, $type> {
            crate::tmp_ref::TmpRefMut::new(self, unsafe {
                <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                    &mut self.c_ptr_mut().super_0,
                )
            })
        }
    };
}

macro_rules! c_accessor_passthrough {
    ($rust:ident, $rust_mut:ident, $c:ident, $type:ty, $type_mut:ty) => {
        pub fn $rust(&self) -> $type {
            self.c_ptr_ref().$c
        }

        pub fn $rust_mut(&mut self) -> $type_mut {
            self.c_ptr_mut().$c
        }
    };
}

macro_rules! c_accessor_array {
    ($rust:ident, $rust_mut:ident, $rust_index:ident, $rust_index_mut:ident, $parent_type:ident, $type:ident, $c_type:ident, $c:ident, $count_fn:ident) => {
        pub fn $rust(&self) -> crate::c_interface::CArrayIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CArrayIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_mut(
            &mut self,
        ) -> crate::c_interface::CArrayMutIterator<$parent_type, $type, $c_type> {
            crate::c_interface::CArrayMutIterator::new(
                self,
                self.c_ptr_ref().$c,
                self.$count_fn() as usize,
            )
        }

        pub fn $rust_index(&self, index: usize) -> Option<crate::tmp_ref::TmpRef<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::tmp_ref::TmpRef::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_ref().$c.offset(index as isize),
                    )
                }))
            } else {
                None
            }
        }

        pub fn $rust_index_mut(
            &mut self,
            index: usize,
        ) -> Option<crate::tmp_ref::TmpRefMut<Self, $type>> {
            if index < self.$count_fn() as usize {
                Some(crate::tmp_ref::TmpRefMut::new(self, unsafe {
                    <$type as crate::c_interface::NewFromPtr<$c_type>>::new_from_ptr(
                        *self.c_ptr_mut().$c.offset(index as isize),
                    )
                }))
            } else {
                None
            }
        }
    };
}

macro_rules! c_attachment_accessors {
    ($c:expr) => {
        #[inline]
        pub fn name(&self) -> &str {
            unsafe {
                std::ffi::CStr::from_ptr(self.attachment().name)
                    .to_str()
                    .unwrap()
            }
        }

        #[inline]
        pub fn attachment_type(&self) -> crate::attachment::AttachmentType {
            self.attachment().type_0.into()
        }
    };
}

macro_rules! c_vertex_attachment_accessors {
    ($c:expr) => {
        // TODO: fill accessors
        #[inline]
        pub unsafe fn compute_world_vertices(
            &self,
            slot: &crate::slot::Slot,
            start: i32,
            count: i32,
            world_vertices: &mut [f32],
            offset: i32,
            stride: i32,
        ) {
            crate::c::spVertexAttachment_computeWorldVertices(
                self.vertex_attachment() as *const crate::c::spVertexAttachment
                    as *mut crate::c::spVertexAttachment,
                slot.c_ptr(),
                start,
                count,
                world_vertices.as_mut_ptr(),
                offset,
                stride,
            );
        }

        c_accessor_for!(
            vertex_attachment,
            vertex_attachment_mut,
            world_vertices_length,
            world_vertex_length_mut,
            worldVerticesLength,
            i32
        );
        c_accessor_for!(
            vertex_attachment,
            vertex_attachment_mut,
            id,
            id_mut,
            id,
            i32
        );
    };
}

pub mod animation_state;
pub mod animation_state_data;
pub mod atlas;
pub mod attachment;
pub mod bone;
pub mod c;
pub mod c_interface;
pub mod clipping_attachment;
pub mod color;
pub mod error;
pub mod extension;
pub mod mesh_attachment;
pub mod region_attachment;
pub mod renderer_object;
pub mod skeleton;
pub mod skeleton_clipping;
pub mod skeleton_controller;
pub mod skeleton_data;
pub mod skeleton_json;
pub mod slot;
pub mod sync_ptr;
pub mod texture_region;
pub mod tmp_ref;

#[cfg(test)]
pub mod tests;
