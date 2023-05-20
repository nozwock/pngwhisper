use std::path::Path;

use anyhow::Result;
use console::style;
use itertools::Itertools;

use pngwhisper::png::{chunk::Chunk, chunk_type::ChunkType, Png};

/// Encodes a message into a PNG image
pub fn encode<P>(file: P, message: &str, chunk_type: ChunkType) -> Result<Png>
where
    P: AsRef<Path>,
{
    let mut png = Png::from_path(file)?;
    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().into()));
    Ok(png)
}

/// Searches for hidden messages in a PNG image
pub fn decode<P>(file: P, chunk_type: ChunkType) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let png = Png::from_path(file)?;
    Ok(png
        .chunks_by_type(&chunk_type)
        .into_iter()
        .map(|chunk| chunk.data_as_lossy_string().into())
        .collect_vec())
}

/// Remove a chunk from a PNG image
pub fn remove<P>(file: P, chunk_type: ChunkType) -> Result<Png>
where
    P: AsRef<Path>,
{
    let mut png = Png::from_path(file)?;
    png.remove_chunk(&chunk_type)?;
    Ok(png)
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks<P>(file: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let png = Png::from_path(file)?;
    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "{} \"{}\"",
            style(format!("{} ({}):", i + 1, chunk.chunk_type()))
                .yellow()
                .bold(),
            chunk.data_as_lossy_string()
        )
    }
    Ok(())
}
