use anyhow::{bail, Result};
use clap::Parser;
use commands::{decode, encode, print_chunks, remove};

use crate::args::Cli;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        args::Commands::Encode {
            file,
            message,
            kind,
        } => {
            let kind = kind.unwrap();
            eprintln!("Using '{}' chunk type...", kind);
            fs_err::write(&file, encode(&file, &message, kind)?.as_bytes())?;
            eprintln!("The message has been successfully encoded into the PNG file.");
        }
        args::Commands::Decode { file, kind } => {
            let kind = kind.unwrap();
            let decoded_chunks = decode(&file, kind)?;
            if decoded_chunks.is_empty() {
                bail!(
                    "No chunks of type '{}' were found in \"{}\"",
                    kind,
                    file.display()
                )
            } else {
                eprintln!(
                    "Found {} chunk{} of type '{}'...\n",
                    decoded_chunks.len(),
                    if decoded_chunks.len() == 1 { "" } else { "s" },
                    kind
                );
                for chunk in decoded_chunks {
                    println!("{}", chunk);
                }
            }
        }
        args::Commands::Remove { file, kind, all } => {
            let kind = kind.unwrap();
            eprintln!("Using '{}' chunk type...", kind);

            if all {
                let mut png = None;
                loop {
                    match remove(&file, kind) {
                        Ok(_png) => {
                            png = Some(_png);
                        }
                        Err(err) => {
                            if let Some(png) = png {
                                fs_err::write(&file, png.as_bytes())?;
                                eprintln!("All matching chunks have been successfully removed from the PNG file.");
                            } else {
                                bail!(err)
                            }

                            break;
                        }
                    }
                }
            } else {
                fs_err::write(&file, remove(&file, kind)?.as_bytes())?;
                eprintln!("One matching chunk has been successfully removed from the PNG file.");
            }
        }
        args::Commands::Print { file } => {
            print_chunks(file)?;
        }
    }

    Ok(())
}
