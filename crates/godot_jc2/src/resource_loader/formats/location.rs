use binrw::BinRead;
use godot::prelude::*;
use jc2_file_formats::property_container::{
    FromPropertyValue, PropertyBlockFile, PropertyContainer, PropertyFile, PropertyValue,
};

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcLocation();

impl JcResourceFormat<3> for JcLocation {
    const EXTENSIONS: [&str; 3] = ["blo", "bl", "epe"];
    type Result = Node3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        match read_properties(buffer.as_slice()) {
            Ok(properties) => {
                let mut parent = Node3D::new_alloc();
                let mut objects: Vec<Gd<Node3D>> = vec![];

                for properties in &properties {
                    if let Err(error) = create_objects(
                        &mut parent,
                        Transform3D::default(),
                        &properties,
                        &mut objects,
                        thread,
                    ) {
                        for mut object in objects {
                            object.queue_free();
                        }
                        parent.queue_free();
                        return Err(error);
                    }
                }
                Ok(parent)
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

fn create_objects(
    parent: &mut Gd<Node3D>,
    transform: Transform3D,
    properties: &PropertyContainer,
    objects: &mut Vec<Gd<Node3D>>,
    thread: &mut JcResourceThread,
) -> JcResourceResult<()> {
    struct JcTransform(Transform3D);

    impl<'a> FromPropertyValue<'a> for JcTransform {
        fn from_property_value(value: &'a PropertyValue) -> Option<Self> {
            match value {
                PropertyValue::Mat3x4(value) => Some(Self(Transform3D::from_cols(
                    Vector3::new(value[0], value[1], value[2]),
                    Vector3::new(value[3], value[4], value[5]),
                    Vector3::new(value[6], value[7], value[8]),
                    Vector3::new(value[9], value[10], value[11]),
                ))),
                _ => None,
            }
        }
    }

    if let Some(world) = properties.get_value::<JcTransform>("world") {
        let mut child: Option<Gd<Node3D>> = match properties.get_value::<&str>("_class") {
            Some("CDamageableObject" | "CLandmark" | "CPlantedTree" | "CSimpleRigidObject") => {
                match properties.get_value::<&str>("filename") {
                    None | Some("") => None,
                    Some(path) => Some(thread.create_resource_from_path(path.into())?.cast()),
                }
            }
            None | Some(_) => None,
        };

        let transform = transform * world.0;
        if let Some(child) = &mut child {
            parent.add_child(&*child);
            child.set_transform(transform);
            objects.push(child.clone());
        }

        let mut parent = child.unwrap_or_else(|| parent.clone());
        for properties in properties.containers() {
            create_objects(&mut parent, transform, properties, objects, thread)?;
        }
    } else {
        for properties in properties.containers() {
            create_objects(parent, transform, properties, objects, thread)?;
        }
    }

    Ok(())
}
