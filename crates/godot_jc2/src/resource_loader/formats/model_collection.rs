use std::collections::{HashMap, HashSet};

use godot::prelude::*;
use jc2_file_formats::{
    common::NullString,
    model_collection::{ModelCollection, VegetationInstance},
};
use jc2_hashing::HashString;

use crate::resource_loader::formats::JcLod;

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcModelCollection();

impl JcResourceFormat for JcModelCollection {
    const EXTENSIONS: [&str; 1] = ["cgd"];
    type Result = Node3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let mut cursor = binrw::io::Cursor::new(buffer.as_slice());
        match ModelCollection::read(&mut cursor) {
            Ok(collection) => {
                let (models, vegetation) =
                    load(&collection.models, &collection.vegetation_instances, thread)?;

                let mut result = Node3D::new_alloc();
                for (index, instance) in collection.instances().enumerate() {
                    let model = &models[instance.model_index as usize];
                    let name: GString = GString::from(&model.get_name());

                    if let Some(model) = model.duplicate() {
                        let mut model = model.cast::<Node3D>();

                        let index = GString::from(&index.to_string());
                        model.set_name(GString::new().join(&[name, index].into()).arg());

                        let transform = &instance.transform;
                        model.set_transform(Transform3D::from_cols(
                            Vector3::new(transform[0], transform[1], transform[2]),
                            Vector3::new(transform[4], transform[5], transform[6]),
                            Vector3::new(transform[8], transform[9], transform[10]),
                            Vector3::new(transform[12], transform[13], transform[14]),
                        ));

                        result.add_child(&model);
                    };
                }
                for (index, instance) in collection.vegetation_instances.iter().enumerate() {
                    let model = &vegetation[&instance.model_hash];
                    let name: GString = GString::from(&model.get_name());

                    if let Some(model) = model.duplicate() {
                        let mut model = model.cast::<Node3D>();

                        let index = GString::from(&index.to_string());
                        model.set_name(GString::new().join(&[name, index].into()).arg());

                        let basis = Basis::from_axis_angle(
                            Vector3::UP,
                            instance.yaw as f32 / u8::MAX as f32,
                        );
                        model.set_transform(Transform3D::from_cols(
                            basis.rows[0],
                            basis.rows[1],
                            basis.rows[2],
                            Vector3::new(
                                instance.position.x,
                                instance.position.y,
                                instance.position.z,
                            ),
                        ));

                        result.add_child(&model);
                    };
                }
                Ok(result)
            }
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

fn load(
    models: &[NullString],
    vegetation: &[VegetationInstance],
    thread: &mut JcResourceThread,
) -> JcResourceResult<(Vec<Gd<Node3D>>, HashMap<HashString, Gd<Node3D>>)> {
    let mut models = load_models(models, thread)?;
    match load_vegetation(vegetation, thread) {
        Ok(vegetation) => Ok((models, vegetation)),
        Err(error) => {
            for model in &mut models {
                model.queue_free();
            }
            Err(error)
        }
    }
}

fn load_models(
    paths: &[NullString],
    thread: &mut JcResourceThread,
) -> JcResourceResult<Vec<Gd<Node3D>>> {
    let mut models = Vec::with_capacity(paths.len());
    for path in paths {
        match thread.create_resource_from_path(path.as_ref().into()) {
            Ok(model) => models.push(model.cast::<Node3D>()),
            Err(error) => {
                for model in &mut models {
                    model.queue_free();
                }
                return Err(error);
            }
        }
    }
    Ok(models)
}

fn load_vegetation(
    vegetation: &[VegetationInstance],
    thread: &mut JcResourceThread,
) -> JcResourceResult<HashMap<HashString, Gd<Node3D>>> {
    let hashes: HashSet<HashString> = vegetation.into_iter().map(|i| i.model_hash).collect();
    let hashes: Vec<HashString> = hashes.into_iter().collect();
    let mut models = Vec::with_capacity(hashes.len());
    for hash in &hashes {
        match thread.create_resource_from_hash::<JcLod>(*hash) {
            Ok(model) => models.push(model.cast::<Node3D>()),
            Err(error) => {
                for model in &mut models {
                    model.queue_free();
                }
                return Err(error);
            }
        }
    }
    Ok(hashes.into_iter().zip(models.into_iter()).collect())
}
