use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::bail;
use clap::{Parser, Subcommand};
use jc2_hashing::HashString;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Prunes conflicting hashes in source from target")]
    Prune {
        source: std::path::PathBuf,
        target: std::path::PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Prune { source, target } => {
            if !source.is_file() {
                bail!("{source:?} is not a file");
            }

            if !target.is_file() {
                bail!("{target:?} is not a file");
            }

            if source == target {
                bail!("source and target must not be the same");
            }

            let source_set: HashSet<HashString> = fs::read_to_string(source)?
                .lines()
                .map(HashString::from)
                .collect();

            let (target_set, mut target_map): (HashSet<HashString>, HashMap<HashString, String>) = {
                let content = fs::read_to_string(&target)?;
                let hashes: Vec<HashString> = content.lines().map(HashString::from).collect();

                (
                    hashes.iter().cloned().collect(),
                    hashes
                        .into_iter()
                        .zip(content.lines().map(str::to_string))
                        .collect(),
                )
            };

            let mut removed = 0usize;
            for conflict in source_set.intersection(&target_set) {
                target_map.remove(conflict);
                removed += 1;
            }
            println!("Removed {removed} from {target:?}");

            fs::rename(&target, target.with_extension("old"))?;

            fs::write(
                target,
                target_map
                    .values()
                    .fold(String::new(), |result, value| format!("{result}\n{value}")),
            )?;
        }
    }

    Ok(())
}
