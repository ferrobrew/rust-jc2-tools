use std::{cell::Cell, thread::LocalKey};

use godot::{
    classes::{
        ArrayMesh, Image, ImageTexture, StandardMaterial3D, Texture2D,
        base_material_3d::{Feature, Flags, TextureParam},
        image::Format,
        mesh::PrimitiveType,
    },
    prelude::*,
};
use godot_utils::mesh_builder::{MeshBuilder, MeshSurfaceBuilder};
use jc2_file_formats::{
    math::{
        Vec2, Vec3, Vec4,
        ops::{VecCross, VecDot},
    },
    render_block_model::{
        BillboardFoliageRenderBlock, CarPaintRenderBlock, CarPaintSimpleRenderBlock,
        DeformableWindowRenderBlock, FacadeRenderBlock, GeneralRenderBlock, HaloRenderBlock,
        LambertRenderBlock, Material, PrimitiveType as JcPrimitiveType, RenderBlock,
        RenderBlockModel, SkinnedGeneralRenderBlock, VegetationBarkRenderBlock,
        VegetationFoliageRenderBlock, WindowRenderBlock,
    },
};

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcModel();

impl JcResourceFormat for JcModel {
    const EXTENSIONS: [&str; 1] = ["rbm"];
    type Result = ArrayMesh;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let mut cursor = binrw::io::Cursor::new(buffer.as_slice());
        match RenderBlockModel::read(&mut cursor) {
            Ok(rbm) => {
                let mut mesh = MeshBuilder::new();
                for block in rbm.blocks.iter() {
                    // Hack for log spam
                    if matches!(
                        block,
                        RenderBlock::BillboardFoliage(_) | RenderBlock::Halo(_)
                    ) {
                        continue;
                    }

                    let primitive_type = block.primitive_type();
                    let material = block.material(thread)?;
                    mesh = mesh.surface(|surface| {
                        block.surface(surface.primitive_type(primitive_type).material(material))
                    });
                }
                Ok(mesh.build())
            }
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

trait SurfacePrimitiveType {
    fn primitive_type(&self) -> PrimitiveType;
}

impl SurfacePrimitiveType for RenderBlock {
    fn primitive_type(&self) -> PrimitiveType {
        let primitive_type = match self {
            RenderBlock::BillboardFoliage(block) => block.material.primitive_type,
            RenderBlock::CarPaint(block) => block.material.primitive_type,
            RenderBlock::CarPaintSimple(block) => block.material.primitive_type,
            RenderBlock::DeformableWindow(block) => block.material.primitive_type,
            RenderBlock::Facade(block) => block.material.primitive_type,
            RenderBlock::General(block) => block.material.primitive_type,
            RenderBlock::Halo(block) => block.material.primitive_type,
            RenderBlock::Lambert(block) => block.material.primitive_type,
            RenderBlock::SkinnedGeneral(block) => block.material.primitive_type,
            RenderBlock::VegetationBark(block) => block.material.primitive_type,
            RenderBlock::VegetationFoliage(block) => block.material.primitive_type,
            RenderBlock::Window(block) => block.material.primitive_type,
        };

        match primitive_type {
            JcPrimitiveType::TriangleList | JcPrimitiveType::IndexedTriangleList => {
                PrimitiveType::TRIANGLES
            }
            JcPrimitiveType::TriangleStrip | JcPrimitiveType::IndexedTriangleStrip => {
                PrimitiveType::TRIANGLE_STRIP
            }
            JcPrimitiveType::TriangleFan | JcPrimitiveType::IndexedTriangleFan => {
                todo!("Triangle fan to triangle list conversion not implemented")
            }
            JcPrimitiveType::LineList => PrimitiveType::LINES,
            JcPrimitiveType::PointSprite | JcPrimitiveType::IndexedPointSprite => {
                PrimitiveType::POINTS
            }
        }
    }
}

trait SurfaceBuilder {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>>;
    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder;
}

impl SurfaceBuilder for RenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        match self {
            RenderBlock::BillboardFoliage(block) => block.material(thread),
            RenderBlock::CarPaint(block) => block.material(thread),
            RenderBlock::CarPaintSimple(block) => block.material(thread),
            RenderBlock::DeformableWindow(block) => block.material(thread),
            RenderBlock::Facade(block) => block.material(thread),
            RenderBlock::General(block) => block.material(thread),
            RenderBlock::Halo(block) => block.material(thread),
            RenderBlock::Lambert(block) => block.material(thread),
            RenderBlock::SkinnedGeneral(block) => block.material(thread),
            RenderBlock::VegetationBark(block) => block.material(thread),
            RenderBlock::VegetationFoliage(block) => block.material(thread),
            RenderBlock::Window(block) => block.material(thread),
        }
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        match self {
            RenderBlock::BillboardFoliage(block) => block.surface(surface),
            RenderBlock::CarPaint(block) => block.surface(surface),
            RenderBlock::CarPaintSimple(block) => block.surface(surface),
            RenderBlock::DeformableWindow(block) => block.surface(surface),
            RenderBlock::Facade(block) => block.surface(surface),
            RenderBlock::General(block) => block.surface(surface),
            RenderBlock::Halo(block) => block.surface(surface),
            RenderBlock::Lambert(block) => block.surface(surface),
            RenderBlock::SkinnedGeneral(block) => block.surface(surface),
            RenderBlock::VegetationBark(block) => block.surface(surface),
            RenderBlock::VegetationFoliage(block) => block.surface(surface),
            RenderBlock::Window(block) => block.surface(surface),
        }
    }
}

impl SurfaceBuilder for BillboardFoliageRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.dimensions);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .uv1(uv1)
            .uv2(uv2) // TODO: use custom data?
            .indices(indices)
    }
}

impl SurfaceBuilder for CarPaintRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent).into_iter())
            .collect();
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);
        // TODO: light / emissive?

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .uv1(uv1)
            .indices(indices)

        // TODO: cannot be used in tandem with render blocks that don't also implement blend shapes... use custom data?
        // .blend_shape(|blend_shape| {
        //     let vertices: PackedVector3Array =
        //         self.vertices.iter().collect_godot(|v| v.morph_position);
        //     let normals: PackedVector3Array =
        //         self.vertices.iter().collect_godot(|v| v.morph_normal);
        //     let tangents: PackedFloat32Array = self
        //         .vertices
        //         .iter()
        //         .flat_map(|v| <[f32; 4]>::from(v.tangent))
        //         .collect();
        //     blend_shape
        //         .vertices(vertices)
        //         .normals(normals)
        //         .tangents(tangents)
        // })
    }
}

impl SurfaceBuilder for CarPaintSimpleRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| {
                let [x, y, z] = v.tangent.into();
                let w = v.normal.cross(v.tangent).dot(v.binormal).signum();
                [x, y, z, w].into_iter()
            })
            .collect();
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .uv1(uv1)
            .indices(indices)
    }
}

impl SurfaceBuilder for DeformableWindowRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent))
            .collect();
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .uv1(uv1)
            .indices(indices)

        // TODO: cannot be used in tandem with render blocks that don't also implement blend shapes... use custom data?
        // .blend_shape(|blend_shape| {
        //     let vertices: PackedVector3Array =
        //         self.vertices.iter().collect_godot(|v| v.morph_position);
        //     let normals: PackedVector3Array =
        //         self.vertices.iter().collect_godot(|v| v.morph_normal);
        //     let tangents: PackedFloat32Array = self
        //         .vertices
        //         .iter()
        //         .flat_map(|v| <[f32; 4]>::from(v.tangent))
        //         .collect();
        //     blend_shape
        //         .vertices(vertices)
        //         .normals(normals)
        //         .tangents(tangents)
        // })
    }
}

impl SurfaceBuilder for FacadeRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self
            .vertices
            .iter()
            .collect_godot(|v| v.position * self.attributes.scale);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent))
            .collect();
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

impl SurfaceBuilder for GeneralRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        let mut material = create_material(thread, &self.material)?;

        material.set_uv1_scale(self.attributes.vertex_info.uv0_extent.into_godot());
        material.set_uv2_scale(self.attributes.vertex_info.uv1_extent.into_godot());
        material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);
        material.set_flag(Flags::SRGB_VERTEX_COLOR, true);

        Ok(material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self
            .vertices
            .iter()
            .collect_godot(|v| v.position * self.attributes.vertex_info.scale);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent))
            .collect();
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

impl SurfaceBuilder for HaloRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.dimensions);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2) // TODO: use custom data?
            .indices(indices)
    }
}

impl SurfaceBuilder for LambertRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        let mut material = create_material(thread, &self.material)?;

        material.set_uv1_scale(self.attributes.vertex_info.uv0_extent.into_godot());
        material.set_uv2_scale(self.attributes.vertex_info.uv1_extent.into_godot());
        material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);
        material.set_flag(Flags::SRGB_VERTEX_COLOR, true);

        Ok(material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self
            .vertices
            .iter()
            .collect_godot(|v| v.position * self.attributes.vertex_info.scale);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent))
            .collect();
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

impl SurfaceBuilder for SkinnedGeneralRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| {
                let [x, y, z] = v.tangent.into();
                let w = v.normal.cross(v.tangent).dot(v.binormal).signum();
                [x, y, z, w]
            })
            .collect();
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .uv1(uv1)
            .indices(indices)
    }
}

impl SurfaceBuilder for VegetationBarkRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| {
                let [x, y, z] = v.tangent.into();
                let w = v.normal.cross(v.tangent).dot(v.binormal).signum();
                [x, y, z, w]
            })
            .collect();
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

impl SurfaceBuilder for VegetationFoliageRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| {
                let [x, y, z] = v.tangent.into();
                let w = v.normal.cross(v.tangent).dot(v.binormal).signum();
                [x, y, z, w]
            })
            .collect();
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

impl SurfaceBuilder for WindowRenderBlock {
    fn material(&self, thread: &mut JcResourceThread) -> JcResourceResult<Gd<StandardMaterial3D>> {
        create_material(thread, &self.material)
    }

    fn surface(&self, surface: MeshSurfaceBuilder) -> MeshSurfaceBuilder {
        let vertices: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.position);
        let uv1: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv0);
        let uv2: PackedVector2Array = self.vertices.iter().collect_godot(|v| v.uv1);
        let normals: PackedVector3Array = self.vertices.iter().collect_godot(|v| v.normal);
        let tangents: PackedFloat32Array = self
            .vertices
            .iter()
            .flat_map(|v| <[f32; 4]>::from(v.tangent))
            .collect();
        let colors: PackedColorArray = self.vertices.iter().collect_godot(|v| v.color);
        let indices: PackedInt32Array = self.indices.iter().rev().collect_godot(|&i| i);

        surface
            .vertices(vertices)
            .normals(normals)
            .tangents(tangents)
            .colors(colors)
            .uv1(uv1)
            .uv2(uv2)
            .indices(indices)
    }
}

trait CollectGodot: IntoIterator + Sized {
    #[inline]
    fn collect_godot<R, F, I, G>(self, f: F) -> R
    where
        F: Fn(Self::Item) -> I,
        I: IntoGodot<G>,
        R: FromIterator<G>,
    {
        self.into_iter().map(|v| f(v).into_godot()).collect()
    }
}

impl<T> CollectGodot for T where T: IntoIterator {}

trait IntoGodot<T> {
    fn into_godot(self) -> T;
}

impl IntoGodot<i32> for u16 {
    #[inline]
    fn into_godot(self) -> i32 {
        self as i32
    }
}

impl IntoGodot<Vector2> for Vec2<f32> {
    #[inline]
    fn into_godot(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl IntoGodot<Vector3> for Vec2<f32> {
    #[inline]
    fn into_godot(self) -> Vector3 {
        Vector3::new(self.x, self.y, 0f32)
    }
}

impl IntoGodot<Color> for Vec3<f32> {
    #[inline]
    fn into_godot(self) -> Color {
        Color::from_rgb(self.x, self.y, self.z)
    }
}

impl IntoGodot<Vector3> for Vec3<f32> {
    #[inline]
    fn into_godot(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl IntoGodot<Vector4> for Vec3<f32> {
    #[inline]
    fn into_godot(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 0f32)
    }
}

impl IntoGodot<Color> for Vec4<f32> {
    #[inline]
    fn into_godot(self) -> Color {
        Color::from_rgba(self.x, self.y, self.z, self.w)
    }
}

impl IntoGodot<Vector4> for Vec4<f32> {
    #[inline]
    fn into_godot(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, self.w)
    }
}

fn create_material(
    thread: &mut JcResourceThread,
    material: &Material,
) -> JcResourceResult<Gd<StandardMaterial3D>> {
    thread_local! {
        static ALBEDO: Cell<Option<Gd<Texture2D>>> = Cell::new(fallback(Color::from_rgb(1.0, 0.0, 1.0)));
        static NORMAL: Cell<Option<Gd<Texture2D>>> = Cell::new(fallback(Color::from_rgb(1.0, 0.0, 1.0)));
    }

    let fetch = |texture: &'static LocalKey<Cell<Option<Gd<Texture2D>>>>| {
        if let Some(value) = texture.take() {
            texture.set(Some(value.clone()));
            Ok(value)
        } else {
            unreachable!("failed to create fallback texture")
        }
    };

    let (albedo, normal) = (
        texture(&material.textures[0], || fetch(&ALBEDO), thread)?,
        texture(&material.textures[1], || fetch(&NORMAL), thread)?,
    );

    let mut material = StandardMaterial3D::new_gd();
    material.set_texture(TextureParam::ALBEDO, &albedo);
    material.set_texture(TextureParam::NORMAL, &normal);
    material.set_feature(Feature::NORMAL_MAPPING, true);
    Ok(material)
}

fn texture<F: FnOnce() -> JcResourceResult<Gd<Texture2D>>>(
    path: &str,
    fallback: F,
    thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Texture2D>> {
    thread.create_resource(path.to_godot()).map_or_else(
        |_| {
            godot_warn!("Failed to texture '{path}', using fallback!");
            fallback()
        },
        |t| Ok(t.cast()),
    )
}

fn fallback(color: Color) -> Option<Gd<Texture2D>> {
    let mut image = Image::new_gd();
    image.set_data(
        1,
        1,
        false,
        Format::RGBA8,
        &PackedByteArray::from([color.r8(), color.g8(), color.b8(), color.a8()]),
    );
    ImageTexture::create_from_image(&image).map(|x| x.upcast())
}
