use godot::{classes::GeometryInstance3D, prelude::*};

use super::{
    GodotError, JcModel, JcResourceError, JcResourceFormat, JcResourceResult, JcResourceThread,
};

const LOD_DISTANCES: [f32; 6] = [0.0, 10.0, 25.0, 50.0, 100.0, 500.0];

#[allow(non_camel_case_types)]
pub struct JcLod();

impl JcResourceFormat for JcLod {
    const EXTENSIONS: [&str; 1] = ["lod"];
    type Result = GeometryInstance3D;

    /* FORMAT:
     *  - files are only valid if they contain >= 6 lines
     *  - first five lines are rbm paths, the sixth is the lod factor
     *  - the same rbm can occupy multiple levels
     *  - if the rbm file name is "-" then it's intended to be hidden (not implemented)
     *  - a lod factor of -1.0 means "use fov factor only " (not implemented)
     *
     * LOD:
     *  - Default FOV is 46.8 degrees vertical FOV
     *  - FOV LOD factor is calculated using `max(tan(default_fov * 0.5) / tan(fov * 0.5), 1.0)`
     *  - The game uses 0-10, 10-25, 25-50, 50-100, 100-500 meters as LOD distances
     */
    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let file = String::from_utf8_lossy(buffer.as_slice()).to_owned();
        let lines: Vec<&str> = file.lines().collect();

        if lines.len() >= 6 {
            let lods = &lines[0..5];
            let factor: f32 = lines[5].parse().unwrap_or(1f32).abs();

            let mut lod_count = 1;
            let mut meshes = Vec::with_capacity(5);
            for lod_level in 1..6 {
                let previous_lod = lods[lod_level - 1];
                let next_lod = lod_level == 5 || previous_lod != lods[lod_level];

                if next_lod && !previous_lod.ends_with('-') {
                    let path = GString::from(previous_lod);
                    let mut mesh = JcModel::from_path(path, thread)?;

                    mesh.set_visibility_range_begin(LOD_DISTANCES[lod_level - lod_count] * factor);
                    mesh.set_visibility_range_end(LOD_DISTANCES[lod_level] * factor);

                    meshes.push(mesh);
                    lod_count = 1;
                } else {
                    lod_count += 1;
                }
            }

            let mut instance = GeometryInstance3D::new_alloc();
            for mesh in meshes {
                instance.add_child(&mesh);
            }

            Ok(instance)
        } else {
            Err(JcResourceError::FileAccess {
                path,
                error: GodotError::ERR_INVALID_DATA,
            })
        }
    }
}
