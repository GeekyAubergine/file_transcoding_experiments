use encoders::Encoder;

mod encoders;
mod png;
mod window;

#[derive(Debug, Clone)]
pub struct PixelData {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
}

impl PixelData {
    fn new(r: u16, g: u16, b: u16, a: u16) -> PixelData {
        PixelData { r, g, b, a }
    }
    fn white() -> PixelData {
        PixelData::new(65535, 65535, 65535, 65535)
    }
    fn black() -> PixelData {
        PixelData::new(0, 0, 0, 65535)
    }
    fn rgba(&self) -> (u16, u16, u16, u16) {
        (self.r, self.g, self.b, self.a)
    }
}

#[derive(Debug, Clone)]
pub struct ImageData {
    width: u32,
    height: u32,
    pixels: Vec<PixelData>,
}

impl ImageData {
    pub fn new(width: u32, height: u32, pixels: Vec<PixelData>) -> ImageData {
        ImageData {
            width,
            height,
            pixels,
        }
    }
}

enum MagicNumbers {
    PNG = 0x89_50_4E_47,
    JPEG = 0xFF_D8_FF,
}

pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    (a as u16) << 8 | b as u16
}

pub const fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | d as u32
}

pub fn u32_to_u8(a: u32) -> (u8, u8, u8, u8) {
    (
        ((a >> 24) & 0xFF) as u8,
        ((a >> 16) & 0xFF) as u8,
        ((a >> 8) & 0xFF) as u8,
        (a & 0xFF) as u8,
    )
}

pub fn print_data_as_hex(data: &[u8], start: usize, end: usize) {
    println!(
        "{}",
        data.iter()
            .skip(start)
            .take(end)
            .map(|x| format!("{:02x} ", x))
            .collect::<String>()
    );
}

pub fn print_data_as_dec(data: &[u8], start: usize, end: usize) {
    println!(
        "{}",
        data.iter()
            .skip(start)
            .take(end)
            .map(|x| format!("{} ", x))
            .collect::<String>()
    );
}

pub fn u32_to_hex(x: u32) -> String {
    format!("0x{:x}", x)
}

pub fn u32_to_dec(x: u32) -> String {
    let data = u32_to_u8(x);
    format!("{} {} {} {}", data.0, data.1, data.2, data.3)
}

pub fn u32_to_bin(x: u32) -> String {
    format!("{:b}", x)
}

// fn magic_numbers_for_data

fn main() {
    match std::fs::read("test_resources/png/basn0g01.png") {
        Ok(data) => {
            // print_data_as_hex(&data, 0, 10);
            let result = Encoder::decode(&data);
            if result.is_ok() {
                println!("Decoded image");
            } else {
                println!("Error decoding image data: {:?}", result.err());
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    match std::fs::read("input2.jpeg") {
        Ok(data) => {
            print_data_as_hex(&data, 0, 10);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
