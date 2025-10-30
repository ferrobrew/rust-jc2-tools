use godot::{
    classes::{Image, ImageTexture},
    prelude::*,
};

use super::{
    GodotError, JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread,
};

pub fn register() -> JcResourceFormat {
    (GString::from("dds"), load)
}

pub fn load(
    path: GString,
    buffer: PackedByteArray,
    _thread: &mut JcResourceThread,
) -> JcResourceResult<Gd<Object>> {
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

    Ok(texture.upcast::<Object>())
}
