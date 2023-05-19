use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode<P>(file: P, message: &str, chunk_type: ChunkType) -> Result<Png>
where
    P: AsRef<Path>,
{
    let mut png = Png::from_path(file)?;
    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().into()));
    Ok(png)
}

pub fn decode<P>(file: P, chunk_type: ChunkType) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut png = Png::from_path(file)?;
    Ok(png
        .chunks_by_type(&chunk_type)
        .into_iter()
        .map(|chunk| chunk.to_string())
        .collect_vec())
}

pub fn remove<P>(file: P, chunk_type: ChunkType) -> Result<()>
where
    P: AsRef<Path>,
{
    todo!()
}

pub fn print_chunks<P>(file: P) -> Result<()>
where
    P: AsRef<Path>,
{
    todo!()
}
