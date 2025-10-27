use std::ops::{Deref, DerefMut};

use binrw::binrw;
use bitvec::{store::BitStore, vec::BitVec};

use super::{BinReadWrite, LengthType};

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LengthBitVec<T: BinReadWrite + BitStore, L: LengthType> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    pub value: BitVec<T>,
    marker: std::marker::PhantomData<L>,
}

impl<T: BinReadWrite + BitStore, L: LengthType> LengthBitVec<T, L> {
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<BitVec<T>> {
        let count = <L as LengthType>::parse(reader, endian, ())?;
        let mut result = BitVec::with_capacity(count);
        if count > 0 {
            let chunks = 1 + (count / (size_of::<T>() * 8));
            for _ in 0..chunks {
                result.extend_from_raw_slice(&[T::read_options(reader, endian, ())?]);
            }
            result.truncate(count);
        }
        Ok(result)
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &BitVec<T>) -> binrw::BinResult<()> {
        <L as LengthType>::write(value.len(), writer, endian, ())?;
        if !value.is_empty() {
            for element in value.as_raw_slice() {
                element.write_options(writer, endian, ())?;
            }
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        if self.value.is_empty() {
            std::mem::size_of::<L>()
        } else {
            let chunks = 1 + (self.value.len() / (size_of::<T>() * 8));
            std::mem::size_of::<L>() + std::mem::size_of::<T>() * chunks
        }
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> Deref for LengthBitVec<T, L> {
    type Target = BitVec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> DerefMut for LengthBitVec<T, L> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> AsRef<[T]> for LengthBitVec<T, L> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.value.as_raw_slice()
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> AsMut<[T]> for LengthBitVec<T, L> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.value.as_raw_mut_slice()
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> IntoIterator for LengthBitVec<T, L> {
    type Item = bool;
    type IntoIter = <BitVec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> From<LengthBitVec<T, L>> for BitVec<T> {
    #[inline]
    fn from(value: LengthBitVec<T, L>) -> Self {
        value.value
    }
}

impl<T: BinReadWrite + BitStore, L: LengthType> From<BitVec<T>> for LengthBitVec<T, L> {
    #[inline]
    fn from(value: BitVec<T>) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}

impl<T: BinReadWrite + BitStore + Clone, L: LengthType> From<&[T]> for LengthBitVec<T, L> {
    #[inline]
    fn from(value: &[T]) -> Self {
        Self {
            value: BitVec::<T>::from_slice(value),
            marker: Default::default(),
        }
    }
}
