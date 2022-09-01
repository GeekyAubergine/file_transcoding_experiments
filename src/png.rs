use crate::{
    encoders::{Encodable, EncoderError},
    print_data_as_hex, u32_to_dec, u32_to_hex, u8_to_u32,
    window::Window,
    DataType, ImageData,
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
enum PNG_Chunk_Type {
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

impl PNG_Chunk_Type {
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
    pub fn from_u32(value: u32) -> Option<PNG_Chunk_Type> {
        match value {
            PNG_CHUNK_TYPE_IHDR => Some(PNG_Chunk_Type::IHDR),
            PNG_CHUNK_TYPE_PLTE => Some(PNG_Chunk_Type::PLTE),
            PNG_CHUNK_TYPE_IDAT => Some(PNG_Chunk_Type::IDAT),
            PNG_CHUNK_TYPE_IEND => Some(PNG_Chunk_Type::IEND),
            PNG_CHUNK_TYPE_TRNS => Some(PNG_Chunk_Type::TRNS),
            PNG_CHUNK_TYPE_CHRM => Some(PNG_Chunk_Type::CHRM),
            PNG_CHUNK_TYPE_GAMA => Some(PNG_Chunk_Type::GAMA),
            PNG_CHUNK_TYPE_ICCP => Some(PNG_Chunk_Type::ICCP),
            PNG_CHUNK_TYPE_SRGB => Some(PNG_Chunk_Type::SRGB),
            PNG_CHUNK_TYPE_SBIT => Some(PNG_Chunk_Type::SBIT),
            PNG_CHUNK_TYPE_TEXT => Some(PNG_Chunk_Type::TEXT),
            PNG_CHUNK_TYPE_ZTXT => Some(PNG_Chunk_Type::ZTXT),
            PNG_CHUNK_TYPE_ITXT => Some(PNG_Chunk_Type::ITXT),
            PNG_CHUNK_TYPE_BKGD => Some(PNG_Chunk_Type::BKGD),
            PNG_CHUNK_TYPE_HIST => Some(PNG_Chunk_Type::HIST),
            PNG_CHUNK_TYPE_PHYS => Some(PNG_Chunk_Type::PHYS),
            PNG_CHUNK_TYPE_SPLT => Some(PNG_Chunk_Type::SPLT),
            PNG_CHUNK_TYPE_TIME => Some(PNG_Chunk_Type::TIME),
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

#[derive(Debug)]
struct RawPngChunk {
    length: u32,
    chunk_type: PNG_Chunk_Type,
    data: Vec<u8>,
    crc: u32,
}

impl RawPngChunk {
    fn new(length: u32, chunk_type: PNG_Chunk_Type, data: Vec<u8>, crc: u32) -> RawPngChunk {
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

        if let Some(chunk_type) = PNG_Chunk_Type::from_u32(chunk_type) {
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

enum PngChunk<'a> {
    IHDR {
        width: u32,
        height: u32,
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    },
    IDAT {
        data: &'a Vec<u8>,
    },
    IEND,
    Other,
}

impl<'a> PngChunk<'a> {
    fn from_raw_chunk(chunk: &RawPngChunk) -> Result<PngChunk, EncoderError> {
        match chunk.chunk_type {
            PNG_Chunk_Type::IHDR => {
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

                if color_type == 0 {
                    if bit_depth != 1
                        && bit_depth != 2
                        && bit_depth != 4
                        && bit_depth != 8
                        && bit_depth != 16
                    {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Invalid bit depth for color type 0".to_string(),
                        ));
                    }
                } else if color_type == 2 {
                    if bit_depth != 8 && bit_depth != 16 {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Invalid bit depth for color type 2".to_string(),
                        ));
                    }
                } else if color_type == 3 {
                    if bit_depth != 1 && bit_depth != 2 && bit_depth != 4 && bit_depth != 8 {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Invalid bit depth for color type 3".to_string(),
                        ));
                    }
                } else if color_type == 4 {
                    if bit_depth != 8 && bit_depth != 16 {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Invalid bit depth for color type 4".to_string(),
                        ));
                    }
                } else if color_type == 6 {
                    if bit_depth != 8 && bit_depth != 16 {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Invalid bit depth for color type 6".to_string(),
                        ));
                    }
                } else {
                    return Err(EncoderError::InvalidImageDimensions(
                        "Invalid color type".to_string(),
                    ));
                }

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
            PNG_Chunk_Type::IDAT => Ok(PngChunk::IDAT { data: &chunk.data }),
            PNG_Chunk_Type::IEND => Ok(PngChunk::IEND),
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
                width, height, bit_depth, color_type, compression_method, filter_method, interlace_method
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
        let mut valid_chunks = Vec::new();
        let mut has_ihdr = false;
        let mut has_idat = false;
        let mut has_started_idat = false;
        let mut has_ended_idat = false;
        let mut has_iend = false;
        for chunk in chunks {
            // All IDAT chunks must be consecutive
            match chunk {
                PngChunk::IDAT { .. } => {}
                _ => {
                    if has_ended_idat {
                        return Err(EncoderError::InvalidImageDimensions(
                            "Chunk after end of IDAT block".to_string(),
                        ));
                    }
                }
            }

            match chunk {
                PngChunk::IHDR { .. } => {
                    if has_ihdr {
                        return Err(EncoderError::InvalidData(
                            "Multiple IHDR chunks".to_string(),
                        ));
                    }
                    has_ihdr = true;
                    valid_chunks.push(chunk);
                }
                PngChunk::IDAT { .. } => {
                    if !has_ihdr {
                        return Err(EncoderError::InvalidData(
                            "IDAT chunk before IHDR".to_string(),
                        ));
                    }
                    has_started_idat = true;
                    has_idat = true;
                    valid_chunks.push(chunk);
                }
                PngChunk::IEND => {
                    if has_iend {
                        return Err(EncoderError::InvalidData(
                            "Multiple IEND chunks".to_string(),
                        ));
                    }
                    has_iend = true;
                    valid_chunks.push(chunk);
                }
                PngChunk::Other => {
                    if !has_ihdr {
                        return Err(EncoderError::InvalidData("Chunk before IHDR".to_string()));
                    }
                    if has_iend {
                        return Err(EncoderError::InvalidData("Chunk after IEND".to_string()));
                    }
                }
            }
        }
        if !has_ihdr {
            return Err(EncoderError::InvalidData("No IHDR chunk".to_string()));
        }
        if !has_idat {
            return Err(EncoderError::InvalidData("No IDAT chunk".to_string()));
        }
        if !has_iend {
            return Err(EncoderError::InvalidData("No IEND chunk".to_string()));
        }
        Ok(valid_chunks)
    } else {
        chunks
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

        for chunk in chunks {
            println!("{}", chunk.as_string());
        }

        let window = Window::new(ImageData {
            width: 200,
            height: 300,
            pixels: Vec::new(),
            data_type: DataType::ARGB_8888,
        });

        window.show().unwrap();

        todo!()
    }
}
