use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use smallvec::SmallVec;

use crate::{
    dev::Reset,
    types::{Addr, Word},
    utils::bits::BitMap,
};

use super::{
    graphic::TilePos,
    oam::{
        Object,
        ObjectPaletteSelect::{OBP0, OBP1},
        ObjectPixel,
    },
    BGWPixel, PPU,
};
#[derive(Serialize, Deserialize)]
pub(super) enum FetchState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub(super) enum FetchType {
    FetchWindow,
    FetchBackground,
}

#[derive(Serialize, Deserialize)]
pub(super) struct Fetcher {
    pub fetch_type: FetchType,
    pub window_line: Word,
    pub state: FetchState,
    pub fetch_x: Word,
    pub tile_x_begin: i16,
    pub bgw_fetched_data: [Word; 2],
    pub push_x: Word,
    /// 块号, 块内行号
    pub bgw_data_idx: (Addr, Word),

    pub row_intersect_objects: SmallVec<[Object; 10]>,
    pub objects_to_draw: SmallVec<[Object; 3]>,
    pub objects_fetched_data: [[Word; 2]; 3],
}

impl Reset for Fetcher {
    fn reset(&mut self) {
        self.fetch_type = FetchType::FetchWindow;
        self.window_line = 0;
        self.state = FetchState::Tile;
        self.fetch_x = 0;
        self.tile_x_begin = 0;
        self.bgw_fetched_data = [0, 0];
        self.push_x = 0;
        self.bgw_data_idx = (0, 0);
        self.row_intersect_objects.clear();
        self.objects_to_draw.clear();
        self.objects_fetched_data.fill([0, 0]);
    }
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            fetch_type: FetchType::FetchWindow,
            window_line: 0,
            state: FetchState::Tile,
            fetch_x: 0,
            tile_x_begin: 0,
            bgw_fetched_data: [0, 0],
            push_x: 0,
            bgw_data_idx: (0, 0),
            row_intersect_objects: SmallVec::new(),
            objects_to_draw: SmallVec::new(),
            objects_fetched_data: Default::default(),
        }
    }
}

impl PPU {
    pub(super) fn fetcher_update(&mut self) {
        match self.fetcher.state {
            FetchState::Tile => self.get_tile(),
            FetchState::Data0 => self.get_data0(),
            FetchState::Data1 => self.get_data1(),
            FetchState::Idle => self.idle(),
            FetchState::Push => self.push_pixel(),
        };
    }

    fn get_tile(&mut self) {
        if self.lcdc.window_bg_enabled() {
            match self.fetcher.fetch_type {
                super::FetchType::FetchWindow => self.get_window_tile(),
                super::FetchType::FetchBackground => self.get_background_tile(),
            }
        } else {
            self.fetcher.tile_x_begin = self.fetcher.fetch_x as i16;
        }
        if self.lcdc.obj_enabled() {
            self.get_object_tile()
        }
        self.fetcher.state = FetchState::Data0;
        self.fetcher.fetch_x += 8;
    }
    fn get_data0(&mut self) {
        if self.lcdc.window_bg_enabled() {
            let (i, j) = self.fetcher.bgw_data_idx;
            unsafe {
                *self.fetcher.bgw_fetched_data.get_unchecked_mut(0) = *self
                    .vram
                    .tiles_area()
                    .get_unchecked(i as usize)
                    .get_unchecked(j as usize)
                    .get_unchecked(0)
            };
        }
        if self.lcdc.obj_enabled() {
            let obj_height = self.lcdc.obj_height();
            for (i, obj) in self.fetcher.objects_to_draw.iter().enumerate() {
                let ty = self.ly + 16 - obj.y;
                let ty = if obj.y_flip() {
                    obj_height.wrapping_sub(1).wrapping_sub(ty)
                } else {
                    ty
                };
                let tile_idx = obj.tile_idx;
                let tile_idx = if obj_height == 16 {
                    tile_idx.clear_at(0)
                } else {
                    tile_idx
                };
                unsafe {
                    *self
                        .fetcher
                        .objects_fetched_data
                        .get_unchecked_mut(i as usize)
                        .get_unchecked_mut(0) = *self
                        .vram
                        .tiles_area()
                        .get_unchecked(tile_idx as usize)
                        .get_unchecked(ty as usize)
                        .get_unchecked(0)
                };
            }
        }
        self.fetcher.state = FetchState::Data1;
    }
    fn get_data1(&mut self) {
        if self.lcdc.window_bg_enabled() {
            let (i, j) = self.fetcher.bgw_data_idx;
            unsafe {
                *self.fetcher.bgw_fetched_data.get_unchecked_mut(1) = *self
                    .vram
                    .tiles_area()
                    .get_unchecked(i as usize)
                    .get_unchecked(j as usize)
                    .get_unchecked(1)
            };
        }
        if self.lcdc.obj_enabled() {
            let obj_height = self.lcdc.obj_height();
            for (i, obj) in self.fetcher.objects_to_draw.iter().enumerate() {
                let ty = self.ly + 16 - obj.y;
                let ty = if obj.y_flip() {
                    obj_height.wrapping_sub(1).wrapping_sub(ty)
                } else {
                    ty
                };
                let tile_idx = obj.tile_idx;
                let tile_idx = if obj_height == 16 {
                    tile_idx.clear_at(0)
                } else {
                    tile_idx
                };
                unsafe {
                    *self
                        .fetcher
                        .objects_fetched_data
                        .get_unchecked_mut(i as usize)
                        .get_unchecked_mut(1) = *self
                        .vram
                        .tiles_area()
                        .get_unchecked(tile_idx as usize)
                        .get_unchecked(ty as usize)
                        .get_unchecked(1)
                };
            }
        }
        self.fetcher.state = FetchState::Idle;
    }
    fn idle(&mut self) {
        self.fetcher.state = FetchState::Push;
    }

    fn push_pixel(&mut self) {
        let mut pushed = false;
        if self.bgw_queue.len() < 8 {
            let push_begin = self.fetcher.push_x;
            self.push_bgw_pixel();
            let push_end = self.fetcher.push_x;
            self.push_object_pixels(push_begin, push_end);
            pushed = true
        }
        if pushed {
            self.fetcher.state = FetchState::Tile;
        }
    }

    fn push_bgw_pixel(&mut self) {
        let [lo, hi] = self.fetcher.bgw_fetched_data;
        for i in 0u8..8u8 {
            if self.fetcher.tile_x_begin + (i as i16) < 0 {
                continue;
            }
            if self.fetcher.fetch_type == FetchType::FetchBackground
                && self.window_visible()
                && (self.fetcher.push_x + 7) >= self.wx
                && self.ly >= self.wy
            {
                self.fetcher.fetch_type = FetchType::FetchWindow;
                self.fetcher.fetch_x = self.fetcher.push_x;
                break;
            }
            let pixel = if self.lcdc.window_bg_enabled() {
                let lo = lo.at(7 - i);
                let hi = hi.at(7 - i);
                let color = hi << 1 | lo;
                BGWPixel {
                    color,
                    palette: self.bgp,
                }
            } else {
                Default::default()
            };
            self.bgw_queue.push_back(pixel);
            self.fetcher.push_x += 1;
        }
    }

    fn push_object_pixels(&mut self, push_begin: Word, push_end: Word) {
        for i in push_begin..push_end {
            let mut pixel = ObjectPixel::new();
            if self.lcdc.enabled() {
                for (j, obj) in self.fetcher.objects_to_draw.iter().enumerate() {
                    let x = (obj.x as i16) - 8;
                    let offset = (i as i16) - x;
                    if offset < 0 || offset > 7 {
                        continue;
                    }
                    let [b1, b2] = *unsafe { self.fetcher.objects_fetched_data.get_unchecked(j) };
                    let b = if obj.x_flip() {
                        offset as Word
                    } else {
                        7 - (offset as Word)
                    };
                    let lo = b1.at(b);
                    let hi = b2.at(b);
                    let color = hi << 1 | lo;
                    if color == 0 {
                        continue;
                    }
                    let palette = match obj.palette() {
                        OBP0 => self.obp0,
                        OBP1 => self.obp1,
                    };
                    let bg_priority = obj.priority();
                    pixel = ObjectPixel {
                        color,
                        palette,
                        bg_priority,
                    };
                    break;
                }
            }
            self.obj_queue.push_back(pixel);
        }
    }

    fn get_background_tile(&mut self) {
        let y = self.ly + self.scy;
        let x = self.fetcher.fetch_x + self.scx;
        let tile_idx = TilePos::from_point(x, y).to_idx();
        let &data_idx = unsafe {
            self.vram
                .map_area(self.lcdc.bg_map_area())
                .get_unchecked(tile_idx)
        };
        self.fetcher.bgw_data_idx = (self.lcdc.window_bg_data_area().addr(data_idx), y % 8);
        let tile_x = (self.fetcher.fetch_x as i16) + (self.scx as i16);
        self.fetcher.tile_x_begin = (tile_x / 8) * 8 - self.scx as i16;
    }

    fn get_window_tile(&mut self) {
        let y = self.fetcher.window_line;
        let x = self.fetcher.fetch_x + 7 - self.wx;
        let tile_idx = TilePos::from_point(x, y).to_idx();
        let &data_idx = unsafe {
            self.vram
                .map_area(self.lcdc.window_map_area())
                .get_unchecked(tile_idx)
        };
        self.fetcher.bgw_data_idx = (self.lcdc.window_bg_data_area().addr(data_idx), y % 8);
        let tile_x = (self.fetcher.fetch_x as i16) - (self.wx as i16 - 7);
        self.fetcher.tile_x_begin = (tile_x / 8) * 8 + (self.wx as i16) - 7;
    }

    fn get_object_tile(&mut self) {
        let fetcher = &mut self.fetcher;
        fetcher.objects_to_draw.clear();
        for obj in &fetcher.row_intersect_objects {
            let x = (obj.x as i16) - 8;
            if (x >= fetcher.tile_x_begin && x < fetcher.tile_x_begin + 8)
                || (x + 7 >= fetcher.tile_x_begin && x + 7 < fetcher.tile_x_begin + 8)
            {
                fetcher.objects_to_draw.push(*obj);
                if fetcher.objects_to_draw.len() >= 3 {
                    break;
                }
            }
        }
    }
}

impl PPU {
    pub(super) fn fetcher_oam_scan(&mut self) {
        self.fetcher.row_intersect_objects.clear();
        let obj_height = self.lcdc.obj_height();
        for obj in self.oam.as_objs() {
            if self.fetcher.row_intersect_objects.len() >= 10 {
                break;
            }
            // len < 10
            if obj.y <= self.ly + 16 && obj.y + obj_height > self.ly + 16 {
                let pos = self
                    .fetcher
                    .row_intersect_objects
                    .iter()
                    .enumerate()
                    .find(|(_, other)| other.x > obj.x)
                    .map(|(idx, _)| idx)
                    .unwrap_or(self.fetcher.row_intersect_objects.len());
                self.fetcher.row_intersect_objects.insert(pos, obj.clone());
            }
        }
    }
}
