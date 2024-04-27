use jc2_file_formats::render_block_model::SimpleVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for SimpleVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                std::mem::offset_of!(SimpleVertex, position),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Normals,
                std::mem::offset_of!(SimpleVertex, normal),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                std::mem::offset_of!(SimpleVertex, uv0),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Tangents,
                std::mem::offset_of!(SimpleVertex, tangent),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
