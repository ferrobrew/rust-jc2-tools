use binrw::BinRead;
use godot::prelude::*;
use jc2_file_formats::property_container::{PropertyBlockFile, PropertyContainer, PropertyFile};

use super::{
    JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread,
    classes::game_objects::vehicles::props::JcVehiclePropData,
};

pub struct JcVehicleDoll();

impl JcResourceFormat for JcVehicleDoll {
    const EXTENSIONS: &[&str] = &["mvdoll", "vdoll"];
    type Result = Node3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        match read_properties(buffer.as_slice()) {
            Ok(properties) => {
                let Some(properties) = properties.get(0).and_then(|x| x.get_container("_vdoll"))
                else {
                    return Err(JcResourceError::FileAccess {
                        path,
                        error: godot::global::Error::ERR_INVALID_DATA,
                    });
                };

                let mut root = Node3D::new_alloc();
                root.set_name(&StringName::from(&path.get_file().get_basename()));

                let mut vehicle = JcVehiclePropData::default();
                vehicle.read(&properties);

                if let Some(modules) = properties.get_container("_default_modules") {
                    match load_hierarchy("Modules", &modules, thread) {
                        Ok(node) => {
                            root.add_child(&node);
                        }
                        Err(err) => {
                            root.free();
                            return Err(err);
                        }
                    }
                }

                if let Some(parts) = properties.get_container("_parts") {
                    match load_hierarchy("Parts", &parts, thread) {
                        Ok(node) => {
                            root.add_child(&node);
                        }
                        Err(err) => {
                            root.free();
                            return Err(err);
                        }
                    }

                    // TODO: recursively load _modules, _effects, and _smartobjects... kind of implies we need to be able to extend import/export with visitor...?
                }

                if let Some(families) = properties.get_container("_families") {
                    match load_hierarchy("Families", &families, thread) {
                        Ok(node) => {
                            root.add_child(&node);
                        }
                        Err(err) => {
                            root.free();
                            return Err(err);
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

// TODO: This could should be shared with JcLocation?
fn load_hierarchy(
    name: &str,
    properties: &PropertyContainer,
    thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Node3D>> {
    let mut root = Node3D::new_alloc();
    root.set_name(&StringName::from(&GString::from(name)));

    let mut stack: Vec<_> = properties
        .containers()
        .map(|p| (p, root.clone(), root.get_transform().affine_inverse()))
        .collect();
    while let Some((properties, mut parent, parent_world_inverse)) = stack.pop() {
        if properties.contains("_class") {
            match thread.import(properties) {
                Ok(mut object) => {
                    let object_world = object.get_transform();
                    stack.extend(
                        properties
                            .containers()
                            .map(|p| (p, object.clone(), object_world.affine_inverse())),
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
