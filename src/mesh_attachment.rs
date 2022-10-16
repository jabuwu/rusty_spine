use crate::{
    c::{
        c_float, c_ushort, spAttachment, spMeshAttachment, spMeshAttachment_newLinkedMesh,
        spVertexAttachment,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Attachment,
};

#[cfg(not(feature="spine38"))]
use crate::{
    c::{
        spMeshAttachment_updateRegion, spTextureRegion
    },
    texture_region::TextureRegion,
};

#[cfg(feature="spine38")]
use crate::c::spMeshAttachment_updateUVs;

#[cfg(feature = "mint")]
use mint::Vector2;

/// A deforming mesh attachment.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#MeshAttachment)
#[derive(Debug)]
pub struct MeshAttachment {
    c_mesh_attachment: SyncPtr<spMeshAttachment>,
}

impl NewFromPtr<spMeshAttachment> for MeshAttachment {
    unsafe fn new_from_ptr(c_mesh_attachment: *const spMeshAttachment) -> Self {
        Self {
            c_mesh_attachment: SyncPtr(c_mesh_attachment as *mut spMeshAttachment),
        }
    }
}

impl MeshAttachment {
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0.super_0 }
    }

    fn vertex_attachment(&self) -> &spVertexAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    pub unsafe fn new_linked_mesh(&self) -> Attachment {
        Attachment::new_from_ptr(spMeshAttachment_newLinkedMesh(self.c_ptr()) as *const spAttachment)
    }

    #[cfg(not(feature="spine38"))]
    pub unsafe fn update_region(&mut self) {
        spMeshAttachment_updateRegion(self.c_ptr());
    }

    #[cfg(feature="spine38")]
    pub unsafe fn update_uvs(&mut self) { spMeshAttachment_updateUVs(self.c_ptr()); }

    c_attachment_accessors!();
    c_vertex_attachment_accessors!();
    c_accessor_string!(path, path);
    c_accessor_color!(color, color);
    c_accessor!(hull_length, hullLength, i32);
    c_accessor!(width, width, f32);
    c_accessor!(height, height, f32);
    c_accessor_renderer_object!();
    #[cfg(not(feature="spine38"))]
    c_accessor_tmp_ptr_optional!(region, region_mut, region, TextureRegion, spTextureRegion);
    c_accessor_tmp_ptr!(
        parent_mesh,
        parent_mesh_mut,
        parentMesh,
        MeshAttachment,
        spMeshAttachment
    );
    c_accessor!(triangles_count, trianglesCount, i32);
    c_accessor_passthrough!(triangles, triangles, *mut c_ushort);
    c_accessor!(edges_count, edgesCount, i32);
    c_accessor_passthrough!(edges, edges, *mut i32);
    c_accessor_passthrough!(uvs, uvs, *mut c_float);
    c_accessor_passthrough!(region_uvs, regionUVs, *mut c_float);
    c_ptr!(c_mesh_attachment, spMeshAttachment);
    // TODO: sequence accessor
}

#[cfg(feature = "mint")]
impl MeshAttachment {
    pub fn size(&self) -> Vector2<f32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }
}
