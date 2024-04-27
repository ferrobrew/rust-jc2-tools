use jc2_file_formats::render_block_model::FacadeVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for FacadeVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                std::mem::offset_of!(FacadeVertex, position),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                std::mem::offset_of!(FacadeVertex, uv0),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(1),
                std::mem::offset_of!(FacadeVertex, uv1),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Normals,
                std::mem::offset_of!(FacadeVertex, normal),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Tangents,
                std::mem::offset_of!(FacadeVertex, tangent),
            ),
            (
                AccessorType::Vec4,
                AccessorComponentType::F32,
                MeshSemantic::Colors(0),
                std::mem::offset_of!(FacadeVertex, color),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(2),
                std::mem::offset_of!(FacadeVertex, uv2),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
