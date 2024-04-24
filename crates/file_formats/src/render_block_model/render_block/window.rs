use binrw::binrw;
use bitflags::bitflags;

use crate::render_block_model::{GeneralVertex, IndexBuffer, Material, VertexBuffer, VertexFormat};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum WindowVersion {
    #[default]
    V0,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct WindowFlags: u32 {
        const ANIMATE_TEXTURE = 1 << 0;
        const ONE_SIDED = 1 << 1;
        const SIMPLE = 1 << 2;
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct WindowAttributes {
    pub specular_power: f32,
    pub flags: WindowFlags,
}

impl Default for WindowAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            specular_power: 10.0,
            flags: WindowFlags::default(),
        }
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct WindowRenderBlock {
    pub version: WindowVersion,
    pub attributes: WindowAttributes,
    pub material: Material,
    #[brw(args(VertexFormat::F32))]
    pub vertices: VertexBuffer<GeneralVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}
