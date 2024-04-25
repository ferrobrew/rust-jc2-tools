use jc2_file_formats::render_block_model::BillboardFoliageVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for BillboardFoliageVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        vec![
            (
                AccessorType::Vec3,
                AccessorComponentType::F32,
                MeshSemantic::Positions,
                bytemuck::offset_of!(BillboardFoliageVertex, position),
            ),
            (
                AccessorType::Vec2,
                AccessorComponentType::F32,
                MeshSemantic::TexCoords(0),
                bytemuck::offset_of!(BillboardFoliageVertex, uv0),
            ),
        ]
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
