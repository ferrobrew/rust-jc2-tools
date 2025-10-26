use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};

#[binrw]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct NullString {
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    pub value: String,
}

impl NullString {
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<String> {
        let mut buffer = vec![];
        loop {
            let val = u8::read_options(reader, endian, ())?;
            if val == 0 {
                return Ok(String::from_utf8_lossy(&buffer).to_string());
            }
            buffer.push(val);
        }
    }

    #[binrw::writer(writer, endian)]
    fn write(value: &String) -> binrw::BinResult<()> {
        value.as_bytes().write_options(writer, endian, ())?;
        0u8.write_options(writer, endian, ())?;
        Ok(())
    }

    pub fn size(&self) -> usize {
        1 + self.value.as_bytes().len()
    }
}

impl AsRef<str> for NullString {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Deref for NullString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for NullString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl From<NullString> for String {
    #[inline]
    fn from(value: NullString) -> Self {
        value.value
    }
}

impl From<String> for NullString {
    #[inline]
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for NullString {
    #[inline]
    fn from(value: &str) -> Self {
        Self {
            value: value.into(),
        }
    }
}
