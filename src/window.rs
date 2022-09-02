use pixels::{Error, Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::{DataType, ImageData};

pub struct Window {
    image_data: ImageData,
}

impl Window {
    pub fn new(image_data: ImageData) -> Window {
        Window { image_data }
    }

    pub fn show(&self) -> Result<(), Error> {
        let mut input = WinitInputHelper::new();
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(self.image_data.width, self.image_data.height);
            WindowBuilder::new()
                .with_title("Pixel Viewer")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let mut frame_pixels = Pixels::new(
            self.image_data.width,
            self.image_data.height,
            surface_texture,
        )?;

        match self.image_data.data_type {
            DataType::ARGB_8888 => {
                self.image_data
                    .pixels
                    .iter()
                    .enumerate()
                    .for_each(|(index, pixel)| {
                        frame_pixels.get_frame()[index] = *pixel as u8;
                    });
            }
            _ => {}
        }

        event_loop.run(move |event, _, control_flow| {
            if input.update(&event) {
                if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                    return;
                }
            }
        });
    }
}
