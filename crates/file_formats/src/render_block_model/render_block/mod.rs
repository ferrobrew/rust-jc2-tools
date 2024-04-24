use std::ops::{Deref, DerefMut};

use binrw::{binrw, BinRead, BinWrite};

use super::RenderBlockError;

mod car_paint_simple;
pub use car_paint_simple::*;

mod car_paint;
pub use car_paint::*;

mod deformable_window;
pub use deformable_window::*;

mod general;
pub use general::*;

mod lambert;
pub use lambert::*;

mod skinned_general;
pub use skinned_general::*;

mod window;
pub use window::*;

#[binrw]
#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum RenderBlock {
    // /// HashString::from_str("BillboardFoliage")
    // #[brw(magic(2907872880u32))]
    // BillboardFoliage(BillboardFoliageRenderBlock),

    // /// HashString::from_str("Box")
    // #[brw(magic(1097613365u32))]
    // Box(BoxRenderBlock),

    /// HashString::from_str("CarPaint")
    #[brw(magic(3448970869u32))]
    CarPaint(CarPaintRenderBlock),

    /// HashString::from_str("CarPaintSimple")
    #[brw(magic(2173928592u32))]
    CarPaintSimple(CarPaintSimpleRenderBlock),

    /// HashString::from_str("DeformableWindow")
    #[brw(magic(112326146u32))]
    DeformableWindow(DeformableWindowRenderBlock),

    // /// HashString::from_str("Facade")
    // #[brw(magic(3459897279u32))]
    // Facade(FacadeRenderBlock),

    /// HashString::from_str("General")
    #[brw(magic(2807577387u32))]
    General(GeneralRenderBlock),

    // /// HashString::from_str("Halo")
    // #[brw(magic(1708766642u32))]
    // Halo(HaloRenderBlock),

    /// HashString::from_str("Lambert")
    #[brw(magic(3587672800u32))]
    Lambert(LambertRenderBlock),

    // /// HashString::from_str("Merged")
    // #[brw(magic(2441454787u32))]
    // Merged(MergedRenderBlock),

    // /// HashString::from_str("Occluder")
    // #[brw(magic(709121340u32))]
    // Occluder(OccluderRenderBlock),

    // /// HashString::from_str("Road")
    // #[brw(magic(1183865387u32))]
    // Road(RoadRenderBlock),

    /// HashString::from_str("SkinnedGeneral")
    #[brw(magic(1583709984u32))]
    SkinnedGeneral(SkinnedGeneralRenderBlock),

    // /// HashString::from_str("VegetationBark")
    // #[brw(magic(2985890621u32))]
    // VegetationBark(VegetationBarkRenderBlock),

    // /// HashString::from_str("VegetationFoliage")
    // #[brw(magic(3617096902u32))]
    // VegetationFoliage(VegetationFoliageRenderBlock),

    /// HashString::from_str("Window")
    #[brw(magic(1528824822u32))]
    Window(WindowRenderBlock),
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

            const BLOCK_FOOTER: u32 = 2309737967u32;
            if u32::read_options(reader, endian, ())? != BLOCK_FOOTER {
                return Err(BinError::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new(RenderBlockError::InvalidBlockFooter),
                });
            }
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
