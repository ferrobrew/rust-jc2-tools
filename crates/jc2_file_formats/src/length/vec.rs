use std::ops::{Deref, DerefMut};

use binrw::binrw;

use super::{BinReadWrite, BinResult, LengthType};

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LengthVec<T: BinReadWrite, L: LengthType> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    value: Vec<T>,
    marker: std::marker::PhantomData<L>,
}

impl<T: BinReadWrite, L: LengthType> LengthVec<T, L> {
    #[binrw::parser(reader, endian)]
    fn parse() -> BinResult<Vec<T>> {
        let count = <L as LengthType>::parse(reader, endian, ())?;
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(T::read_options(reader, endian, ())?);
        }
        Ok(result)
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &Vec<T>) -> BinResult<()> {
        <L as LengthType>::write(value.len(), writer, endian, ())?;
        for element in value {
            element.write_options(writer, endian, ())?;
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<L>() + std::mem::size_of::<T>() * self.value.len()
    }
}

impl<T: BinReadWrite, L: LengthType> AsRef<[T]> for LengthVec<T, L> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.value
    }
}

impl<T: BinReadWrite, L: LengthType> Deref for LengthVec<T, L> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: BinReadWrite, L: LengthType> DerefMut for LengthVec<T, L> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: BinReadWrite, L: LengthType> From<LengthVec<T, L>> for Vec<T> {
    #[inline]
    fn from(value: LengthVec<T, L>) -> Self {
        value.value
    }
}

impl<T: BinReadWrite, L: LengthType> From<Vec<T>> for LengthVec<T, L> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}

impl<T: BinReadWrite + Clone, L: LengthType> From<&[T]> for LengthVec<T, L> {
    #[inline]
    fn from(value: &[T]) -> Self {
        Self {
            value: value.into(),
            marker: Default::default(),
        }
    }
}
