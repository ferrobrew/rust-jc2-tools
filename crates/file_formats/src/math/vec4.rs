use binrw::binrw;

use super::{
    ops::{VecDot, VecLength},
    Vec3, VecType, VecTypeFloat,
};

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec4<T: VecType> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VecType> From<Vec3<T>> for Vec4<T> {
    fn from(value: Vec3<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: Default::default(),
        }
    }
}

impl<T: VecType> Vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn splat(value: T) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }
}

impl<T: VecTypeFloat> VecLength<T> for Vec4<T> {
    #[inline]
    fn length(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    fn length_squared(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

impl<T: VecTypeFloat> VecDot<T> for Vec4<T> {
    #[inline]
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
