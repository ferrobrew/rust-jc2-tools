use jc2_file_formats::render_block_model::HaloVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for HaloVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                bytemuck::offset_of!(HaloVertex, position),
            ),
            (
                AccessorType::Vec4,
                AccessorComponentType::F32,
                MeshSemantic::Colors(0),
                bytemuck::offset_of!(HaloVertex, color),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                bytemuck::offset_of!(HaloVertex, uv0),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
