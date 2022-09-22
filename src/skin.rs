use crate::{
    c::spSkin,
    c_interface::{NewFromPtr, SyncPtr},
};

#[derive(Debug)]
pub struct Skin {
    c_skin: SyncPtr<spSkin>,
}

impl NewFromPtr<spSkin> for Skin {
    unsafe fn new_from_ptr(c_skin: *const spSkin) -> Self {
        Self {
            c_skin: SyncPtr(c_skin as *mut spSkin),
        }
    }
}

impl Skin {
    c_accessor_string!(name, name);
    c_ptr!(c_skin, spSkin);
    // TODO: accessors
}
