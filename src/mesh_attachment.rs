use crate::{
    c::{c_ushort, spAttachment, spMeshAttachment, spTextureRegion, spVertexAttachment},
    c_interface::NewFromPtr,
    sync_ptr::SyncPtr,
    texture_region::TextureRegion,
};

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
        &self.c_ptr_ref().super_0.super_0
    }

    fn vertex_attachment(&self) -> &spVertexAttachment {
        &self.c_ptr_ref().super_0
    }

    fn vertex_attachment_mut(&mut self) -> &mut spVertexAttachment {
        &mut self.c_ptr_mut().super_0
    }

    c_ptr!(c_mesh_attachment, spMeshAttachment);
    c_attachment_accessors!(super_0.super_0);
    c_vertex_attachment_accessors!(super_0.super_0);
    c_accessor_string!(path, path);
    c_accessor_color!(color, color_mut, color);
    c_accessor!(hull_length, hull_length_mut, hullLength, i32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);
    c_accessor!(triangles_count, triangles_count_mut, trianglesCount, i32);
    c_accessor_renderer_object!();
    c_accessor_tmp_ptr!(region, region_mut, region, TextureRegion, spTextureRegion);
    c_accessor_tmp_ptr!(
        parent_mesh,
        parent_mesh_mut,
        parentMesh,
        MeshAttachment,
        spMeshAttachment
    );

    // TODO: better accessor than passthrough?
    c_accessor_passthrough!(
        triangles,
        triangles_mut,
        triangles,
        *const c_ushort,
        *mut c_ushort
    );

    // TODO: sequence, regionUVs, uvs, parentMesh, edges
}
