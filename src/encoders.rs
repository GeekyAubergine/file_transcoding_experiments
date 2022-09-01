use crate::{png::PNG, print_data_as_hex, ImageData};

#[derive(Debug)]
pub enum EncoderError {
    InvalidData(String),
    InvalidImageDimensions(String),
}

enum EncodingType {
    PNG,
}

pub trait Encodable {
    fn data_matches_format(&self, data: &[u8]) -> bool;
    fn encode(&self, data: &ImageData) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Result<ImageData, EncoderError>;
}

const PNG_DECODER: PNG = PNG {};

pub struct Encoder {}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {}
    }
    pub fn decode(data: &[u8]) -> Result<ImageData, EncoderError> {
        PNG_DECODER.decode(data)
    }
}

