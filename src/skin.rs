use crate::{c::spSkin, c_interface::NewFromPtr, sync_ptr::SyncPtr};

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
    c_ptr!(c_skin, spSkin);
    c_accessor_string!(name, name);
    // TODO: accessors
}
