use jc2_file_formats::render_block_model::GeneralVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for GeneralVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                bytemuck::offset_of!(GeneralVertex, position),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                bytemuck::offset_of!(GeneralVertex, uv0),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(1),
                bytemuck::offset_of!(GeneralVertex, uv1),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Normals,
                bytemuck::offset_of!(GeneralVertex, normal),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Tangents,
                bytemuck::offset_of!(GeneralVertex, tangent),
            ),
            (
                AccessorType::Vec4,
                AccessorComponentType::F32,
                MeshSemantic::Colors(0),
                bytemuck::offset_of!(GeneralVertex, color),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
