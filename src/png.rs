use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use pixels::wgpu::Color;

use crate::{
    encoders::{Encodable, EncoderError},
    print_data_as_hex, u32_to_bin, u32_to_dec, u32_to_hex, u8_to_u32,
    window::Window,
    ImageData, PixelData,
};

pub struct PNG {}

const PNG_DATA_OFFSET: usize = 8;

const PNG_CHUNK_TYPE_IHDR: u32 = u8_to_u32(73, 72, 68, 82);
const PNG_CHUNK_TYPE_PLTE: u32 = u8_to_u32(80, 76, 84, 69);
const PNG_CHUNK_TYPE_IDAT: u32 = u8_to_u32(73, 68, 65, 84);
const PNG_CHUNK_TYPE_IEND: u32 = u8_to_u32(73, 69, 78, 68);
const PNG_CHUNK_TYPE_TRNS: u32 = u8_to_u32(116, 82, 78, 83);
const PNG_CHUNK_TYPE_CHRM: u32 = u8_to_u32(99, 72, 82, 77);
const PNG_CHUNK_TYPE_ICCP: u32 = u8_to_u32(105, 67, 67, 80);
const PNG_CHUNK_TYPE_GAMA: u32 = u8_to_u32(103, 65, 77, 65);
const PNG_CHUNK_TYPE_SRGB: u32 = u8_to_u32(115, 82, 71, 66);
const PNG_CHUNK_TYPE_SBIT: u32 = u8_to_u32(115, 66, 73, 84);
const PNG_CHUNK_TYPE_TEXT: u32 = u8_to_u32(84, 88, 89, 84);
const PNG_CHUNK_TYPE_ZTXT: u32 = u8_to_u32(122, 88, 89, 84);
const PNG_CHUNK_TYPE_ITXT: u32 = u8_to_u32(105, 84, 89, 84);
const PNG_CHUNK_TYPE_BKGD: u32 = u8_to_u32(98, 75, 71, 68);
const PNG_CHUNK_TYPE_HIST: u32 = u8_to_u32(104, 73, 83, 84);
const PNG_CHUNK_TYPE_PHYS: u32 = u8_to_u32(112, 72, 89, 83);
const PNG_CHUNK_TYPE_SPLT: u32 = u8_to_u32(115, 80, 76, 84);
const PNG_CHUNK_TYPE_TIME: u32 = u8_to_u32(116, 73, 77, 69);

#[derive(Debug)]
enum PNGChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,
    TRNS,
    CHRM,
    GAMA,
    ICCP,
    SRGB,
    SBIT,
    TEXT,
    ZTXT,
    ITXT,
    BKGD,
    HIST,
    PHYS,
    SPLT,
    TIME,
}

impl PNGChunkType {
    fn value(&self) -> Option<u32> {
        match self {
            Self::IHDR => Some(PNG_CHUNK_TYPE_IHDR),
            Self::PLTE => Some(PNG_CHUNK_TYPE_PLTE),
            Self::IDAT => Some(PNG_CHUNK_TYPE_IDAT),
            Self::IEND => Some(PNG_CHUNK_TYPE_IEND),
            Self::TRNS => Some(PNG_CHUNK_TYPE_TRNS),
            Self::CHRM => Some(PNG_CHUNK_TYPE_CHRM),
            Self::GAMA => Some(PNG_CHUNK_TYPE_GAMA),
            Self::ICCP => Some(PNG_CHUNK_TYPE_ICCP),
            Self::SRGB => Some(PNG_CHUNK_TYPE_SRGB),
            Self::SBIT => Some(PNG_CHUNK_TYPE_SBIT),
            Self::TEXT => Some(PNG_CHUNK_TYPE_TEXT),
            Self::ZTXT => Some(PNG_CHUNK_TYPE_ZTXT),
            Self::ITXT => Some(PNG_CHUNK_TYPE_ITXT),
            Self::BKGD => Some(PNG_CHUNK_TYPE_BKGD),
            Self::HIST => Some(PNG_CHUNK_TYPE_HIST),
            Self::PHYS => Some(PNG_CHUNK_TYPE_PHYS),
            Self::SPLT => Some(PNG_CHUNK_TYPE_SPLT),
            Self::TIME => Some(PNG_CHUNK_TYPE_TIME),
        }
    }
    pub fn from_u32(value: u32) -> Option<PNGChunkType> {
        match value {
            PNG_CHUNK_TYPE_IHDR => Some(PNGChunkType::IHDR),
            PNG_CHUNK_TYPE_PLTE => Some(PNGChunkType::PLTE),
            PNG_CHUNK_TYPE_IDAT => Some(PNGChunkType::IDAT),
            PNG_CHUNK_TYPE_IEND => Some(PNGChunkType::IEND),
            PNG_CHUNK_TYPE_TRNS => Some(PNGChunkType::TRNS),
            PNG_CHUNK_TYPE_CHRM => Some(PNGChunkType::CHRM),
            PNG_CHUNK_TYPE_GAMA => Some(PNGChunkType::GAMA),
            PNG_CHUNK_TYPE_ICCP => Some(PNGChunkType::ICCP),
            PNG_CHUNK_TYPE_SRGB => Some(PNGChunkType::SRGB),
            PNG_CHUNK_TYPE_SBIT => Some(PNGChunkType::SBIT),
            PNG_CHUNK_TYPE_TEXT => Some(PNGChunkType::TEXT),
            PNG_CHUNK_TYPE_ZTXT => Some(PNGChunkType::ZTXT),
            PNG_CHUNK_TYPE_ITXT => Some(PNGChunkType::ITXT),
            PNG_CHUNK_TYPE_BKGD => Some(PNGChunkType::BKGD),
            PNG_CHUNK_TYPE_HIST => Some(PNGChunkType::HIST),
            PNG_CHUNK_TYPE_PHYS => Some(PNGChunkType::PHYS),
            PNG_CHUNK_TYPE_SPLT => Some(PNGChunkType::SPLT),
            PNG_CHUNK_TYPE_TIME => Some(PNGChunkType::TIME),
            _ => None,
        }
    }
    pub fn string(&self) -> String {
        match self {
            Self::IHDR => "IHDR".to_string(),
            Self::PLTE => "PLTE".to_string(),
            Self::IDAT => "IDAT".to_string(),
            Self::IEND => "IEND".to_string(),
            Self::TRNS => "TRNS".to_string(),
            Self::CHRM => "CHRM".to_string(),
            Self::GAMA => "GAMA".to_string(),
            Self::ICCP => "ICCP".to_string(),
            Self::SRGB => "SRGB".to_string(),
            Self::SBIT => "SBIT".to_string(),
            Self::TEXT => "TEXT".to_string(),
            Self::ZTXT => "ZTXT".to_string(),
            Self::ITXT => "ITXT".to_string(),
            Self::BKGD => "BKGD".to_string(),
            Self::HIST => "HIST".to_string(),
            Self::PHYS => "PHYS".to_string(),
            Self::SPLT => "SPLT".to_string(),
            Self::TIME => "TIME".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum BitDepth {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}

impl BitDepth {
    fn value(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Four => 4,
            Self::Eight => 8,
            Self::Sixteen => 16,
        }
    }
    fn from_u8(value: u8) -> Result<BitDepth, EncoderError> {
        match value {
            1 => Ok(BitDepth::One),
            2 => Ok(BitDepth::Two),
            4 => Ok(BitDepth::Four),
            8 => Ok(BitDepth::Eight),
            16 => Ok(BitDepth::Sixteen),
            _ => Err(EncoderError::InvalidData(format!(
                "Invalid bit depth: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum ColorType {
    Grayscale,
    Truecolor,
    IndexedColor,
    GrayscaleAlpha,
    TruecolorAlpha,
}

impl ColorType {
    fn value(&self) -> u8 {
        match self {
            Self::Grayscale => 0,
            Self::Truecolor => 2,
            Self::IndexedColor => 3,
            Self::GrayscaleAlpha => 4,
            Self::TruecolorAlpha => 6,
        }
    }
    fn from_u8_and_bit_depth(value: u8, bit_depth: &BitDepth) -> Result<ColorType, EncoderError> {
        match value {
            0 => match bit_depth {
                BitDepth::One => Ok(ColorType::Grayscale),
                BitDepth::Two => Ok(ColorType::Grayscale),
                BitDepth::Four => Ok(ColorType::Grayscale),
                BitDepth::Eight => Ok(ColorType::Grayscale),
                BitDepth::Sixteen => Ok(ColorType::Grayscale),
            },
            2 => match bit_depth {
                BitDepth::Eight => Ok(ColorType::Truecolor),
                BitDepth::Sixteen => Ok(ColorType::Truecolor),
                _ => Err(EncoderError::InvalidData(format!(
                    "Invalid bit depth: {} for color type: {}",
                    bit_depth.value(),
                    value
                ))),
            },
            3 => match bit_depth {
                BitDepth::One => Ok(ColorType::IndexedColor),
                BitDepth::Two => Ok(ColorType::IndexedColor),
                BitDepth::Four => Ok(ColorType::IndexedColor),
                BitDepth::Eight => Ok(ColorType::IndexedColor),
                _ => Err(EncoderError::InvalidData(format!(
                    "Invalid bit depth: {} for color type: {}",
                    bit_depth.value(),
                    value
                ))),
            },
            4 => match bit_depth {
                BitDepth::Eight => Ok(ColorType::GrayscaleAlpha),
                BitDepth::Sixteen => Ok(ColorType::GrayscaleAlpha),
                _ => Err(EncoderError::InvalidData(format!(
                    "Invalid bit depth: {} for color type: {}",
                    bit_depth.value(),
                    value
                ))),
            },
            6 => match bit_depth {
                BitDepth::Eight => Ok(ColorType::TruecolorAlpha),
                BitDepth::Sixteen => Ok(ColorType::TruecolorAlpha),
                _ => Err(EncoderError::InvalidData(format!(
                    "Invalid bit depth: {} for color type: {}",
                    bit_depth.value(),
                    value
                ))),
            },
            _ => Err(EncoderError::InvalidData(format!(
                "Invalid color type: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum CompressionMethod {
    Deflate,
}

impl CompressionMethod {
    fn value(&self) -> u8 {
        match self {
            Self::Deflate => 0,
        }
    }
    fn from_u8(value: u8) -> Result<CompressionMethod, EncoderError> {
        match value {
            0 => Ok(CompressionMethod::Deflate),
            _ => Err(EncoderError::InvalidData(format!(
                "Invalid compression method: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum FilterMethod {
    Adaptive,
}

impl FilterMethod {
    fn value(&self) -> u8 {
        match self {
            Self::Adaptive => 0,
        }
    }
    fn from_u8(value: u8) -> Result<FilterMethod, EncoderError> {
        match value {
            0 => Ok(FilterMethod::Adaptive),
            _ => Err(EncoderError::InvalidData(format!(
                "Invalid filter method: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum InterlaceMethod {
    None,
    Adam7,
}

impl InterlaceMethod {
    fn value(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Adam7 => 1,
        }
    }
    fn from_u8(value: u8) -> Result<InterlaceMethod, EncoderError> {
        match value {
            0 => Ok(InterlaceMethod::None),
            1 => Ok(InterlaceMethod::Adam7),
            _ => Err(EncoderError::InvalidData(format!(
                "Invalid interlace method: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
struct RawPngChunk {
    length: u32,
    chunk_type: PNGChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl RawPngChunk {
    fn new(length: u32, chunk_type: PNGChunkType, data: Vec<u8>, crc: u32) -> RawPngChunk {
        RawPngChunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
}

fn data_to_raw_chunks(data: &[u8]) -> Result<Vec<RawPngChunk>, EncoderError> {
    let mut chunks = Vec::new();
    let mut current_offset = PNG_DATA_OFFSET;
    while current_offset < data.len() {
        if current_offset + 8 > data.len() {
            return Err(EncoderError::InvalidData(
                "PNG chunk is too small".to_string(),
            ));
        }
        let length = u8_to_u32(
            data[current_offset],
            data[current_offset + 1],
            data[current_offset + 2],
            data[current_offset + 3],
        );

        let chunk_type = u8_to_u32(
            data[current_offset + 4],
            data[current_offset + 5],
            data[current_offset + 6],
            data[current_offset + 7],
        );

        let chunk_data = &data[current_offset + 8..current_offset + 8 + length as usize];

        let crc = u8_to_u32(
            data[current_offset + 8 + length as usize],
            data[current_offset + 9 + length as usize],
            data[current_offset + 10 + length as usize],
            data[current_offset + 11 + length as usize],
        );

        if let Some(chunk_type) = PNGChunkType::from_u32(chunk_type) {
            let chunk = RawPngChunk::new(length, chunk_type, chunk_data.to_vec(), crc);
            chunks.push(chunk);
        } else {
            return Err(EncoderError::InvalidData(format!(
                "Invalid PNG chunk type: {} {} {}",
                chunk_type,
                u32_to_hex(chunk_type),
                u32_to_dec(chunk_type),
            )));
        }
        current_offset += 12 + length as usize;
    }
    Ok(chunks)
}

#[derive(Debug, Clone)]
enum PngChunk {
    IHDR {
        width: u32,
        height: u32,
        bit_depth: BitDepth,
        color_type: ColorType,
        compression_method: CompressionMethod,
        filter_method: FilterMethod,
        interlace_method: InterlaceMethod,
    },
    IDAT {
        data: Vec<u8>,
    },
    IEND,
    Other,
}

impl PngChunk {
    fn from_raw_chunk(chunk: &RawPngChunk) -> Result<PngChunk, EncoderError> {
        match chunk.chunk_type {
            PNGChunkType::IHDR => {
                let width = u8_to_u32(chunk.data[0], chunk.data[1], chunk.data[2], chunk.data[3]);
                let height = u8_to_u32(chunk.data[4], chunk.data[5], chunk.data[6], chunk.data[7]);
                let bit_depth = chunk.data[8];
                let color_type = chunk.data[9];
                let compression_method = chunk.data[10];
                let filter_method = chunk.data[11];
                let interlace_method = chunk.data[12];

                // Validation

                if width == 0 || height == 0 {
                    return Err(EncoderError::InvalidImageDimensions(
                        "Width or height is 0".to_string(),
                    ));
                }

                let bit_depth = BitDepth::from_u8(bit_depth)?;
                let color_type = ColorType::from_u8_and_bit_depth(color_type, &bit_depth)?;
                let compression_method = CompressionMethod::from_u8(compression_method)?;
                let filter_method = FilterMethod::from_u8(filter_method)?;
                let interlace_method = InterlaceMethod::from_u8(interlace_method)?;

                Ok(PngChunk::IHDR {
                    width,
                    height,
                    bit_depth,
                    color_type,
                    compression_method,
                    filter_method,
                    interlace_method,
                })
            }
            PNGChunkType::IDAT => {
                return Ok(PngChunk::IDAT {
                    // data: chunk.data.clone(),
                    data: decompress_to_vec_zlib(&chunk.data)?,
                });
                // let data = decompress_to_vec(chunk.data.as_slice());
                // match data {
                //     Ok(data) => Ok(PngChunk::IDAT { data }),
                //     Err(_) => Err(EncoderError::InvalidData("Invalid IDAT chunk".to_string())),
                // }
            }
            PNGChunkType::IEND => Ok(PngChunk::IEND),
            _ => Ok(PngChunk::Other), // _ => panic!("Unknown chunk type"),
        }
    }
    fn as_string(&self) -> String {
        match self {
            Self::IHDR {
                width,
                height,
                bit_depth,
                color_type,
                compression_method,
                filter_method,
                interlace_method,
            } => format!(
                "IHDR: width: {}, height: {}, bit_depth: {}, color_type: {}, compression_method: {}, filter_method: {}, interlace_method: {}",
                width, height, bit_depth.value(), color_type.value(), compression_method.value(), filter_method.value(), interlace_method.value()
            ),
            Self::IDAT { data } => format!("IDAT: data len: {}", data.len()),
            Self::IEND => "IEND".to_string(),
            Self::Other => "Other".to_string(),
        }
    }
}

fn validate_chunks(
    chunks: Result<Vec<PngChunk>, EncoderError>,
) -> Result<Vec<PngChunk>, EncoderError> {
    if let Ok(chunks) = chunks {
        if chunks.is_empty() {
            return Err(EncoderError::InvalidData("No chunks given".to_string()));
        }

        let mut valid_chunks: Vec<PngChunk> = Vec::new();
        let mut has_ihdr = false;
        let mut has_idat = false;
        let mut has_iend = false;
        let mut has_ended_idat = false;

        for (index, chunk) in chunks.iter().enumerate() {
            // All IDAT chunks must be consecutive
            match chunk {
                PngChunk::IDAT { .. } => {
                    if has_ended_idat {
                        return Err(EncoderError::InvalidImageDimensions(
                            "IDAT chunk after end of IDAT block".to_string(),
                        ));
                    }
                }
                _ => {
                    if has_idat {
                        has_ended_idat = true;
                    }
                }
            }

            match (index, chunk) {
                (0, PngChunk::IHDR { .. }) => {
                    valid_chunks.push(chunk.clone());
                    has_ihdr = true;
                }
                (_, PngChunk::IHDR { .. }) => {
                    if has_ihdr {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Multiple IHDR chunks".to_string(),
                        ));
                    }
                    return Err(EncoderError::InvalidData(
                        "IHDR chunk must be first".to_string(),
                    ));
                }
                (index, PngChunk::IEND) => {
                    if index != chunks.len() - 1 {
                        return Err(EncoderError::InvalidData(
                            "IEND chunk must be last".to_string(),
                        ));
                    }
                    valid_chunks.push(PngChunk::IEND);
                    has_iend = true;
                }
                (_, PngChunk::IDAT { .. }) => {
                    has_idat = true;
                    valid_chunks.push(chunk.clone());
                }
                (_, PngChunk::Other) => {
                    valid_chunks.push(chunk.clone());
                }
            }
        }

        if !has_ihdr {
            return Err(EncoderError::InvalidData("No IHDR chunk".to_string()));
        }

        if !has_idat {
            return Err(EncoderError::InvalidData("No IDAT chunks".to_string()));
        }

        if !has_iend {
            return Err(EncoderError::InvalidData("No IEND chunk".to_string()));
        }

        Ok(valid_chunks)
    } else {
        Err(EncoderError::InvalidData("No chunks".to_string()))
    }
}

fn bytes_per_scanline(width: u32, color_type: &ColorType, bit_depth: &BitDepth) -> u32 {
    match color_type {
        ColorType::Grayscale => {
            let bits_per_pixel = bit_depth.value() as u32;
            let bits_per_scanline = width * bits_per_pixel;
            let bytes_per_scanline = bits_per_scanline / 8;
            if bits_per_scanline % 8 != 0 {
                bytes_per_scanline + 1
            } else {
                bytes_per_scanline
            }
        }
        _ => todo!(),
    }
}

struct PngFileMetadata {
    width: u32,
    height: u32,
    bit_depth: BitDepth,
    color_type: ColorType,
    compression_method: CompressionMethod,
    filter_method: FilterMethod,
    interlace_method: InterlaceMethod,
}

struct PngFile {
    metadata: PngFileMetadata,
    data: Vec<u8>,
}

fn scanline_to_pixels_data(scanline: &[u8], bit_depth: &BitDepth) -> Vec<PixelData> {
    match bit_depth {
        BitDepth::One => {
            scanline
                .iter()
                .map(|byte| {
                    let mut pixels = Vec::with_capacity(8);
                    for i in 0..8 {
                        let bit = (byte >> (7 - i)) & 1;
                        if bit == 1 {
                            pixels.push(PixelData::white())
                        } else {
                            pixels.push(PixelData::black())
                        }
                    }
                    pixels
                })
                .flatten()
                .collect()
        }
        _ => todo!(),
    }
}

// fn pixels_data_to_pixles(
//     pixels_data: &[u16],
//     color_type: &ColorType,
//     bit_depth: &BitDepth,
// ) -> Vec<u16> {
//     match color_type {
//         ColorType::Grayscale => match bit_depth {
//             BitDepth::One => {
//                 let mut pixels = Vec::new();
//                 for pixel in pixels_data.iter() {
//                     let pixel = if pixel == &0 { 0 } else { 255 };
//                     pixels.push(pixel);
//                 }
//                 pixels
//             }
//             _ => todo!(),
//         },
//         _ => todo!(),
//     }
// }

impl PngFile {
    fn new(chunks: Vec<PngChunk>) -> Result<Self, EncoderError> {
        let mut metadata: Option<PngFileMetadata> = None;
        let mut data: Vec<u8> = Vec::new();

        for chunk in chunks {
            match chunk {
                PngChunk::IHDR {
                    width,
                    height,
                    bit_depth,
                    color_type,
                    compression_method,
                    filter_method,
                    interlace_method,
                } => {
                    metadata = Some(PngFileMetadata {
                        width,
                        height,
                        bit_depth,
                        color_type,
                        compression_method,
                        filter_method,
                        interlace_method,
                    });
                }
                PngChunk::IDAT { data: idat_data } => {
                    data.extend(idat_data);
                }
                PngChunk::IEND => {}
                PngChunk::Other => {}
            }
        }

        match metadata {
            Some(metadata) => Ok(Self { metadata, data }),
            None => Err(EncoderError::InvalidData("No metadata".to_string())),
        }
    }

    fn width(&self) -> u32 {
        self.metadata.width
    }

    fn height(&self) -> u32 {
        self.metadata.height
    }

    fn color_type(&self) -> &ColorType {
        &self.metadata.color_type
    }

    fn bit_depth(&self) -> &BitDepth {
        &self.metadata.bit_depth
    }

    fn to_image_data(&self) -> Result<ImageData, EncoderError> {
        let mut lines = Vec::new();
        // for line in 0..self.height() {
        //     let mut line = Vec::new();
        //     for pixel in 0..self.width() {
        //         let pixel = 0;
        //         line.push(PixelData::white());
        //     }
        //     lines.push(line);
        // }

        print!("{:?}", self.data);

        // let data = decompress_to_vec_zlib(self.data.as_slice())?;

        print!("{:?}", self.data);
        println!("width: {}, height: {}", self.width(), self.height());
        let scan_length =
            bytes_per_scanline(self.width(), self.color_type(), self.bit_depth()) as usize + 1;
        println!("Scan length: {}", scan_length);
        for line in self.data.chunks(scan_length) {
            println!("Line: {:?}", line.len());
            if line.len() != scan_length {
                return Err(EncoderError::InvalidData(
                    "Invalid scanline length".to_string(),
                ));
            }
            let filter = line[0];
            println!("Filter: {} {}", filter, u32_to_bin(filter as u32));
            let line_data = &line[1..];

            let pixels = scanline_to_pixels_data(line_data, self.bit_depth());

            // let pixels = pixels_data_to_pixles(&pixels_data, self.color_type(), self.bit_depth());

            println!(
                "Line data: {:?}",
                line_data.chunks(self.bit_depth().value() as usize)
            );

            // println!("Pixels data: {:?}", pixels_data);

            println!("Pixels: {:?} {}", pixels, pixels.len());
            lines.push(pixels);
        }

        Ok(ImageData {
            width: self.width(),
            height: self.height(),
            pixels: lines.into_iter().flatten().collect(),
        })
    }
}

impl Encodable for PNG {
    fn data_matches_format(&self, data: &[u8]) -> bool {
        data.len() > 8 && data[0..8] == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
    }

    fn encode(&self, data: &ImageData) -> Vec<u8> {
        todo!()
    }

    fn decode(&self, data: &[u8]) -> Result<ImageData, EncoderError> {
        print_data_as_hex(data, PNG_DATA_OFFSET, 64);

        let raw_chunnks = data_to_raw_chunks(data)?;

        let chunks: Result<Vec<PngChunk>, EncoderError> = raw_chunnks
            .iter()
            .map(|chunk| PngChunk::from_raw_chunk(chunk))
            .collect();

        let chunks = validate_chunks(chunks)?;

        for chunk in chunks.clone() {
            println!("{}", chunk.as_string());
        }

        let png_file = PngFile::new(chunks)?;

        let image_data = png_file.to_image_data()?;

        let window = Window::new(image_data);

        window.show().unwrap();

        todo!()
    }
}

#[cfg(test)]
mod test {
    // ---- chunk validation

    use crate::png::bytes_per_scanline;
    use crate::png::BitDepth;
    use crate::png::ColorType;
    use crate::png::CompressionMethod;
    use crate::png::FilterMethod;
    use crate::png::InterlaceMethod;

    use super::validate_chunks;
    use super::PngChunk;

    #[test]
    fn test_validate_chunks_no_chunks() {
        let chunks = vec![];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(
            chunks.err().unwrap().message(),
            "No chunks given".to_string()
        );
    }

    #[test]
    fn test_validate_chunks_no_ihdr() {
        let chunks = vec![PngChunk::IDAT { data: Vec::new() }];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(chunks.err().unwrap().message(), "No IHDR chunk".to_string());
    }

    #[test]
    fn test_validate_chunks_no_idat() {
        let chunks = vec![PngChunk::IHDR {
            width: 1,
            height: 1,
            bit_depth: BitDepth::One,
            color_type: ColorType::Grayscale,
            compression_method: CompressionMethod::Deflate,
            filter_method: FilterMethod::Adaptive,
            interlace_method: InterlaceMethod::None,
        }];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(
            chunks.err().unwrap().message(),
            "No IDAT chunks".to_string()
        );
    }

    #[test]
    fn test_validate_chunks_no_iend() {
        let chunks = vec![
            PngChunk::IHDR {
                width: 1,
                height: 1,
                bit_depth: BitDepth::One,
                color_type: ColorType::Grayscale,
                compression_method: CompressionMethod::Deflate,
                filter_method: FilterMethod::Adaptive,
                interlace_method: InterlaceMethod::None,
            },
            PngChunk::IDAT { data: Vec::new() },
        ];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(chunks.err().unwrap().message(), "No IEND chunk".to_string());
    }

    #[test]
    fn test_validate_chunks_multiple_ihdr() {
        let chunks = vec![
            PngChunk::IHDR {
                width: 1,
                height: 1,
                bit_depth: BitDepth::One,
                color_type: ColorType::Grayscale,
                compression_method: CompressionMethod::Deflate,
                filter_method: FilterMethod::Adaptive,
                interlace_method: InterlaceMethod::None,
            },
            PngChunk::IHDR {
                width: 1,
                height: 1,
                bit_depth: BitDepth::One,
                color_type: ColorType::Grayscale,
                compression_method: CompressionMethod::Deflate,
                filter_method: FilterMethod::Adaptive,
                interlace_method: InterlaceMethod::None,
            },
        ];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(
            chunks.err().unwrap().message(),
            "Multiple IHDR chunks".to_string()
        );
    }

    #[test]
    fn test_validate_chunks_multiple_separated_idat() {
        let chunks = vec![
            PngChunk::IHDR {
                width: 1,
                height: 1,
                bit_depth: BitDepth::One,
                color_type: ColorType::Grayscale,
                compression_method: CompressionMethod::Deflate,
                filter_method: FilterMethod::Adaptive,
                interlace_method: InterlaceMethod::None,
            },
            PngChunk::IDAT { data: Vec::new() },
            PngChunk::Other,
            PngChunk::IDAT { data: Vec::new() },
        ];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(
            chunks.err().unwrap().message(),
            "IDAT chunk after end of IDAT block".to_string()
        );
    }

    #[test]
    fn test_validate_chunks_multiple_iend() {
        let chunks = vec![
            PngChunk::IHDR {
                width: 1,
                height: 1,
                bit_depth: BitDepth::One,
                color_type: ColorType::Grayscale,
                compression_method: CompressionMethod::Deflate,
                filter_method: FilterMethod::Adaptive,
                interlace_method: InterlaceMethod::None,
            },
            PngChunk::IDAT { data: Vec::new() },
            PngChunk::IEND,
            PngChunk::IEND,
        ];
        let chunks = validate_chunks(Ok(chunks));
        assert!(chunks.is_err());
        assert_eq!(
            chunks.err().unwrap().message(),
            "IEND chunk must be last".to_string()
        );
    }

    #[test]
    fn test_bytes_per_scanline() {
        assert_eq!(
            bytes_per_scanline(32, &ColorType::Grayscale, &BitDepth::One),
            4
        );
        assert_eq!(
            bytes_per_scanline(33, &ColorType::Grayscale, &BitDepth::One),
            5
        );
    }
}
