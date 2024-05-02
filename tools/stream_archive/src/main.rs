use std::path::PathBuf;

use clap::Parser;
use jc2_file_formats::archive::StreamArchive;

#[derive(Parser)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.file.is_dir() {
        let mut archive = StreamArchive::default();
        for file in std::fs::read_dir(args.file.clone())? {
            let file = file?;
            if !file.path().is_file() {
                continue;
            }
            let file_name = file.file_name();
            archive.entries.insert(
                file_name.to_string_lossy().into(),
                std::fs::read(file.path())?,
            );
        }
        archive.write(&mut std::fs::File::create(args.file.with_extension("ee"))?)?;
    } else if args.file.is_file() {
        let file = std::fs::File::open(args.file.clone())?;
        let archive = StreamArchive::read(&mut std::io::BufReader::new(file))?;
        let root = args.file.with_extension("");
        std::fs::create_dir(&root)?;
        for (name, data) in &archive.entries {
            std::fs::write(root.clone().join(name), data)?;
        }
    }

    Ok(())
}
