use binrw::BinRead;
use godot::prelude::*;
use jc2_file_formats::property_container::{PropertyBlockFile, PropertyContainer, PropertyFile};

use crate::property_container::JcPropertyContainer;

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcProperties();

impl JcResourceFormat<2> for JcProperties {
    const EXTENSIONS: [&str; 2] = ["bin", "bl"];
    type Result = JcPropertyContainer;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        _thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        match read(buffer.as_slice()) {
            Ok(container) => Ok(JcPropertyContainer::new(container.into())),
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

fn read(buffer: &[u8]) -> binrw::BinResult<PropertyContainer> {
    let header = &buffer[0..4];
    let mut cursor = binrw::io::Cursor::new(buffer);
    Ok(match &header {
        &b"PCBB" => PropertyBlockFile::read_le(&mut cursor)?.into(),
        _ => PropertyFile::read_le(&mut cursor)?.into(),
    })
}
