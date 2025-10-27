use clap::Parser;
use jc2_file_formats::{BinRead, terrain::TerrainFile};

#[derive(Parser)]
struct Args {
    #[arg()]
    file: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.file.is_file() {
        let file = std::fs::File::open(args.file.clone())?;
        let mut reader = std::io::BufReader::new(file);

        let terrain = TerrainFile::read_le(&mut reader)?;

        std::fs::write(
            args.file.with_extension("normal.dds"),
            terrain.textures.normal_map,
        )?;

        std::fs::write(
            args.file.with_extension("material.dds"),
            terrain.textures.material_map,
        )?;

        std::fs::write(
            args.file.with_extension("weight.dds"),
            terrain.textures.weight_map,
        )?;

        std::fs::write(
            args.file.with_extension("minimap.dds"),
            terrain.textures.map_tile,
        )?;
    }

    Ok(())
}
