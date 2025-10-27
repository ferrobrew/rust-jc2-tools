use num_traits::{NumCast, Unsigned};
use thiserror::Error;

use crate::BinReadWrite;

pub trait LengthType: BinReadWrite + NumCast + Unsigned + Copy {
    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<usize> {
        if let Some(count) = Self::read_options(reader, endian, ())?.to_usize() {
            Ok(count)
        } else {
            Err(binrw::Error::Custom {
                pos: reader.stream_position()? - std::mem::size_of::<Self>() as u64,
                err: Box::new(LengthError::InvalidLength),
            })
        }
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write(value: usize) -> binrw::BinResult<Self> {
        let length: Option<Self> = NumCast::from(value);
        if let Some(length) = length {
            length.write_options(writer, endian, ())?;
            Ok(length)
        } else {
            Err(binrw::Error::Custom {
                pos: writer.stream_position()?,
                err: Box::new(LengthError::InvalidLength),
            })
        }
    }
}

impl<T> LengthType for T where T: BinReadWrite + NumCast + Unsigned + Copy {}

#[derive(Error, Debug)]
pub enum LengthError {
    #[error("invalid length")]
    InvalidLength,
}

mod bitvec;
pub use bitvec::*;

mod string;
pub use string::*;

mod vec;
pub use vec::*;
