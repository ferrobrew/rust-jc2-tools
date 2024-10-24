use std::io::{Read, Seek};

use clap::Parser;
use jc2_file_formats::{
    property_container::{PropertyBlockFile, PropertyContainer, PropertyFile},
    BinRead,
};

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

        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        reader.seek(std::io::SeekFrom::Start(0))?;

        let properties: PropertyContainer = match &header {
            b"PCBB" => PropertyBlockFile::read_le(&mut reader)?.into(),
            _ => PropertyFile::read_le(&mut reader)?.into(),
        };
        println!("{properties:#?}");
    }

    Ok(())
}
