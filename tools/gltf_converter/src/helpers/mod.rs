use std::{mem::size_of_val, slice};

use jc2_file_formats::render_block_model::RenderBlock;

mod deformable;
mod general;
mod simple;
mod skinned;

type AccessorType = gltf_json::accessor::Type;
type AccessorComponentType = gltf_json::accessor::ComponentType;
type MeshSemantic = gltf_json::mesh::Semantic;

pub type GltfMeshAccessor = (AccessorType, AccessorComponentType, MeshSemantic, usize);

pub trait GltfMeshAccessors {
    fn get_mesh_accessors() -> Vec<GltfMeshAccessor>;
}

pub trait GltfHelpers {
    fn get_index_buffer(&self) -> &[u8];
    fn get_vertex_buffer(&self) -> &[u8];
    fn get_mesh_accessors(&self) -> Vec<GltfMeshAccessor>;
}

#[inline]
fn bytes<T>(value: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(value.as_ptr() as *const u8, size_of_val(value)) }
}

#[inline]
fn accessors<T: GltfMeshAccessors>(_: &[T]) -> Vec<GltfMeshAccessor> {
    T::get_mesh_accessors()
}

impl GltfHelpers for RenderBlock {
    #[inline]
    fn get_vertex_buffer(&self) -> &[u8] {
        match self {
            RenderBlock::CarPaint(data) => bytes(&data.vertices),
            RenderBlock::CarPaintSimple(data) => bytes(&data.vertices),
            RenderBlock::General(data) => bytes(&data.vertices),
            RenderBlock::Lambert(data) => bytes(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => bytes(&data.vertices),
        }
    }

    #[inline]
    fn get_index_buffer(&self) -> &[u8] {
        match self {
            RenderBlock::CarPaint(data) => bytes(&data.indices),
            RenderBlock::CarPaintSimple(data) => bytes(&data.indices),
            RenderBlock::General(data) => bytes(&data.indices),
            RenderBlock::Lambert(data) => bytes(&data.indices),
            RenderBlock::SkinnedGeneral(data) => bytes(&data.indices),
        }
    }

    #[inline]
    fn get_mesh_accessors(&self) -> Vec<GltfMeshAccessor> {
        match self {
            RenderBlock::CarPaint(data) => accessors(&data.vertices),
            RenderBlock::CarPaintSimple(data) => accessors(&data.vertices),
            RenderBlock::General(data) => accessors(&data.vertices),
            RenderBlock::Lambert(data) => accessors(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => accessors(&data.vertices),
        }
    }
}
