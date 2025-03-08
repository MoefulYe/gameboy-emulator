use web_sys::OffscreenCanvasRenderingContext2d;

use crate::{
    dev::ppu::graphic::{
        decode_tiles, RGBAPalette, RawTileMatrix, ScreenBitmap, TilesBitmap, NO_COLOR,
        SCREEN_HEIGHT, SCREEN_WIDTH, TILES_HEIGHT, TILES_WIDTH,
    },
    utils::bytes::as_bytes,
};
pub trait ScreenOutput {
    fn put_screen(&mut self, idx: u8);
    fn buffer(&mut self, idx: u8) -> &mut ScreenBitmap;
}
pub struct WebScreenOutput {
    canvas: Option<OffscreenCanvasRenderingContext2d>,
    screen_buffers: [Box<ScreenBitmap>; 2],
}
impl WebScreenOutput {
    pub fn new() -> Self {
        Self {
            canvas: None,
            screen_buffers: [
                Box::new([[NO_COLOR; 160]; 144]),
                Box::new([[NO_COLOR; 160]; 144]),
            ],
        }
    }

    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.canvas = Some(canvas);
    }
}

impl ScreenOutput for WebScreenOutput {
    fn put_screen(&mut self, idx: u8) {
        let u8s = unsafe {
            js_sys::Uint8ClampedArray::view(as_bytes::<ScreenBitmap>(
                self.screen_buffers.get_unchecked(idx as usize).as_ref(),
            ))
        };
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

    fn buffer(&mut self, idx: u8) -> &mut ScreenBitmap {
        unsafe { self.screen_buffers.get_unchecked_mut(idx as usize).as_mut() }
    }
}

pub trait TileOutput {
    fn put_tile(&mut self, tiles: &RawTileMatrix, palette: &RGBAPalette);
}

pub struct WebTileOutput {
    canvas: Option<OffscreenCanvasRenderingContext2d>,
    tiles_buffer: Box<TilesBitmap>,
}

impl WebTileOutput {
    pub fn new() -> Self {
        Self {
            canvas: None,
            tiles_buffer: Box::new([[NO_COLOR; 128]; 192]),
        }
    }

    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.canvas = Some(canvas);
    }
}

impl TileOutput for WebTileOutput {
    fn put_tile(&mut self, tiles: &RawTileMatrix, palette: &RGBAPalette) {
        decode_tiles(tiles, palette, &mut self.tiles_buffer);
        let u8s = unsafe {
            js_sys::Uint8ClampedArray::view(as_bytes::<TilesBitmap>(self.tiles_buffer.as_ref()))
        };
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
