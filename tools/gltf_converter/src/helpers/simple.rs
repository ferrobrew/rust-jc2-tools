use jc2_file_formats::render_block_model::SimpleVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for SimpleVertex {
    fn get_mesh_accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                bytemuck::offset_of!(SimpleVertex, position),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Normals,
                bytemuck::offset_of!(SimpleVertex, normal),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                bytemuck::offset_of!(SimpleVertex, uv0),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Tangents,
                bytemuck::offset_of!(SimpleVertex, tangent),
            ),
        ]
    }
}
