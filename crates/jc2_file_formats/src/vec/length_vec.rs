use std::ops::{Deref, DerefMut};

use binrw::{binrw, BinRead, BinResult, BinWrite};
use num_traits::{NumCast, Unsigned};

use super::VecError;

type BinError = binrw::Error;

pub trait LengthType:
    BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()> + NumCast + Unsigned + Copy
{
}

impl<T> LengthType for T where
    T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()> + NumCast + Unsigned + Copy
{
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LengthVec<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    value: Vec<T>,
    marker: std::marker::PhantomData<L>,
}

impl<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType> LengthVec<T, L> {
    #[binrw::parser(reader, endian)]
    fn parse() -> BinResult<Vec<T>> {
        if let Some(count) = L::read_options(reader, endian, ())?.to_usize() {
            let mut result = Vec::with_capacity(count);
            for _ in 0..count {
                result.push(T::read_options(reader, endian, ())?);
            }
            Ok(result)
        } else {
            Err(BinError::Custom {
                pos: reader.stream_position()?,
                err: Box::new(VecError::InvalidLength),
            })
        }
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &Vec<T>) -> BinResult<()> {
        let length: Option<L> = NumCast::from(value.len());
        if let Some(length) = length {
            length.write_options(writer, endian, ())?;
            for element in value {
                element.write_options(writer, endian, ())?;
            }
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(VecError::InvalidLength),
            })
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<L>() + std::mem::size_of::<T>() * self.value.len()
    }
}

impl<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType> Deref
    for LengthVec<T, L>
{
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType> DerefMut
    for LengthVec<T, L>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType>
    From<LengthVec<T, L>> for Vec<T>
{
    fn from(value: LengthVec<T, L>) -> Self {
        value.value
    }
}

impl<T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()>, L: LengthType> From<Vec<T>>
    for LengthVec<T, L>
{
    fn from(value: Vec<T>) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}
