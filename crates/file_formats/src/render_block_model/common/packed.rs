use binrw::binrw;

use crate::math::{Vec2, Vec3, Vec4};

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedUV(i16, i16);

impl From<Vec2<f32>> for PackedUV {
    fn from(value: Vec2<f32>) -> Self {
        Self(
            (value.x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.y.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
        )
    }
}

impl From<PackedUV> for Vec2<f32> {
    fn from(value: PackedUV) -> Self {
        Self {
            x: value.0 as f32 / i16::MAX as f32,
            y: value.1 as f32 / i16::MAX as f32,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedPosition(i16, i16, i16);

impl From<Vec3<f32>> for PackedPosition {
    fn from(value: Vec3<f32>) -> Self {
        Self(
            (value.x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.y.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.z.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
        )
    }
}

impl From<PackedPosition> for Vec3<f32> {
    fn from(value: PackedPosition) -> Self {
        Self {
            x: value.0 as f32 / i16::MAX as f32,
            y: value.1 as f32 / i16::MAX as f32,
            z: value.2 as f32 / i16::MAX as f32,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedNormalF32(f32);

impl From<Vec3<f32>> for PackedNormalF32 {
    fn from(value: Vec3<f32>) -> Self {
        Self({
            let x = (value.x * 0.5 + 1.0) * 1.0;
            let y = (value.y * 0.5 + 1.0) * 256.0;
            let z = (value.z * 0.5 + 1.0) * 65536.0;
            x + y + z
        })
    }
}

impl From<PackedNormalF32> for Vec3<f32> {
    fn from(value: PackedNormalF32) -> Self {
        Self {
            x: (value.0 / 1.0).fract() * 2.0 - 1.0,
            y: (value.0 / 256.0).fract() * 2.0 - 1.0,
            z: (value.0 / 65536.0).fract() * 2.0 - 1.0,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedNormalU32(u32);

impl From<Vec3<f32>> for PackedNormalU32 {
    fn from(value: Vec3<f32>) -> Self {
        Self({
            let x = ((((value.x * 127.0) + 128.0) as u32) & 0xFF) << 24;
            let y = ((((value.y * 127.0) + 128.0) as u32) & 0xFF) << 16;
            let z = ((((value.z * 127.0) + 128.0) as u32) & 0xFF) << 8;
            x + y + z + 128u32
        })
    }
}

impl From<PackedNormalU32> for Vec3<f32> {
    fn from(value: PackedNormalU32) -> Self {
        Self {
            x: ((((value.0 >> 24) & 0xFF) as f32) - 128.0) / 127.0,
            y: ((((value.0 >> 16) & 0xFF) as f32) - 128.0) / 127.0,
            z: ((((value.0 >> 8) & 0xFF) as f32) - 128.0) / 127.0,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedRGB(f32);

impl From<Vec3<f32>> for PackedRGB {
    fn from(value: Vec3<f32>) -> Self {
        Self({
            let x = value.x * 1.0;
            let y = value.y * 64.0;
            let z = value.z * 4096.0;
            x + y + z
        })
    }
}

impl From<PackedRGB> for Vec3<f32> {
    fn from(value: PackedRGB) -> Self {
        Self {
            x: (value.0 / 1.0).fract(),
            y: (value.0 / 64.0).fract(),
            z: (value.0 / 4096.0).fract(),
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedRGBA(f32);

impl From<Vec4<f32>> for PackedRGBA {
    fn from(value: Vec4<f32>) -> Self {
        Self({
            let x = value.x * 1.0;
            let y = value.y * 64.0;
            let z = value.z * 4096.0;
            let w = value.z * 262144.0;
            x + y + z + w
        })
    }
}

impl From<PackedRGBA> for Vec4<f32> {
    fn from(value: PackedRGBA) -> Self {
        Self {
            x: (value.0 / 1.0).fract(),
            y: (value.0 / 64.0).fract(),
            z: (value.0 / 4096.0).fract(),
            w: (value.0 / 262144.0).fract(),
        }
    }
}
