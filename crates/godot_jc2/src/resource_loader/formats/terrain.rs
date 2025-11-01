use std::ops::{Index, IndexMut};

use binrw::BinRead;
use godot::{
    classes::{
        MeshInstance3D, StandardMaterial3D, Texture2D,
        base_material_3d::{Flags, TextureFilter, TextureParam},
        mesh::PrimitiveType,
    },
    prelude::*,
};
use godot_utils::mesh_builder::MeshBuilder;
use jc2_file_formats::terrain::{TerrainChunk, TerrainMesh};

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread, JcTexture};

pub struct JcTerrain();

impl JcResourceFormat for JcTerrain {
    const EXTENSIONS: [&str; 1] = ["dat"];
    type Result = MeshInstance3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let mut cursor = binrw::io::Cursor::new(buffer.as_slice());
        match TerrainChunk::read_le(&mut cursor) {
            Ok(chunk) => {
                let map_tile = texture(&path, &chunk.textures.map_tile, thread)?;

                let mesh = TerrainChunkMesh::new(
                    {
                        let parts = path.split("_");
                        let length = parts.len();
                        Vector2i::new(
                            parts[length - 2].to_int() as i32,
                            parts[length - 1].to_int() as i32,
                        )
                    },
                    &chunk.lods.low[0],
                );

                let mesh = MeshBuilder::new()
                    .surface(|surface| {
                        let mut material = StandardMaterial3D::new_gd();

                        material.set_flag(Flags::USE_TEXTURE_REPEAT, false);
                        material.set_texture_filter(TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC);
                        material.set_texture(TextureParam::ALBEDO, &map_tile);

                        let vertices: PackedVector3Array = mesh
                            .vertices()
                            .iter()
                            .map(|vertex| {
                                const WIDTH_SCALE: f32 = (1.0 / 64.0) * 512.0;
                                const HEIGHT_SCALE: f32 = (1.0 / u16::MAX as f32) * 2200.0;

                                let [x, z] = vertex.position;
                                let height_map = |x| 2 + (x as usize * 2);
                                let y = chunk.height_map[height_map(x) + height_map(z) * 132];

                                Vector3 {
                                    x: x as f32 * WIDTH_SCALE,
                                    y: y as f32 * HEIGHT_SCALE,
                                    z: z as f32 * WIDTH_SCALE,
                                }
                            })
                            .collect();

                        let uv1: PackedVector2Array = mesh
                            .vertices()
                            .iter()
                            .map(|vertex| {
                                const INVERSE_DIVISIONS: f32 = 1.0 / MAX_DIVISIONS as f32;
                                Vector2 {
                                    x: vertex.position[0] as f32 * INVERSE_DIVISIONS,
                                    y: vertex.position[1] as f32 * INVERSE_DIVISIONS,
                                }
                            })
                            .collect();

                        let indices: PackedInt32Array = mesh
                            .indices()
                            .iter()
                            .flat_map(|indices| indices.iter().rev())
                            .map(|&index| index as i32)
                            .collect();

                        surface
                            .primitive_type(PrimitiveType::TRIANGLES)
                            .material(material)
                            .vertices(vertices)
                            .uv1(uv1)
                            .indices(indices)
                    })
                    .build();

                let mut instance = MeshInstance3D::new_alloc();
                instance.set_mesh(&mesh);
                Ok(instance)
            }
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

fn texture(
    path: &GString,
    buffer: &[u8],
    thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Texture2D>> {
    let result = JcTexture::from_buffer(path.clone(), PackedByteArray::from(buffer), thread)?;
    Ok(result.upcast::<Texture2D>())
}

pub const MAX_DIVISIONS: usize = 64;
pub const MAX_POINTS: usize = MAX_DIVISIONS + 1;
pub const MAX_VERTICES: usize = MAX_POINTS * MAX_POINTS;
pub const MAX_QUADRANTS: usize = MAX_DIVISIONS * MAX_DIVISIONS;
pub const MAX_TRIANGLES: usize = MAX_QUADRANTS * 2;
pub const MAX_INDICES: usize = MAX_TRIANGLES * 3;
pub const MAX_BUCKET_SIZE: usize = MAX_INDICES / 4;

#[derive(Debug, Clone)]
struct TerrainChunkMesh {
    vertex_count: u16,
    vertices: Vertices,
    indices: Indices,
    quad_index_count: [usize; 4],
    quad_indices: [u16; MAX_INDICES],
}

impl Default for TerrainChunkMesh {
    fn default() -> Self {
        Self {
            vertex_count: Default::default(),
            vertices: Default::default(),
            indices: Default::default(),
            quad_index_count: Default::default(),
            quad_indices: [0u16; MAX_INDICES],
        }
    }
}

impl TerrainChunkMesh {
    pub fn new(position: Vector2i, chunk: &TerrainMesh) -> Self {
        let mut result = Self::default();

        result.initialize();

        let mut triangles = chunk.triangles.iter().by_vals();

        if (position.x + position.y) % 2 == 1 {
            result.divide([0, 2, 1], 0, &mut triangles);
            result.divide([3, 1, 2], 1, &mut triangles);
        } else {
            result.divide([1, 0, 3], 0, &mut triangles);
            result.divide([2, 3, 0], 1, &mut triangles);
        }

        result
    }

    fn initialize(&mut self) {
        let vertex = &mut self.vertices[self.vertex_count];
        vertex.position = [0, 0];
        vertex.vertex_index = self.vertex_count;
        vertex.parent_indices = [self.vertex_count; 3];
        self.indices[Indices::index_of(vertex.position)] = self.vertex_count;
        self.vertex_count += 1;

        let vertex = &mut self.vertices[self.vertex_count];
        vertex.position = [64, 0];
        vertex.vertex_index = self.vertex_count;
        vertex.parent_indices = [self.vertex_count; 3];
        self.indices[Indices::index_of(vertex.position)] = self.vertex_count;
        self.vertex_count += 1;

        let vertex = &mut self.vertices[self.vertex_count];
        vertex.position = [0, 64];
        vertex.vertex_index = self.vertex_count;
        vertex.parent_indices = [self.vertex_count; 3];
        self.indices[Indices::index_of(vertex.position)] = self.vertex_count;
        self.vertex_count += 1;

        let vertex = &mut self.vertices[self.vertex_count];
        vertex.position = [64, 64];
        vertex.vertex_index = self.vertex_count;
        vertex.parent_indices = [self.vertex_count; 3];
        self.indices[Indices::index_of(vertex.position)] = self.vertex_count;
        self.vertex_count += 1;
    }

    fn divide(&mut self, triangle: [u16; 3], code: u16, triangles: &mut dyn Iterator<Item = bool>) {
        #[derive(Default, Clone)]
        struct QueuedTriangle {
            vertices: [u16; 3],
            parent_vertices: [u16; 3],
            code: u16,
            depth: u16,
        }

        let mut queue: [QueuedTriangle; 256] = std::array::from_fn(|_| QueuedTriangle::default());
        let mut queue_size = 0usize;

        // Push root triangle
        queue[queue_size].vertices = triangle;
        queue[queue_size].parent_vertices = triangle;
        queue[queue_size].code = code;
        queue_size += 1;

        // Recursively divide quad
        while queue_size > 0 {
            queue_size -= 1;

            // Fetch corners
            let triangle = queue[queue_size].clone();
            let v0 = &self.vertices[triangle.vertices[0]];
            let v1 = &self.vertices[triangle.vertices[1]];
            let v2 = &self.vertices[triangle.vertices[2]];

            if triangles.next().expect("unexpected end of stream") {
                // Calculate mid-point + index
                let position = [
                    (v1.position[0] + v2.position[0]) / 2,
                    (v1.position[1] + v2.position[1]) / 2,
                ];
                let index = Indices::index_of(position);

                // Create vertex
                if self.indices[index] == u16::MAX {
                    let v = &mut self.vertices[self.vertex_count];
                    v.vertex_index = self.vertex_count;
                    v.position = position;
                    v.parent_indices = triangle.parent_vertices;

                    self.indices[index] = self.vertex_count;
                    self.vertex_count += 1;
                }

                // Push left triangle
                let next_triangle = &mut queue[queue_size];
                next_triangle.vertices = [
                    self.indices[index],
                    triangle.vertices[2],
                    triangle.vertices[0],
                ];
                next_triangle.parent_vertices = triangle.parent_vertices;
                next_triangle.code = (triangle.code / 2) + 1;
                next_triangle.depth = triangle.depth + 1;
                queue_size += 1;

                // Push right triangle
                let next_triangle = &mut queue[queue_size];
                next_triangle.vertices = [
                    self.indices[index],
                    triangle.vertices[0],
                    triangle.vertices[1],
                ];
                next_triangle.parent_vertices = triangle.parent_vertices;
                next_triangle.code = triangle.code / 2;
                next_triangle.depth = triangle.depth + 1;
                queue_size += 1;
            } else {
                // Calculate bucket
                let bucket_index = {
                    const H: u8 = (MAX_DIVISIONS / 2) as u8;
                    let x = v0.position[0] >= H && v1.position[0] >= H && v2.position[0] >= H;
                    let y = v0.position[1] >= H && v1.position[1] >= H && v2.position[1] >= H;
                    x as usize + (y as usize * 2)
                };

                // Create triangle
                let offset = self.quad_index_count[bucket_index] + (bucket_index * MAX_BUCKET_SIZE);
                self.quad_indices[offset] = v0.vertex_index;
                self.quad_indices[offset + 1] = v1.vertex_index;
                self.quad_indices[offset + 2] = v2.vertex_index;
                self.quad_index_count[bucket_index] += 3;
                continue;
            }
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices.0[0..self.vertex_count as usize]
    }

    pub fn indices(&self) -> [&[u16]; 4] {
        let indices = |bucket| {
            let start = MAX_BUCKET_SIZE * bucket;
            let end = start + self.quad_index_count[bucket];
            &self.quad_indices[start..end]
        };

        [indices(0), indices(1), indices(2), indices(3)]
    }
}

#[derive(Debug, Default, Clone)]
struct Vertex {
    position: [u8; 2],
    vertex_index: u16,
    parent_indices: [u16; 3],
}

#[derive(Debug, Clone)]
struct Vertices([Vertex; MAX_VERTICES]);

impl Default for Vertices {
    #[inline]
    fn default() -> Self {
        Self(std::array::from_fn(|_| Vertex::default()))
    }
}

impl Index<u16> for Vertices {
    type Output = Vertex;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Vertices {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Debug, Clone)]
struct Indices([u16; MAX_VERTICES]);

impl Indices {
    #[inline]
    pub const fn index_of(position: [u8; 2]) -> u16 {
        position[0] as u16 + (position[1] as u16 * MAX_POINTS as u16)
    }
}

impl Default for Indices {
    #[inline]
    fn default() -> Self {
        Self([u16::MAX; MAX_VERTICES])
    }
}

impl Index<u16> for Indices {
    type Output = u16;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Indices {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
