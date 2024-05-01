use binrw::{BinRead, BinWrite};
use num_traits::{Float, Num};

pub trait VecType:
    for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()> + Num + Copy + Default
{
}

impl<T> VecType for T where
    T: for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()> + Num + Copy + Default
{
}

pub trait VecTypeFloat: VecType + Float {}

impl<T: VecType + Float> VecTypeFloat for T {}

pub mod ops;

mod vec2;
pub use vec2::*;

mod vec3;
pub use vec3::*;

mod vec4;
pub use vec4::*;
