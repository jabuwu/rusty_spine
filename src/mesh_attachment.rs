use crate::{
    c::{
        c_float, c_ushort, spAttachment, spMeshAttachment, spMeshAttachment_newLinkedMesh,
        spVertexAttachment,
    },
    c_interface::{NewFromPtr, SyncPtr},
    Attachment,
};

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
    unsafe fn new_from_ptr(c_mesh_attachment: *mut spMeshAttachment) -> Self {
        Self {
            c_mesh_attachment: SyncPtr(c_mesh_attachment),
        }
    }
}

impl MeshAttachment {
    #[must_use]
    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0.super_0 }
    }

    #[must_use]
    fn vertex_attachment(&self) -> &spVertexAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    #[must_use]
    pub fn new_linked_mesh(&self) -> Attachment {
        unsafe {
            Attachment::new_from_ptr(
                spMeshAttachment_newLinkedMesh(self.c_ptr()).cast::<spAttachment>(),
            )
        }
    }

    c_attachment_accessors!();
    c_vertex_attachment_accessors!();
    c_accessor_string!(path, path);
    c_accessor_color!(color, color);
    c_accessor!(hull_length, hullLength, i32);
    c_accessor!(width, width, f32);
    c_accessor!(height, height, f32);
    c_accessor_renderer_object!();
    c_accessor_tmp_ptr_mut!(
        parent_mesh,
        parent_mesh_mut,
        parentMesh,
        MeshAttachment,
        spMeshAttachment
    );
    c_accessor!(triangles_count, trianglesCount, i32);
    c_accessor_passthrough!(triangles, triangles, *mut c_ushort);
    c_accessor!(edges_count, edgesCount, usize);
    c_accessor_passthrough!(edges, edges, *mut i32);
    c_accessor_passthrough!(uvs, uvs, *mut c_float);
    c_accessor_passthrough!(region_uvs, regionUVs, *mut c_float);
    c_ptr!(c_mesh_attachment, spMeshAttachment);
    // TODO: sequence accessor
}

/// Functions available if using the `mint` feature.
#[cfg(feature = "mint")]
impl MeshAttachment {
    #[must_use]
    pub fn size(&self) -> Vector2<f32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }

    c_vertex_attachment_accessors_mint!();
}
