use crate::{types::Word, utils::bits::BitMap};

pub type RGBA = [u8; 4];
pub type Pixel = RGBA;

pub type Palette = [RGBA; 4];

pub type RawTile = [[u8; 2]; 8];
pub type RawTiles = [[RawTile; 16]; 24];
pub const TILES_WIDTH: usize = 128;
pub const TILES_HEIGHT: usize = 192;
pub type TilesBitmap = [[Pixel; TILES_WIDTH]; TILES_HEIGHT];

pub fn decode_tiles(tiles: &[u8], palette: &Palette, buffer: &mut TilesBitmap) {
    let tiles: &RawTiles = unsafe { &*(tiles.as_ptr() as *const _) };
    _decode(tiles, palette, buffer);
}

fn _decode(tiles: &RawTiles, palette: &Palette, buffer: &mut TilesBitmap) {
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
