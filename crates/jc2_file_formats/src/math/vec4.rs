use binrw::binrw;

use super::{
    ops::{VecDot, VecLength},
    Vec3, VecType, VecTypeFloat,
};

#[binrw]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
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

impl<T: VecTypeFloat> From<[T; 4]> for Vec4<T> {
    #[inline]
    fn from(a: [T; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl<T: VecTypeFloat> From<Vec4<T>> for [T; 4] {
    #[inline]
    fn from(v: Vec4<T>) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl<T: VecTypeFloat> From<(T, T, T, T)> for Vec4<T> {
    #[inline]
    fn from(t: (T, T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl<T: VecTypeFloat> From<Vec4<T>> for (T, T, T, T) {
    #[inline]
    fn from(v: Vec4<T>) -> Self {
        (v.x, v.y, v.z, v.w)
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
