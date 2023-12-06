pub mod yay0;
pub mod yaz0;

mod utils;

use thiserror::Error;
use yay0::{compress_yay0, decompress_yay0};
use yaz0::{compress_yaz0, decompress_yaz0};

#[derive(Copy, Clone, Debug, Error, PartialEq, Eq, Hash)]
pub enum Crunch64Error {
    #[error("Failed to open file")]
    OpenFile,
    #[error("Failed to read file")]
    ReadFile,
    #[error("Failed to write file")]
    WriteFile,
    #[error("File does not begin with Yay0 bytes")]
    InvalidYay0Header,
    #[error("File does not begin with Yaz0 bytes")]
    InvalidYaz0Header,
    #[error("File does not begin with Mio0 bytes")]
    InvalidMio0Header,
    #[error("Unsupported compression type")]
    UnsupportedCompressionType,
    #[error("Unaligned read")]
    UnalignedRead,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompressionType {
    Yay0,
    Yaz0,
    Mio0,
}

impl CompressionType {
    pub fn decompress(self: CompressionType, bytes: &[u8]) -> Result<Box<[u8]>, Crunch64Error> {
        match self {
            CompressionType::Yay0 => decompress_yay0(bytes),
            CompressionType::Yaz0 => decompress_yaz0(bytes),
            _ => Err(Crunch64Error::UnsupportedCompressionType),
        }
    }

    pub fn compress(self: CompressionType, bytes: &[u8]) -> Result<Box<[u8]>, Crunch64Error> {
        match self {
            CompressionType::Yay0 => compress_yay0(bytes),
            CompressionType::Yaz0 => compress_yaz0(bytes),
            _ => Err(Crunch64Error::UnsupportedCompressionType),
        }
    }
}
