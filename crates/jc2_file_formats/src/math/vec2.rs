use binrw::binrw;

use super::{
    ops::{VecDot, VecLength},
    VecType, VecTypeFloat,
};

#[binrw]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vec2<T: VecType> {
    pub x: T,
    pub y: T,
}

impl<T: VecTypeFloat> From<[T; 2]> for Vec2<T> {
    #[inline]
    fn from(a: [T; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl<T: VecTypeFloat> From<Vec2<T>> for [T; 2] {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        [v.x, v.y]
    }
}

impl<T: VecTypeFloat> From<(T, T)> for Vec2<T> {
    #[inline]
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T: VecTypeFloat> From<Vec2<T>> for (T, T) {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        (v.x, v.y)
    }
}

impl<T: VecType> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(value: T) -> Self {
        Self { x: value, y: value }
    }
}

impl<T: VecTypeFloat> VecLength<T> for Vec2<T> {
    #[inline]
    fn length(self) -> T {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    fn length_squared(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: VecTypeFloat> VecDot<T> for Vec2<T> {
    #[inline]
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}
