use crate::{types::Word, utils::bits::BitMap};

pub const TILES_WIDTH: usize = 128;
pub const TILES_HEIGHT: usize = 192;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub type RGBA = u32;
pub type Pixel = RGBA;

pub type RGBAPalette = [RGBA; 4];

pub type RawTile = [[u8; 2]; 8];
pub type RawTileMatrix = [[RawTile; 16]; 24];
pub type RawTiles = [RawTile; 16 * 24];
pub type TilesBitmap = [[Pixel; TILES_WIDTH]; TILES_HEIGHT];
pub type ScreenBitmap = [[Pixel; SCREEN_WIDTH]; SCREEN_HEIGHT];
pub const PPU_LINES_PER_FRAME: u8 = 154;
pub const PPU_CYCLES_PER_LINE: u32 = 456;
pub const PPU_YRES: Word = 144;
pub const PPU_XRES: Word = 160;

pub struct TilePos {
    pub x: u8,
    pub y: u8,
}

impl TilePos {
    pub fn from_point(x: u8, y: u8) -> Self {
        Self { x: x / 8, y: y / 8 }
    }
    pub fn to_idx(self) -> usize {
        (self.y as usize) * 32 + (self.x as usize)
    }
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> RGBA {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    let a = a as u32;
    r | g << 8 | b << 16 | a << 24
}

pub fn decode_tiles(tiles: &RawTileMatrix, palette: &RGBAPalette, buffer: &mut TilesBitmap) {
    // 块间行号
    for (i, tiles) in tiles.iter().enumerate() {
        // 块内行号
        for x in 0..8 {
            // 块间列号
            for (j, tile) in tiles.iter().enumerate() {
                let [lo, hi] = tile[x];
                // 行内列号
                for y in 0..8 {
                    let bit_pos = (7 - y) as Word;
                    let lo = lo.at(bit_pos);
                    let hi = hi.at(bit_pos);
                    let idx = hi << 1 | lo;
                    let color = palette[idx as usize];
                    buffer[i * 8 + x][j * 8 + y] = color;
                }
            }
        }
    }
}

pub const NO_COLOR: RGBA = rgba(0, 0, 0, 0);
const WHITE: RGBA = rgba(153, 161, 120, 255);
const GRAY: RGBA = rgba(87, 93, 67, 255);
const DEEP_GRAY: RGBA = rgba(42, 46, 32, 255);
const BLACK: RGBA = rgba(10, 10, 2, 255);
// const WHITE: RGBA = rgba(0xff, 0xff, 0xff, 0xff);
// const DEEP_GRAY: RGBA = rgba(0xaa, 0xaa, 0xaa, 0xff);
// const GRAY: RGBA = rgba(0x44, 0x44, 0x44, 0xff);
// const BLACK: RGBA = rgba(0, 0, 0, 0);
pub const PALETTE: RGBAPalette = [WHITE, GRAY, DEEP_GRAY, BLACK];
