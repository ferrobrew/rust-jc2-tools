use binrw::BinRead;
use godot::prelude::*;
use jc2_file_formats::property_container::{PropertyBlockFile, PropertyContainer, PropertyFile};

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcLocation();

impl JcResourceFormat for JcLocation {
    const EXTENSIONS: &[&str] = &["blo", "bl", "epe"];
    type Result = Node3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        match read_properties(buffer.as_slice()) {
            Ok(properties) => {
                let mut root = Node3D::new_alloc();
                root.set_name(&StringName::from(&path.get_file().get_basename()));

                // TODO: This could should be shared with JcVehicleDoll?
                let mut stack: Vec<_> = properties
                    .iter()
                    .map(|p| (p, root.clone(), root.get_transform()))
                    .collect();
                while let Some((properties, mut parent, parent_world_inverse)) = stack.pop() {
                    if properties.contains("_class") {
                        match thread.import(properties) {
                            Ok(mut object) => {
                                let object_world = object.get_transform();
                                stack.extend(
                                    properties
                                        .containers()
                                        .map(|p| (p, object.clone(), object_world)),
                                );
                                object.set_transform(parent_world_inverse * object_world);
                                parent
                                    .add_child_ex(&object)
                                    .force_readable_name(true)
                                    .done();
                            }
                            Err(err) => {
                                root.free();
                                return Err(err);
                            }
                        }
                    }
                }

                Ok(root)
            }
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

fn read_properties(buffer: &[u8]) -> binrw::BinResult<Vec<PropertyContainer>> {
    let header = &buffer[0..4];
    let mut cursor = binrw::io::Cursor::new(buffer);
    Ok(match &header {
        &b"PCBB" => PropertyBlockFile::read_le(&mut cursor)?.into(),
        _ => PropertyFile::read_le(&mut cursor)?.into(),
    })
}
