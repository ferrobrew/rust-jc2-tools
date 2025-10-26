use godot::{
    classes::{
        StandardMaterial3D, Texture2D,
        base_material_3d::{Feature, Flags, TextureParam},
        mesh::PrimitiveType,
    },
    prelude::*,
};
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

use crate::resource_loader::{
    JcResourceThread,
    mesh_builder::{MeshBuilder, MeshSurfaceBuilder},
};

use super::{JcResourceError, JcResourceResult};

pub const EXTENSION: &str = "rbm";

// NOTE:
// .lod files are only valid if they contain >= 6 lines
// The first five lines are the rbm paths, some paths may be duplicates (ie/ same model, multiple levels), or be empty (filename is '-', empty model used)
// The next line is the lod factor as an f32, -1.0 means "fov factor only" (which is stupid... why not use 1.0), factor = fov_factor * lod_factor * global_lod_factor
// The default_fov is 46.8deg vertical FOV, and fov_factor = max(tan(default_fov * 0.5) / tan(fov * 0.5), 1.0)
// The lod distances are 0-10, 10-25, 25-50, 50-100, 100-500 meters, these are scaled by the final lod factor

// TODO: we actually need to allow assets like .lod / .rbm to produce *multiple* child assets
// ie/ an RBM needs to load textures, lods need to load multiple RBMs
// it would be very inefficient to ping-pong between this <-> the main thread
// *so* these functions should probably receive &mut ResourceLoader
// *and* the resource loader likely should implement resource caching/pruning
// *and* the resource loader should emit events for *all* loaded resources in a bundle
pub fn load(
    path: GString,
    buffer: PackedByteArray,
    thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Resource>> {
    let mut cursor = binrw::io::Cursor::new(buffer.as_slice());
    match RenderBlockModel::read(&mut cursor) {
        Ok(rbm) => {
            let mut mesh = MeshBuilder::new();
            for block in rbm.blocks.iter() {
                let primitive_type = block.primitive_type();
                let material = block.material(thread)?;
                mesh = mesh.surface(|surface| {
                    block.surface(surface.primitive_type(primitive_type).material(material))
                });
            }
            Ok(mesh.build().upcast::<Resource>())
        }
        Err(error) => Err(JcResourceError::Binrw { path, error }),
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
    // TODO: `load_resource<T: Inherits<Resource>>() -> Gd<T>`
    let (albedo, normal) = (
        thread
            .create_resource(material.textures[0].to_godot())?
            .cast::<Texture2D>(),
        thread
            .create_resource(material.textures[1].to_godot())?
            .cast::<Texture2D>(),
    );

    let mut material = StandardMaterial3D::new_gd();
    material.set_texture(TextureParam::ALBEDO, &albedo);
    material.set_texture(TextureParam::NORMAL, &normal);
    material.set_feature(Feature::NORMAL_MAPPING, true);
    Ok(material)
}
