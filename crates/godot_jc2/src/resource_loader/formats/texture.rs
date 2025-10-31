use godot::{
    classes::{Image, ImageTexture},
    prelude::*,
};

use super::{GodotError, JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread};

pub struct JcTexture();

impl JcResourceFormat for JcTexture {
    const EXTENSIONS: [&str; 1] = ["dds"];
    type Result = ImageTexture;

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        _thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<ImageTexture>> {
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

        Ok(texture)
    }
}
