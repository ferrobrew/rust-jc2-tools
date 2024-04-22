use std::ops::{Deref, DerefMut};

use binrw::{binrw, BinRead, BinWrite};

use super::RenderBlockError;

mod car_paint_simple;
pub use car_paint_simple::*;

mod car_paint;
pub use car_paint::*;

mod general;
pub use general::*;

mod lambert;
pub use lambert::*;

mod skinned_general;
pub use skinned_general::*;

#[binrw]
#[derive(Clone, Debug)]
pub struct RenderBlockWrapper<T>
where
    T: Clone + for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()>,
{
    data: T,
    #[brw(magic(2309737967u32))]
    footer: (),
}

impl<T> Deref for RenderBlockWrapper<T>
where
    T: Clone + for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for RenderBlockWrapper<T>
where
    T: Clone + for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[binrw]
#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum RenderBlock {
    // /// HashString::from_str("2DTex1")
    // #[brw(magic(1172031583u32))]
    // 2DTex1(RenderBlockWrapper::<2DTex1RenderBlock>),

    // /// HashString::from_str("2DTex2")
    // #[brw(magic(140915813u32))]
    // 2DTex2(RenderBlockWrapper::<2DTex2RenderBlock>),

    // /// HashString::from_str("3DText")
    // #[brw(magic(1196347192u32))]
    // 3DText(RenderBlockWrapper::<3DTextRenderBlock>),

    // /// HashString::from_str("AOBox")
    // #[brw(magic(1740435920u32))]
    // AOBox(RenderBlockWrapper::<AOBoxRenderBlock>),

    // /// HashString::from_str("Beam")
    // #[brw(magic(459178773u32))]
    // Beam(RenderBlockWrapper::<BeamRenderBlock>),

    // /// HashString::from_str("BillboardFoliage")
    // #[brw(magic(2907872880u32))]
    // BillboardFoliage(RenderBlockWrapper::<BillboardFoliageRenderBlock>),

    // /// HashString::from_str("Box")
    // #[brw(magic(1097613365u32))]
    // Box(RenderBlockWrapper::<BoxRenderBlock>),

    // /// HashString::from_str("Bullet")
    // #[brw(magic(2438405360u32))]
    // Bullet(RenderBlockWrapper::<BulletRenderBlock>),

    /// HashString::from_str("CarPaint")
    #[brw(magic(3448970869u32))]
    CarPaint(RenderBlockWrapper::<CarPaintRenderBlock>),

    /// HashString::from_str("CarPaintSimple")
    #[brw(magic(2173928592u32))]
    CarPaintSimple(RenderBlockWrapper::<CarPaintSimpleRenderBlock>),

    // /// HashString::from_str("CirrusClouds")
    // #[brw(magic(877238411u32))]
    // CirrusClouds(RenderBlockWrapper::<CirrusCloudsRenderBlock>),

    // /// HashString::from_str("Clouds")
    // #[brw(magic(2744717886u32))]
    // Clouds(RenderBlockWrapper::<CloudsRenderBlock>),

    // /// HashString::from_str("Creatures")
    // #[brw(magic(3238486682u32))]
    // Creatures(RenderBlockWrapper::<CreaturesRenderBlock>),

    // /// HashString::from_str("DecalDeformable")
    // #[brw(magic(3751713046u32))]
    // DecalDeformable(RenderBlockWrapper::<DecalDeformableRenderBlock>),

    // /// HashString::from_str("DecalSimple")
    // #[brw(magic(732945772u32))]
    // DecalSimple(RenderBlockWrapper::<DecalSimpleRenderBlock>),

    // /// HashString::from_str("DecalSkinned")
    // #[brw(magic(2319630529u32))]
    // DecalSkinned(RenderBlockWrapper::<DecalSkinnedRenderBlock>),

    // /// HashString::from_str("DeformableWindow")
    // #[brw(magic(112326146u32))]
    // DeformableWindow(RenderBlockWrapper::<DeformableWindowRenderBlock>),

    // /// HashString::from_str("Facade")
    // #[brw(magic(3459897279u32))]
    // Facade(RenderBlockWrapper::<FacadeRenderBlock>),

    // /// HashString::from_str("Flag")
    // #[brw(magic(3632659675u32))]
    // Flag(RenderBlockWrapper::<FlagRenderBlock>),

    // /// HashString::from_str("FogGradient")
    // #[brw(magic(372767530u32))]
    // FogGradient(RenderBlockWrapper::<FogGradientRenderBlock>),

    // /// HashString::from_str("Font")
    // #[brw(magic(2123339982u32))]
    // Font(RenderBlockWrapper::<FontRenderBlock>),

    /// HashString::from_str("General")
    #[brw(magic(2807577387u32))]
    General(RenderBlockWrapper::<GeneralRenderBlock>),

    // /// HashString::from_str("Grass")
    // #[brw(magic(3497104304u32))]
    // Grass(RenderBlockWrapper::<GrassRenderBlock>),

    // /// HashString::from_str("GuiAnark")
    // #[brw(magic(4184314686u32))]
    // GuiAnark(RenderBlockWrapper::<GuiAnarkRenderBlock>),

    // /// HashString::from_str("Halo")
    // #[brw(magic(1708766642u32))]
    // Halo(RenderBlockWrapper::<HaloRenderBlock>),

    /// HashString::from_str("Lambert")
    #[brw(magic(3587672800u32))]
    Lambert(RenderBlockWrapper::<LambertRenderBlock>),

    // /// HashString::from_str("Leaves")
    // #[brw(magic(1241505016u32))]
    // Leaves(RenderBlockWrapper::<LeavesRenderBlock>),

    // /// HashString::from_str("Lights")
    // #[brw(magic(3678991706u32))]
    // Lights(RenderBlockWrapper::<LightsRenderBlock>),

    // /// HashString::from_str("Line")
    // #[brw(magic(3229313126u32))]
    // Line(RenderBlockWrapper::<LineRenderBlock>),

    // /// HashString::from_str("Merged")
    // #[brw(magic(2441454787u32))]
    // Merged(RenderBlockWrapper::<MergedRenderBlock>),

    // /// HashString::from_str("NvWaterHighEnd")
    // #[brw(magic(3868154914u32))]
    // NvWaterHighEnd(RenderBlockWrapper::<NvWaterHighEndRenderBlock>),

    // /// HashString::from_str("Occluder")
    // #[brw(magic(709121340u32))]
    // Occluder(RenderBlockWrapper::<OccluderRenderBlock>),

    // /// HashString::from_str("Open")
    // #[brw(magic(1528015433u32))]
    // Open(RenderBlockWrapper::<OpenRenderBlock>),

    // /// HashString::from_str("Particle")
    // #[brw(magic(2650374750u32))]
    // Particle(RenderBlockWrapper::<ParticleRenderBlock>),

    // /// HashString::from_str("Skidmarks")
    // #[brw(magic(1295678767u32))]
    // Skidmarks(RenderBlockWrapper::<SkidmarksRenderBlock>),

    /// HashString::from_str("SkinnedGeneral")
    #[brw(magic(1583709984u32))]
    SkinnedGeneral(RenderBlockWrapper::<SkinnedGeneralRenderBlock>),

    // /// HashString::from_str("SkyGradient")
    // #[brw(magic(1678205320u32))]
    // SkyGradient(RenderBlockWrapper::<SkyGradientRenderBlock>),

    // /// HashString::from_str("SoftClouds")
    // #[brw(magic(3003704052u32))]
    // SoftClouds(RenderBlockWrapper::<SoftCloudsRenderBlock>),

    // /// HashString::from_str("SplineRoad")
    // #[brw(magic(15575981u32))]
    // SplineRoad(RenderBlockWrapper::<SplineRoadRenderBlock>),

    // /// HashString::from_str("Stars")
    // #[brw(magic(421981765u32))]
    // Stars(RenderBlockWrapper::<StarsRenderBlock>),

    // /// HashString::from_str("Terrain")
    // #[brw(magic(1959219946u32))]
    // Terrain(RenderBlockWrapper::<TerrainRenderBlock>),

    // /// HashString::from_str("TerrainForest")
    // #[brw(magic(1596301290u32))]
    // TerrainForest(RenderBlockWrapper::<TerrainForestRenderBlock>),

    // /// HashString::from_str("TerrainForestFin")
    // #[brw(magic(4053769774u32))]
    // TerrainForestFin(RenderBlockWrapper::<TerrainForestFinRenderBlock>),

    // /// HashString::from_str("TreeImpostorTrunk")
    // #[brw(magic(4067247576u32))]
    // TreeImpostorTrunk(RenderBlockWrapper::<TreeImpostorTrunkRenderBlock>),

    // /// HashString::from_str("TreeImpostorTop")
    // #[brw(magic(2071355251u32))]
    // TreeImpostorTop(RenderBlockWrapper::<TreeImpostorTopRenderBlock>),

    // /// HashString::from_str("Triangle")
    // #[brw(magic(3952535426u32))]
    // Triangle(RenderBlockWrapper::<TriangleRenderBlock>),

    // /// HashString::from_str("VegetationBark")
    // #[brw(magic(2985890621u32))]
    // VegetationBark(RenderBlockWrapper::<VegetationBarkRenderBlock>),

    // /// HashString::from_str("VegetationFoliage")
    // #[brw(magic(3617096902u32))]
    // VegetationFoliage(RenderBlockWrapper::<VegetationFoliageRenderBlock>),

    // /// HashString::from_str("WaterGodrays")
    // #[brw(magic(2526366255u32))]
    // WaterGodrays(RenderBlockWrapper::<WaterGodraysRenderBlock>),

    // /// HashString::from_str("WaterHighEnd")
    // #[brw(magic(1435073460u32))]
    // WaterHighEnd(RenderBlockWrapper::<WaterHighEndRenderBlock>),

    // /// HashString::from_str("WaterWaves")
    // #[brw(magic(1031243393u32))]
    // WaterWaves(RenderBlockWrapper::<WaterWavesRenderBlock>),

    // /// HashString::from_str("Weather")
    // #[brw(magic(2300665221u32))]
    // Weather(RenderBlockWrapper::<WeatherRenderBlock>),

    // /// HashString::from_str("Window")
    // #[brw(magic(1528824822u32))]
    // Window(RenderBlockWrapper::<WindowRenderBlock>),
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
