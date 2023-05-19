use anyhow::Result;
use clap::Parser;

use crate::args::Cli;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse();
    todo!()
}
