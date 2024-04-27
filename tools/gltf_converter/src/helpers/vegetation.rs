use jc2_file_formats::render_block_model::VegetationVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for VegetationVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                std::mem::offset_of!(VegetationVertex, position),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                std::mem::offset_of!(VegetationVertex, uv0),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(1),
                std::mem::offset_of!(VegetationVertex, uv1),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Colors(0),
                std::mem::offset_of!(VegetationVertex, color),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Normals,
                std::mem::offset_of!(VegetationVertex, normal),
            ),
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Tangents,
                std::mem::offset_of!(VegetationVertex, tangent),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
