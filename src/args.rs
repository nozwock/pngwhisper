use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

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
    /// Decode a message from a PNG image
    #[command()]
    Decode {
        #[arg()]
        file: PathBuf,
        /// Chunk type used for the encoded message
        #[arg(short, long, default_value = "wsPr")]
        kind: Option<ChunkType>,
    },
    /// Remove all messages from a PNG image
    #[command(
        after_help = "Caution: Use with care! The PNG file may become corrupted if an incorrect chunk type is used."
    )]
    Remove {
        #[arg()]
        file: PathBuf,
        /// Chunk type used for the encoded message
        #[arg(short, long, default_value = "wsPr")]
        kind: Option<ChunkType>,
    },
    /// Prints a list of searchable chunks in a PNG image
    #[command()]
    Print {
        #[arg()]
        file: PathBuf,
    },
}
