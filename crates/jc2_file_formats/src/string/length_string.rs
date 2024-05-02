use binrw::{binrw, BinRead, BinResult, BinWrite};
use num_traits::{NumCast, Unsigned};

use super::StringError;

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
pub struct LengthString<T: LengthType> {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    value: String,
    marker: std::marker::PhantomData<T>,
}

impl<T: LengthType> LengthString<T> {
    #[binrw::parser(reader, endian)]
    fn parse() -> BinResult<String> {
        if let Some(count) = T::read_options(reader, endian, ())?.to_usize() {
            let mut buffer = vec![0u8; count];
            reader.read_exact(&mut buffer)?;
            Ok(String::from_utf8_lossy(&buffer).to_string())
        } else {
            Err(BinError::Custom {
                pos: reader.stream_position()?,
                err: Box::new(StringError::InvalidLength),
            })
        }
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &String) -> BinResult<()> {
        let length: Option<T> = NumCast::from(value.as_bytes().len());
        if let Some(length) = length {
            length.write_options(writer, endian, ())?;
            writer.write_all(value.as_bytes())?;
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(StringError::InvalidLength),
            })
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<u32>() + self.value.as_bytes().len()
    }
}

impl<T: LengthType> AsRef<str> for LengthString<T> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<T: LengthType> From<LengthString<T>> for String {
    fn from(value: LengthString<T>) -> Self {
        value.value
    }
}

impl<T: LengthType> From<String> for LengthString<T> {
    fn from(value: String) -> Self {
        Self {
            value,
            marker: Default::default(),
        }
    }
}
