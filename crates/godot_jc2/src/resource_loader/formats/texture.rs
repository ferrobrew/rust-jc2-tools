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
        let mut buffer = buffer;

        // Workaround for https://github.com/godotengine/godot/pull/108720
        if let Ok(dds) = image_dds::ddsfile::Dds::read(buffer.as_slice()) {
            let num_levels = dds.get_num_mipmap_levels();
            let required_levels = dds.get_width().max(dds.get_height()).ilog2() + 1;

            if num_levels > 1 && num_levels < required_levels {
                if let (Ok(image), Ok(format)) = (
                    image_dds::image_from_dds(&dds, 0),
                    image_dds::dds_image_format(&dds),
                ) {
                    if let Ok(dds) = image_dds::dds_from_image(
                        &image,
                        format,
                        image_dds::Quality::Normal,
                        image_dds::Mipmaps::GeneratedAutomatic,
                    ) {
                        let mut bytes = Vec::new();

                        if dds.write(&mut bytes).is_ok() {
                            buffer = PackedByteArray::from(bytes);
                        }
                    }
                }
            }
        }

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
