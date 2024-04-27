use std::mem::size_of;

use jc2_file_formats::render_block_model::SkinnedVertex;

use crate::AccessorComponentType;

use super::{AccessorType, GltfMeshAccessor, GltfMeshAccessors, MeshSemantic};

impl GltfMeshAccessors for SkinnedVertex {
    fn accessors() -> Vec<GltfMeshAccessor> {
        let mut result = vec![(
            AccessorType::Vec3,
            AccessorComponentType::F32,
            MeshSemantic::Positions,
            std::mem::offset_of!(SkinnedVertex, position),
        )];
        for i in 0..8 {
            result.push((
                AccessorType::Scalar,
                AccessorComponentType::F32,
                MeshSemantic::Weights(i),
                std::mem::offset_of!(SkinnedVertex, bone_weights) + size_of::<f32>() * i as usize,
            ));
            result.push((
                AccessorType::Scalar,
                AccessorComponentType::U32,
                MeshSemantic::Joints(i),
                std::mem::offset_of!(SkinnedVertex, bone_indices) + size_of::<u32>() * i as usize,
            ));
        }
        result.push((
            AccessorType::Vec3,
            AccessorComponentType::F32,
            MeshSemantic::Normals,
            std::mem::offset_of!(SkinnedVertex, normal),
        ));
        result.push((
            AccessorType::Vec3,
            AccessorComponentType::F32,
            MeshSemantic::Tangents,
            std::mem::offset_of!(SkinnedVertex, tangent),
        ));
        result.push((
            AccessorType::Vec2,
            AccessorComponentType::F32,
            MeshSemantic::TexCoords(0),
            std::mem::offset_of!(SkinnedVertex, uv0),
        ));
        result
    }

    fn target_accessors() -> Option<Vec<GltfMeshAccessor>> {
        None
    }
}
