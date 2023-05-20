use std::path::PathBuf;

use clap::{Parser, Subcommand};

use clap_complete::Shell;
use pngwhisper::png::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a message into a PNG image
    #[command()]
    Encode {
        #[arg()]
        file: PathBuf,
        #[arg()]
        message: String,
        /// Chunk type for the message
        #[arg(short, long, default_value = "wsPr")]
        kind: Option<ChunkType>,
    },
    /// Decode a message in a PNG image
    #[command()]
    Decode {
        #[arg()]
        file: PathBuf,
        /// Chunk type used for the encoded message
        #[arg(short, long, default_value = "wsPr")]
        kind: Option<ChunkType>,
    },
    /// Remove a chunk from a PNG image
    #[command(
        after_help = "Caution: Use with care! The PNG file may become corrupted if an incorrect chunk type is used."
    )]
    Remove {
        #[arg()]
        file: PathBuf,
        /// Chunk type used for the encoded message
        #[arg(short, long, default_value = "wsPr")]
        kind: Option<ChunkType>,
        #[arg(short, long)]
        all: bool,
    },
    /// Prints all of the chunks in a PNG file
    #[command()]
    Print {
        #[arg()]
        file: PathBuf,
    },
    #[command()]
    /// Generate tab-completion scripts for your shell
    Completions {
        #[arg()]
        shell: Shell,
    },
}
