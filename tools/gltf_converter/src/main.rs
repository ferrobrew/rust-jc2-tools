use std::{
    io::{Cursor, Write},
    mem,
};

use gltf_json::{
    buffer::Stride,
    validation::{Checked, USize64},
    Index,
};
use jc2_file_formats::render_block_model::{GeneralVertex, RenderBlock, RenderBlockModel};

type GltfRoot = gltf_json::Root;
type Buffer = gltf_json::Buffer;
type BufferView = gltf_json::buffer::View;
type BufferViewIndex = Index<BufferView>;

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

fn create_buffer_view<T: Clone>(root: &mut GltfRoot, name: &str, buffer: &[T]) -> BufferViewIndex {
    let uri = format!("{name}.bin");
    let mut writer = std::fs::File::create(&uri).expect("failed to create writer");
    writer
        .write_all(&to_padded_byte_vector(buffer.to_vec()))
        .expect("failed to write");

    let byte_length = mem::size_of_val(buffer).into();
    let buffer = root.push(Buffer {
        name: Some(name.into()),
        uri: uri.into(),
        byte_length,
        extensions: Default::default(),
        extras: Default::default(),
    });

    root.push(BufferView {
        name: Some(name.into()),
        target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
        buffer,
        byte_length,
        byte_offset: None,
        byte_stride: Some(Stride(mem::size_of::<T>())),
        extensions: Default::default(),
        extras: Default::default(),
    })
}

type MeshPrimitive = gltf_json::mesh::Primitive;
type MeshSemantic = gltf_json::mesh::Semantic;
type Accessor = gltf_json::Accessor;
type AccessorType = gltf_json::accessor::Type;
type AccessorIndex = gltf_json::Index<gltf_json::Accessor>;

fn create_accessor(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    buffer_view_index: BufferViewIndex,
    accessor_type: AccessorType,
    mesh_semantic: MeshSemantic,
    byte_offset: usize,
) -> Option<AccessorIndex> {
    if let Some(buffer_view) = root.get(buffer_view_index) {
        let count = {
            let mut length = buffer_view.byte_length.0;
            if let Some(offset) = buffer_view.byte_offset {
                length -= offset.0;
            }
            if let Some(stride) = buffer_view.byte_stride {
                length /= stride.0 as u64;
            }
            length
        };
        let accessor = root.push(Accessor {
            buffer_view: Some(buffer_view_index),
            byte_offset: Some(byte_offset.into()),
            count: count.into(),
            component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
                gltf_json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Checked::Valid(accessor_type),
            min: None,
            max: None,
            name: None,
            normalized: matches!(
                mesh_semantic,
                MeshSemantic::Normals | MeshSemantic::Tangents
            ),
            sparse: None,
        });
        primitive
            .attributes
            .insert(Checked::Valid(mesh_semantic), accessor);
        Some(accessor)
    } else {
        None
    }
}

type AccessorComponentType = gltf_json::accessor::ComponentType;

fn create_index_accessor(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    buffer_view_index: BufferViewIndex,
    component_type: AccessorComponentType,
) -> Option<AccessorIndex> {
    if let Some(buffer_view) = root.get(buffer_view_index) {
        let count = {
            let mut length = buffer_view.byte_length.0;
            // TODO: can probably go
            if let Some(offset) = buffer_view.byte_offset {
                length -= offset.0;
            }
            if let Some(stride) = buffer_view.byte_stride {
                length /= stride.0 as u64;
            }
            length
        };
        let accessor = root.push(Accessor {
            buffer_view: Some(buffer_view_index),
            byte_offset: Some(USize64(0)),
            count: count.into(),
            component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
                component_type,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Checked::Valid(gltf_json::accessor::Type::Scalar),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
        });
        primitive.indices = Some(accessor);
        Some(accessor)
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let sharkatron_rbm = include_bytes!("../res/statictram/v110_lod1-body.rbm") as &[u8];
    let rbm = RenderBlockModel::read(&mut Cursor::new(sharkatron_rbm))?;

    let mut root = gltf_json::Root::default();
    let mut primitives: Vec<MeshPrimitive> = Vec::with_capacity(rbm.blocks.len());

    for block in rbm.blocks.iter() {
        match block {
            RenderBlock::General(general) => {
                let mut primitive = MeshPrimitive {
                    extensions: Default::default(),
                    extras: Default::default(),
                    indices: None,
                    material: None,
                    mode: Checked::Valid(gltf_json::mesh::Mode::Triangles),
                    targets: None,
                    attributes: Default::default(),
                };
                let vertices_view = create_buffer_view(&mut root, "vertices", &general.vertices);

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec3,
                    MeshSemantic::Positions,
                    bytemuck::offset_of!(GeneralVertex, position),
                );

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec2,
                    MeshSemantic::TexCoords(0),
                    bytemuck::offset_of!(GeneralVertex, uv0),
                );

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec2,
                    MeshSemantic::TexCoords(1),
                    bytemuck::offset_of!(GeneralVertex, uv1),
                );

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec3,
                    MeshSemantic::Normals,
                    bytemuck::offset_of!(GeneralVertex, normal),
                );

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec3,
                    MeshSemantic::Tangents,
                    bytemuck::offset_of!(GeneralVertex, tangent),
                );

                create_accessor(
                    &mut root,
                    &mut primitive,
                    vertices_view,
                    AccessorType::Vec4,
                    MeshSemantic::Colors(0),
                    bytemuck::offset_of!(GeneralVertex, color),
                );

                let index_view = create_buffer_view(&mut root, "indices", &general.indices);

                create_index_accessor(
                    &mut root,
                    &mut primitive,
                    index_view,
                    AccessorComponentType::U16,
                );

                primitives.push(primitive);
            }
            RenderBlock::Lambert(_) => todo!(),
        }
    }

    let mesh = root.push(gltf_json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives,
        weights: None,
    });

    root.push(gltf_json::Node {
        mesh: Some(mesh),
        camera: Default::default(),
        children: Default::default(),
        extensions: Default::default(),
        extras: Default::default(),
        matrix: Default::default(),
        name: Default::default(),
        rotation: Default::default(),
        scale: Default::default(),
        translation: Default::default(),
        skin: Default::default(),
        weights: Default::default(),
    });

    let writer = std::fs::File::create("Test.gltf")?;
    gltf_json::serialize::to_writer_pretty(writer, &root)?;

    Ok(())
}
