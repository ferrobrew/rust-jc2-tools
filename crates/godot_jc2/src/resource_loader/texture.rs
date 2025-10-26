use godot::{
    classes::{Image, ImageTexture},
    prelude::*,
};

use crate::resource_loader::JcResourceThread;

use super::{GodotError, JcResourceError, JcResourceResult};

pub const EXTENSION: &str = "dds";

pub fn load(
    path: GString,
    buffer: PackedByteArray,
    _thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Resource>> {
    let mut image = Image::new_gd();
    let error = image.load_dds_from_buffer(&buffer);
    if error != GodotError::OK {
        return Err(JcResourceError::FileAccess { path, error });
    }

    let Some(texture) = ImageTexture::create_from_image(&image) else {
        return Err(JcResourceError::FileAccess {
            path,
            error: GodotError::FAILED,
        });
    };

    Ok(texture.upcast::<Resource>())
}
