use std::{ops::Deref, path::PathBuf};

use clap::Parser;
use jc2_file_formats::{property_container::PropertyFile, BinRead};

#[derive(Parser)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.file.is_file() {
        let file = std::fs::File::open(args.file.clone())?;

        let properties = PropertyFile::read_le(&mut std::io::BufReader::new(file))?;
        for section in properties.sections.deref() {
            println!("{section:#?},");
        }
    }

    Ok(())
}
