use godot::{classes::GeometryInstance3D, prelude::*};
use jc2_file_formats::{common::NullString, model_collection::ModelCollection};

use super::{JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcModelCollection();

impl JcResourceFormat for JcModelCollection {
    const EXTENSIONS: [&str; 1] = ["cgd"];
    type Result = GeometryInstance3D;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let mut cursor = binrw::io::Cursor::new(buffer.as_slice());
        match ModelCollection::read(&mut cursor) {
            Ok(collection) => {
                let models = load_models(&collection.models, thread)?;

                let mut result = GeometryInstance3D::new_alloc();
                for (index, instance) in collection.instances().enumerate() {
                    let model = &models[instance.model_index as usize];
                    let name: GString = GString::from(&model.get_name());

                    if let Some(model) = model.duplicate() {
                        let mut model = model.cast::<GeometryInstance3D>();

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
                Ok(result)
            }
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }
}

fn load_models(
    paths: &[NullString],
    thread: &mut JcResourceThread,
) -> JcResourceResult<Vec<Gd<GeometryInstance3D>>> {
    let mut models = Vec::with_capacity(paths.len());
    for path in paths {
        let model = thread.create_resource(path.as_ref().into())?;
        models.push(model.cast::<GeometryInstance3D>())
    }
    Ok(models)
}
