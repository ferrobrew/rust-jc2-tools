use num_traits::{NumCast, Unsigned};
use thiserror::Error;

use crate::BinReadWrite;

type BinError = binrw::Error;
type BinResult<T> = binrw::BinResult<T>;

pub trait LengthType: BinReadWrite + NumCast + Unsigned + Copy {
    #[binrw::parser(reader, endian)]
    fn parse() -> BinResult<usize> {
        if let Some(count) = Self::read_options(reader, endian, ())?.to_usize() {
            Ok(count)
        } else {
            Err(BinError::Custom {
                pos: reader.stream_position()? - std::mem::size_of::<Self>() as u64,
                err: Box::new(LengthError::InvalidLength),
            })
        }
    }

    #[binrw::writer(writer, endian)]
    fn write(value: usize) -> BinResult<Self> {
        let length: Option<Self> = NumCast::from(value);
        if let Some(length) = length {
            length.write_options(writer, endian, ())?;
            Ok(length)
        } else {
            Err(BinError::Custom {
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

mod string;
pub use string::*;

mod vec;
pub use vec::*;
