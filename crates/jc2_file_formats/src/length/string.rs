use std::ops::{Deref, DerefMut};

use binrw::binrw;

use super::{BinResult, LengthType};

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LengthString<T: LengthType> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    value: String,
    marker: std::marker::PhantomData<T>,
}

impl<T: LengthType> LengthString<T> {
    #[binrw::parser(reader, endian)]
    fn parse() -> BinResult<String> {
        let mut buffer = vec![0u8; <T as LengthType>::parse(reader, endian, ())?];
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &String) -> BinResult<()> {
        <T as LengthType>::write(value.as_bytes().len(), writer, endian, ())?;
        writer.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<T>() + self.value.as_bytes().len()
    }
}

impl<T: LengthType> AsRef<str> for LengthString<T> {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<T: LengthType> Deref for LengthString<T> {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: LengthType> DerefMut for LengthString<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: LengthType> From<LengthString<T>> for String {
    #[inline]
    fn from(value: LengthString<T>) -> Self {
        value.value
    }
}

impl<T: LengthType> From<String> for LengthString<T> {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}

impl<T: LengthType> From<&str> for LengthString<T> {
    #[inline]
    fn from(value: &str) -> Self {
        Self {
            value: value.into(),
            marker: Default::default(),
        }
    }
}
