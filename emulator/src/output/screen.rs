use web_sys::OffscreenCanvasRenderingContext2d;

use crate::dev::ppu::graphic::{SCREEN_HEIGHT, SCREEN_WIDTH, TILES_HEIGHT, TILES_WIDTH};
pub trait ScreenOutput {
    fn put_screen(&mut self, screen: &[u8]);
}
pub struct WebScreenOutput {
    canvas: Option<OffscreenCanvasRenderingContext2d>,
}
impl WebScreenOutput {
    pub fn new() -> Self {
        Self { canvas: None }
    }

    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.canvas = Some(canvas);
    }
}

impl ScreenOutput for WebScreenOutput {
    fn put_screen(&mut self, screen: &[u8]) {
        let u8s = unsafe { js_sys::Uint8ClampedArray::view(screen) };
        let image_data = web_sys::ImageData::new_with_js_u8_clamped_array_and_sh(
            &u8s,
            SCREEN_WIDTH as _,
            SCREEN_HEIGHT as _,
        )
        .unwrap();
        if let Some(canvas) = &self.canvas {
            canvas.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }
}

pub trait TileOutput {
    fn put_tile(&mut self, tile: &[u8]);
}

pub struct WebTileOutput {
    canvas: Option<OffscreenCanvasRenderingContext2d>,
}

impl WebTileOutput {
    pub fn new() -> Self {
        Self { canvas: None }
    }

    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.canvas = Some(canvas);
    }
}

impl TileOutput for WebTileOutput {
    fn put_tile(&mut self, tile: &[u8]) {
        let u8s = unsafe { js_sys::Uint8ClampedArray::view(tile) };
        let image_data = web_sys::ImageData::new_with_js_u8_clamped_array_and_sh(
            &u8s,
            TILES_WIDTH as _,
            TILES_HEIGHT as _,
        )
        .unwrap();
        if let Some(canvas) = &self.canvas {
            canvas.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }
}
