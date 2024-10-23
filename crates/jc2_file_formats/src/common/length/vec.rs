use std::ops::{Deref, DerefMut};

use binrw::binrw;

use super::{BinReadWrite, LengthType};

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LengthVec<T: BinReadWrite, L: LengthType, const B: bool = false> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    value: Vec<T>,
    marker: std::marker::PhantomData<L>,
}

impl<T: BinReadWrite, L: LengthType, const B: bool> LengthVec<T, L, B> {
    const WIDTH: usize = if B { std::mem::size_of::<T>() } else { 1 };

    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<Vec<T>> {
        let count = <L as LengthType>::parse(reader, endian, ())? / Self::WIDTH;
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(T::read_options(reader, endian, ())?);
        }
        Ok(result)
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &Vec<T>) -> binrw::BinResult<()> {
        <L as LengthType>::write(value.len() * Self::WIDTH, writer, endian, ())?;
        for element in value {
            element.write_options(writer, endian, ())?;
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<L>() + std::mem::size_of::<T>() * self.value.len()
    }
}

impl<T: BinReadWrite, L: LengthType, const B: bool> AsRef<[T]> for LengthVec<T, L, B> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.value
    }
}

impl<T: BinReadWrite, L: LengthType, const B: bool> Deref for LengthVec<T, L, B> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: BinReadWrite, L: LengthType, const B: bool> DerefMut for LengthVec<T, L, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: BinReadWrite, L: LengthType, const B: bool> From<LengthVec<T, L, B>> for Vec<T> {
    #[inline]
    fn from(value: LengthVec<T, L, B>) -> Self {
        value.value
    }
}

impl<T: BinReadWrite, L: LengthType, const B: bool> From<Vec<T>> for LengthVec<T, L, B> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}

impl<T: BinReadWrite + Clone, L: LengthType, const B: bool> From<&[T]> for LengthVec<T, L, B> {
    #[inline]
    fn from(value: &[T]) -> Self {
        Self {
            value: value.into(),
            marker: Default::default(),
        }
    }
}
