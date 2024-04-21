use std::ops::{Deref, DerefMut};

use binrw::{binrw, BinRead, BinWrite};

use super::RenderBlockError;

mod car_paint_simple;
pub use car_paint_simple::*;

mod general;
pub use general::*;

mod lambert;
pub use lambert::*;

mod skinned_general;
pub use skinned_general::*;

#[binrw]
#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum RenderBlock {
    // /// HashString::from_str("2DTex1")
    // #[brw(magic(1172031583u32))]
    // 2DTex1(2DTex1RenderBlock),

    // /// HashString::from_str("2DTex2")
    // #[brw(magic(140915813u32))]
    // 2DTex2(2DTex2RenderBlock),

    // /// HashString::from_str("3DText")
    // #[brw(magic(1196347192u32))]
    // 3DText(3DTextRenderBlock),

    // /// HashString::from_str("AOBox")
    // #[brw(magic(1740435920u32))]
    // AOBox(AOBoxRenderBlock),

    // /// HashString::from_str("Beam")
    // #[brw(magic(459178773u32))]
    // Beam(BeamRenderBlock),

    // /// HashString::from_str("BillboardFoliage")
    // #[brw(magic(2907872880u32))]
    // BillboardFoliage(BillboardFoliageRenderBlock),

    // /// HashString::from_str("Box")
    // #[brw(magic(1097613365u32))]
    // Box(BoxRenderBlock),

    // /// HashString::from_str("Bullet")
    // #[brw(magic(2438405360u32))]
    // Bullet(BulletRenderBlock),

    // /// HashString::from_str("CarPaint")
    // #[brw(magic(3448970869u32))]
    // CarPaint(CarPaintRenderBlock),

    /// HashString::from_str("CarPaintSimple")
    #[brw(magic(2173928592u32))]
    CarPaintSimple(CarPaintSimpleRenderBlock),

    // /// HashString::from_str("CirrusClouds")
    // #[brw(magic(877238411u32))]
    // CirrusClouds(CirrusCloudsRenderBlock),

    // /// HashString::from_str("Clouds")
    // #[brw(magic(2744717886u32))]
    // Clouds(CloudsRenderBlock),

    // /// HashString::from_str("Creatures")
    // #[brw(magic(3238486682u32))]
    // Creatures(CreaturesRenderBlock),

    // /// HashString::from_str("DecalDeformable")
    // #[brw(magic(3751713046u32))]
    // DecalDeformable(DecalDeformableRenderBlock),

    // /// HashString::from_str("DecalSimple")
    // #[brw(magic(732945772u32))]
    // DecalSimple(DecalSimpleRenderBlock),

    // /// HashString::from_str("DecalSkinned")
    // #[brw(magic(2319630529u32))]
    // DecalSkinned(DecalSkinnedRenderBlock),

    // /// HashString::from_str("DeformableWindow")
    // #[brw(magic(112326146u32))]
    // DeformableWindow(DeformableWindowRenderBlock),

    // /// HashString::from_str("Facade")
    // #[brw(magic(3459897279u32))]
    // Facade(FacadeRenderBlock),

    // /// HashString::from_str("Flag")
    // #[brw(magic(3632659675u32))]
    // Flag(FlagRenderBlock),

    // /// HashString::from_str("FogGradient")
    // #[brw(magic(372767530u32))]
    // FogGradient(FogGradientRenderBlock),

    // /// HashString::from_str("Font")
    // #[brw(magic(2123339982u32))]
    // Font(FontRenderBlock),

    /// HashString::from_str("General")
    #[brw(magic(2807577387u32))]
    General(GeneralRenderBlock),

    // /// HashString::from_str("Grass")
    // #[brw(magic(3497104304u32))]
    // Grass(GrassRenderBlock),

    // /// HashString::from_str("GuiAnark")
    // #[brw(magic(4184314686u32))]
    // GuiAnark(GuiAnarkRenderBlock),

    // /// HashString::from_str("Halo")
    // #[brw(magic(1708766642u32))]
    // Halo(HaloRenderBlock),

    /// HashString::from_str("Lambert")
    #[brw(magic(3587672800u32))]
    Lambert(LambertRenderBlock),

    // /// HashString::from_str("Leaves")
    // #[brw(magic(1241505016u32))]
    // Leaves(LeavesRenderBlock),

    // /// HashString::from_str("Lights")
    // #[brw(magic(3678991706u32))]
    // Lights(LightsRenderBlock),

    // /// HashString::from_str("Line")
    // #[brw(magic(3229313126u32))]
    // Line(LineRenderBlock),

    // /// HashString::from_str("Merged")
    // #[brw(magic(2441454787u32))]
    // Merged(MergedRenderBlock),

    // /// HashString::from_str("NvWaterHighEnd")
    // #[brw(magic(3868154914u32))]
    // NvWaterHighEnd(NvWaterHighEndRenderBlock),

    // /// HashString::from_str("Occluder")
    // #[brw(magic(709121340u32))]
    // Occluder(OccluderRenderBlock),

    // /// HashString::from_str("Open")
    // #[brw(magic(1528015433u32))]
    // Open(OpenRenderBlock),

    // /// HashString::from_str("Particle")
    // #[brw(magic(2650374750u32))]
    // Particle(ParticleRenderBlock),

    // /// HashString::from_str("Skidmarks")
    // #[brw(magic(1295678767u32))]
    // Skidmarks(SkidmarksRenderBlock),

    /// HashString::from_str("SkinnedGeneral")
    #[brw(magic(1583709984u32))]
    SkinnedGeneral(SkinnedGeneralRenderBlock),

    // /// HashString::from_str("SkyGradient")
    // #[brw(magic(1678205320u32))]
    // SkyGradient(SkyGradientRenderBlock),

    // /// HashString::from_str("SoftClouds")
    // #[brw(magic(3003704052u32))]
    // SoftClouds(SoftCloudsRenderBlock),

    // /// HashString::from_str("SplineRoad")
    // #[brw(magic(15575981u32))]
    // SplineRoad(SplineRoadRenderBlock),

    // /// HashString::from_str("Stars")
    // #[brw(magic(421981765u32))]
    // Stars(StarsRenderBlock),

    // /// HashString::from_str("Terrain")
    // #[brw(magic(1959219946u32))]
    // Terrain(TerrainRenderBlock),

    // /// HashString::from_str("TerrainForest")
    // #[brw(magic(1596301290u32))]
    // TerrainForest(TerrainForestRenderBlock),

    // /// HashString::from_str("TerrainForestFin")
    // #[brw(magic(4053769774u32))]
    // TerrainForestFin(TerrainForestFinRenderBlock),

    // /// HashString::from_str("TreeImpostorTrunk")
    // #[brw(magic(4067247576u32))]
    // TreeImpostorTrunk(TreeImpostorTrunkRenderBlock),

    // /// HashString::from_str("TreeImpostorTop")
    // #[brw(magic(2071355251u32))]
    // TreeImpostorTop(TreeImpostorTopRenderBlock),

    // /// HashString::from_str("Triangle")
    // #[brw(magic(3952535426u32))]
    // Triangle(TriangleRenderBlock),

    // /// HashString::from_str("VegetationBark")
    // #[brw(magic(2985890621u32))]
    // VegetationBark(VegetationBarkRenderBlock),

    // /// HashString::from_str("VegetationFoliage")
    // #[brw(magic(3617096902u32))]
    // VegetationFoliage(VegetationFoliageRenderBlock),

    // /// HashString::from_str("WaterGodrays")
    // #[brw(magic(2526366255u32))]
    // WaterGodrays(WaterGodraysRenderBlock),

    // /// HashString::from_str("WaterHighEnd")
    // #[brw(magic(1435073460u32))]
    // WaterHighEnd(WaterHighEndRenderBlock),

    // /// HashString::from_str("WaterWaves")
    // #[brw(magic(1031243393u32))]
    // WaterWaves(WaterWavesRenderBlock),

    // /// HashString::from_str("Weather")
    // #[brw(magic(2300665221u32))]
    // Weather(WeatherRenderBlock),

    // /// HashString::from_str("Window")
    // #[brw(magic(1528824822u32))]
    // Window(WindowRenderBlock),
}

#[derive(Clone, Debug, Default)]
pub struct RenderBlocks(Vec<RenderBlock>);

impl Deref for RenderBlocks {
    type Target = Vec<RenderBlock>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderBlocks {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type BinError = binrw::Error;

impl BinRead for RenderBlocks {
    type Args<'a> = ();

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let length = u32::read_options(reader, endian, ())?;
        let mut blocks = Vec::with_capacity(length as usize);
        for _ in 0..length {
            blocks.push(RenderBlock::read_options(reader, endian, ())?);
        }
        Ok(Self(blocks))
    }
}

impl BinWrite for RenderBlocks {
    type Args<'a> = (usize,);

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        if let Ok(length) = u32::try_from(self.len()) {
            length.write_options(writer, endian, ())?;
            for index in self.iter() {
                index.write_options(writer, endian, ())?;
            }
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(RenderBlockError::InvalidArrayLength),
            })
        }
    }
}
