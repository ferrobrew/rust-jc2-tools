use std::io::{Cursor, Write};

use gltf_json::{
    buffer::Stride,
    validation::{Checked, USize64},
    Index,
};
use helpers::GltfMeshAccessor;
use jc2_file_formats::render_block_model::RenderBlockModel;

use crate::helpers::GltfHelpers;

mod helpers;

type GltfRoot = gltf_json::Root;
type Buffer = gltf_json::Buffer;
type BufferIndex = Index<Buffer>;
type BufferView = gltf_json::buffer::View;
type BufferViewIndex = Index<BufferView>;

fn create_buffer_view(
    root: &mut GltfRoot,
    name: &str,
    buffer: BufferIndex,
    length: usize,
    offset: usize,
    stride: usize,
) -> BufferViewIndex {
    root.push(BufferView {
        name: Some(name.into()),
        target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
        buffer,
        byte_length: length.into(),
        byte_offset: Some(offset.into()),
        byte_stride: Some(Stride(stride)),
        extensions: Default::default(),
        extras: Default::default(),
    })
}

type MeshPrimitive = gltf_json::mesh::Primitive;
type Accessor = gltf_json::Accessor;
type AccessorComponentType = gltf_json::accessor::ComponentType;
type AccessorIndex = gltf_json::Index<gltf_json::Accessor>;

fn create_accessor(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    buffer_view_index: BufferViewIndex,
    accessor: GltfMeshAccessor,
    count: usize,
) -> AccessorIndex {
    let (accessor_type, component_type, semantic, offset) = accessor;
    let accessor = root.push(Accessor {
        buffer_view: Some(buffer_view_index),
        byte_offset: Some((offset).into()),
        count: count.into(),
        component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(component_type)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Checked::Valid(accessor_type),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    primitive
        .attributes
        .insert(Checked::Valid(semantic), accessor);
    accessor
}

fn create_index_accessor(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    buffer_view_index: BufferViewIndex,
    count: usize,
) -> AccessorIndex {
    let accessor = root.push(Accessor {
        buffer_view: Some(buffer_view_index),
        byte_offset: Some(USize64(0)),
        count: count.into(),
        component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
            AccessorComponentType::U16,
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
    accessor
}

fn create_accessors(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    buffer_view_index: BufferViewIndex,
    accessors: &[GltfMeshAccessor],
    count: usize,
) {
    for accessor in accessors {
        create_accessor(root, primitive, buffer_view_index, accessor.clone(), count);
    }
}

fn create_views_and_accessors<T: Clone + GltfHelpers>(
    root: &mut GltfRoot,
    primitive: &mut MeshPrimitive,
    offset: &mut usize,
    block: &T,
    buffer: BufferIndex,
) {
    let idx = root.meshes.len();

    let length = block.vertices_as_bytes().len();
    let stride = block.vertex_stride();
    let view = create_buffer_view(
        root,
        &format!("vertex_{idx}"),
        buffer,
        length,
        *offset,
        stride,
    );
    create_accessors(
        root,
        primitive,
        view,
        &block.accessors(),
        block.vertex_count(),
    );
    *offset += length;

    let length = block.indices_as_bytes().len();
    let stride = block.index_stride();
    let view = create_buffer_view(
        root,
        &format!("index_{idx}"),
        buffer,
        length,
        *offset,
        stride,
    );
    create_index_accessor(root, primitive, view, block.index_count());
    *offset += length;
}

fn main() -> anyhow::Result<()> {
    let data = include_bytes!("../res/babypanay/MC05_LOD1-BabyPanay.rbm") as &[u8];
    let rbm = RenderBlockModel::read(&mut Cursor::new(data))?;

    // First pass, calculate necessary buffer size, and round up to nearest multiple of 4
    let mut buffer_size = 0;

    for block in rbm.blocks.iter() {
        buffer_size += block.vertices_as_bytes().len();
        buffer_size += block.indices_as_bytes().len();
    }

    buffer_size = (buffer_size + 3) & !3;

    // Second pass create the final buffer
    let mut buffer = Vec::with_capacity(buffer_size);

    for block in rbm.blocks.iter() {
        buffer.extend_from_slice(block.vertices_as_bytes());
        buffer.extend_from_slice(block.indices_as_bytes());
    }

    buffer.resize(buffer_size, 0);

    // Create the gltf buffer
    let mut writer = std::fs::File::create("Test.bin")?;
    writer.write_all(&buffer)?;

    let mut root = gltf_json::Root::default();
    let buffer = root.push(Buffer {
        name: Some("buffer".into()),
        uri: Some("Test.bin".into()),
        byte_length: buffer_size.into(),
        extensions: Default::default(),
        extras: Default::default(),
    });

    // Next pass, create the final gltf
    let mut primitives: Vec<MeshPrimitive> = Vec::with_capacity(rbm.blocks.len());
    let mut buffer_offset = 0;

    let default_primitive = MeshPrimitive {
        attributes: Default::default(),
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Checked::Invalid,
        targets: None,
    };

    for block in rbm.blocks.iter() {
        let mut primitive = MeshPrimitive {
            mode: Checked::Valid(block.mesh_mode()),
            ..default_primitive.clone()
        };
        create_views_and_accessors(&mut root, &mut primitive, &mut buffer_offset, block, buffer);
        primitives.push(primitive);
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
