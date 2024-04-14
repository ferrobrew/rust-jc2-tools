use binrw::binrw;

use super::{
    ops::{VecDot, VecLength},
    VecType, VecTypeFloat,
};

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec2<T: VecType> {
    pub x: T,
    pub y: T,
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
