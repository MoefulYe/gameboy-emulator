use crate::{
    types::{Addr, Word},
    utils::bits::BitMap,
};

use super::{graphic::TilePos, PPU};
pub(super) enum FetchState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}

#[derive(PartialEq)]
pub(super) enum FetchType {
    FetchWindow,
    FetchBackground,
}

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
        if self.lcdc.window_bg_enable() {
            match self.fetcher.fetch_type {
                super::FetchType::FetchWindow => self.get_window_tile(),
                super::FetchType::FetchBackground => self.get_background_tile(),
            }
        }
        self.fetcher.state = FetchState::Data0;
        self.fetcher.fetch_x += 8;
    }
    fn get_data0(&mut self) {
        if self.lcdc.window_bg_enable() {
            let (i, j) = self.fetcher.bgw_data_idx;
            self.fetcher.bgw_fetched_data[0] = *unsafe {
                self.vram
                    .flatten_tiles_area()
                    .get_unchecked(i as usize)
                    .get_unchecked(j as usize)
                    .get_unchecked(0)
            };
        }
        self.fetcher.state = FetchState::Data1;
    }
    fn get_data1(&mut self) {
        if self.lcdc.window_bg_enable() {
            let (i, j) = self.fetcher.bgw_data_idx;
            self.fetcher.bgw_fetched_data[1] = *unsafe {
                self.vram
                    .flatten_tiles_area()
                    .get_unchecked(i as usize)
                    .get_unchecked(j as usize)
                    .get_unchecked(1)
            };
        }
        self.fetcher.state = FetchState::Idle;
    }
    fn idle(&mut self) {
        self.fetcher.state = FetchState::Push;
    }

    fn push_pixel(&mut self) {
        let mut pushed = false;
        if self.bgw_queue.len() < 8 {
            self.push_bgw_pixel();
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
            let pixel = if self.lcdc.window_bg_enable() {
                let lo = lo.at(7 - i);
                let hi = hi.at(7 - i);
                let color = hi << 1 | lo;
                self.bgp.apply(color)
            } else {
                0
            };
            self.bgw_queue.push_back(pixel);
            self.fetcher.push_x += 1;
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
                .map_area(self.lcdc.bg_map_area())
                .get_unchecked(tile_idx)
        };
        self.fetcher.bgw_data_idx = (self.lcdc.window_bg_data_area().addr(data_idx), y % 8);
        let tile_x = (self.fetcher.fetch_x as i16) - (self.wx as i16) + 7;
        self.fetcher.tile_x_begin = (tile_x / 8) * 8 + (self.wx as i16) - 7;
    }
}
