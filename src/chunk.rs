use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::chunk_type::ChunkType;

/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-layout
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    kind: ChunkType,
    data: Vec<u8>,
    /// https://chrisballance.com/wp-content/uploads/2015/10/CRC-Primer.html
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Might be of interest
        // https://docs.rs/byteorder/latest/byteorder/

        let length = u32::from_be_bytes(
            value
                .get(..4)
                .map(|x| <[u8; 4]>::try_from(x).unwrap())
                .context("No length")?,
        );
        let kind = ChunkType::try_from(
            value
                .get(4..8)
                .map(|x| <[u8; 4]>::try_from(x).unwrap())
                .context("No chunk type")?,
        )?;

        if !kind.is_valid() {
            bail!("Invalid chunk type")
        }

        let data = value.get(8..8 + length as usize).context("No chunk data")?;
        let crc = u32::from_be_bytes(
            value
                .get(8 + length as usize..8 + length as usize + 4)
                .map(|x| <[u8; 4]>::try_from(x).unwrap())
                .context("No crc")?,
        );

        let chunk = Chunk::new(kind, data.into());

        if crc != chunk.crc() {
            bail!("Incorrect crc")
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): \"{}\"",
            self.kind,
            self.crc,
            self.data_as_lossy_string()
        )
    }
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(
            &chunk_type
                .bytes()
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<_>>(),
        );
        Self {
            length: data.len() as _,
            kind: chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.kind
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        std::str::from_utf8(&self.data)
            .map(|s| s.to_owned())
            .map_err(anyhow::Error::msg)
    }

    pub fn data_as_lossy_string(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.data)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .into_iter()
            .chain(self.chunk_type().bytes().into_iter())
            .chain(self.data().iter().copied())
            .chain(self.crc().to_be_bytes().into_iter())
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
