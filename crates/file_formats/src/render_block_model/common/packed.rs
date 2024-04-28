use binrw::binrw;

use crate::math::{Vec2, Vec3, Vec4};

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedWeightAndIndex(i16);

impl PackedWeightAndIndex {
    #[inline]
    pub fn new(weight: f32, index: u32) -> Self {
        Self(((weight * 255.0) as i16 & 0xFF) | (((index as i32 - 128) & 0xFF) << 8) as i16)
    }

    #[inline]
    pub fn weight(&self) -> f32 {
        (self.0 & 0xFF) as f32 / 255.0
    }

    #[inline]
    pub fn index(&self) -> u32 {
        (((self.0 >> 8) & 0xFF) + 128) as u32
    }
}
#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedUVF32(f32);

impl From<Vec2<f32>> for PackedUVF32 {
    #[inline]
    fn from(value: Vec2<f32>) -> Self {
        let x = ((value.x + 0.5).floor().abs() / 64.0).fract();
        let y = (((value.y + 0.5).floor().abs() / 64.0) * 2048.0).fract();
        Self(x + y)
    }
}

impl From<PackedUVF32> for Vec2<f32> {
    #[inline]
    fn from(value: PackedUVF32) -> Self {
        Self {
            x: value.0.fract(),
            y: value.0.floor() / 2048.0,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedUVI16(i16, i16);

impl From<Vec2<f32>> for PackedUVI16 {
    #[inline]
    fn from(value: Vec2<f32>) -> Self {
        Self(
            (value.x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.y.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
        )
    }
}

impl From<PackedUVI16> for Vec2<f32> {
    #[inline]
    fn from(value: PackedUVI16) -> Self {
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
    #[inline]
    fn from(value: Vec3<f32>) -> Self {
        Self(
            (value.x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.y.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
            (value.z.clamp(-1.0, 1.0) * i16::MAX as f32) as i16,
        )
    }
}

impl From<PackedPosition> for Vec3<f32> {
    #[inline]
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
    #[inline]
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
    #[inline]
    fn from(value: PackedNormalF32) -> Self {
        let n = value.0.abs();
        Self {
            x: (n / 1.0).fract() * 2.0 - 1.0,
            y: (n / 256.0).fract() * 2.0 - 1.0,
            z: (n / 65536.0).fract() * 2.0 - 1.0,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedTangentF32(f32);

impl From<Vec4<f32>> for PackedTangentF32 {
    #[inline]
    fn from(value: Vec4<f32>) -> Self {
        Self({
            let x = (value.x * 0.5 + 1.0) * 1.0;
            let y = (value.y * 0.5 + 1.0) * 256.0;
            let z = (value.z * 0.5 + 1.0) * 65536.0;
            (x + y + z) * value.w.signum()
        })
    }
}

impl From<PackedTangentF32> for Vec4<f32> {
    #[inline]
    fn from(value: PackedTangentF32) -> Self {
        let n = value.0.abs();
        let w = value.0.signum();
        Self {
            x: (n / 1.0).fract() * 2.0 - 1.0,
            y: (n / 256.0).fract() * 2.0 - 1.0,
            z: (n / 65536.0).fract() * 2.0 - 1.0,
            w,
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedNormalU32(u32);

impl From<Vec3<f32>> for PackedNormalU32 {
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
pub struct PackedRGBAF32(f32);

impl From<Vec4<f32>> for PackedRGBAF32 {
    #[inline]
    fn from(value: Vec4<f32>) -> Self {
        Self({
            let x = value.x * 1.0;
            let y = value.y * 64.0;
            let z = value.z * 4096.0;
            let w = value.w * 262144.0;
            x + y + z + w
        })
    }
}

impl From<PackedRGBAF32> for Vec4<f32> {
    #[inline]
    fn from(value: PackedRGBAF32) -> Self {
        Self {
            x: (value.0 / 1.0).fract(),
            y: (value.0 / 64.0).fract(),
            z: (value.0 / 4096.0).fract(),
            w: (value.0 / 262144.0).fract(),
        }
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PackedVec4F32(u32);

impl From<Vec4<f32>> for PackedVec4F32 {
    #[inline]
    fn from(value: Vec4<f32>) -> Self {
        Self({
            let x = (value.x * 255.0) as u32;
            let y = (value.y * 255.0) as u32;
            let z = (value.z * 255.0) as u32;
            let w = (value.z * 255.0) as u32;
            x | (y << 4) | (z << 8) | (w << 12)
        })
    }
}

impl From<PackedVec4F32> for Vec4<f32> {
    #[inline]
    fn from(value: PackedVec4F32) -> Self {
        Self {
            x: (value.0 & 0xFF) as f32 / 255.0,
            y: ((value.0 >> 4) & 0xFF) as f32 / 255.0,
            z: ((value.0 >> 8) & 0xFF) as f32 / 255.0,
            w: ((value.0 >> 12) & 0xFF) as f32 / 255.0,
        }
    }
}

pub type PackedRGBAU32 = PackedVec4F32;
