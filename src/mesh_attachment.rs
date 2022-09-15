use crate::{c::spMeshAttachment, sync_ptr::SyncPtr, texture_region::TextureRegion};

#[derive(Debug)]
pub struct MeshAttachment {
    c_mesh_attachment: SyncPtr<spMeshAttachment>,
}

impl MeshAttachment {
    pub fn new_from_ptr(c_mesh_attachment: *const spMeshAttachment) -> Self {
        Self {
            c_mesh_attachment: SyncPtr(c_mesh_attachment as *mut spMeshAttachment),
        }
    }

    c_ptr!(c_mesh_attachment, spMeshAttachment);
    //TODO c_attachment_accessors!(c_region_attachment);
    //TODO c_accessor_super!(vertex_attachment, vertex_attachment_mut, VertexAttachment);
    c_accessor_string!(path, path);
    c_accessor_color!(color, color_mut, color);
    c_accessor!(hull_length, hull_length_mut, hullLength, i32);
    c_accessor!(width, width_mut, width, f32);
    c_accessor!(height, height_mut, height, f32);
    c_accessor_void_ptr!(renderer_object, renderer_object_mut, rendererObject);
    c_accessor_tmp_ptr!(region, region_mut, region, TextureRegion);
    c_accessor_tmp_ptr!(parent_mesh, parent_mesh_mut, parentMesh, MeshAttachment);

    // TODO: sequence, regionUVs, uvs, triangles, parentMesh, edges
}
