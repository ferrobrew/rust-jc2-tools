use std::ops::{Add, Div, Mul, Sub};

use binrw::binrw;

use super::{
    ops::{VecCross, VecDot, VecLength},
    Vec4, VecType, VecTypeFloat,
};

#[binrw]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vec3<T: VecType> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VecType> From<Vec4<T>> for Vec3<T> {
    fn from(value: Vec4<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl<T: VecTypeFloat> From<[T; 3]> for Vec3<T> {
    #[inline]
    fn from(a: [T; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl<T: VecTypeFloat> From<Vec3<T>> for [T; 3] {
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T: VecTypeFloat> From<(T, T, T)> for Vec3<T> {
    #[inline]
    fn from(t: (T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl<T: VecTypeFloat> From<Vec3<T>> for (T, T, T) {
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        (v.x, v.y, v.z)
    }
}

impl<T: VecType> Vec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(value: T) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline]
    pub const fn extend(&self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
}

impl<T: VecType> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: VecType> Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: VecType> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: VecType> Div<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: VecTypeFloat> VecLength<T> for Vec3<T> {
    #[inline]
    fn length(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    fn length_squared(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T: VecTypeFloat> VecCross<T> for Vec3<T> {
    #[inline]
    fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: VecTypeFloat> VecDot<T> for Vec3<T> {
    #[inline]
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
