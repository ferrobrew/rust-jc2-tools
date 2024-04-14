use super::VecTypeFloat;

pub trait VecLength<T: VecTypeFloat> {
    fn length(self) -> T;
    fn length_squared(self) -> T;
}

pub trait VecCross<T: VecTypeFloat> {
    fn cross(self, rhs: Self) -> Self;
}

pub trait VecDot<T: VecTypeFloat> {
    fn dot(self, rhs: Self) -> T;
}
