use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::bail;

/// Section `3.3` of http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType {
    buf: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self { buf: value })
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self {
                buf: s.as_bytes().try_into()?,
            })
        } else {
            bail!("'{}' is not valid because of non-alphabetic chars", s)
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(&self.buf).map_err(|_| fmt::Error)?)
    }
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.buf
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
        // && (!self.is_critical() || !self.is_safe_to_copy())
    }
    pub fn is_critical(&self) -> bool {
        self.buf[0] >> 5 & 1 == 0
    }
    pub fn is_public(&self) -> bool {
        self.buf[1] >> 5 & 1 == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.buf[2] >> 5 & 1 == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.buf[3] >> 5 & 1 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        // let chunk = ChunkType::from_str("RuSt").unwrap(); // shouldn't this fail? Critical chunk must be unsafe to copy
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
