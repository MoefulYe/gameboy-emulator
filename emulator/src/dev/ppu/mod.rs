use bgp::BGP;
use fetcher::{FetchState, FetchType, Fetcher};
use graphic::{
    decode_tiles, Palette, RawPxiel, ScreenBitmap, TilesBitmap, NO_COLOR, PALETTE,
    PPU_CYCLES_PER_LINE, PPU_LINES_PER_FRAME, PPU_XRES, PPU_YRES, SCREEN_HEIGHT, SCREEN_WIDTH,
    TILES_HEIGHT, TILES_WIDTH,
};
use js_sys::{Uint8Array, Uint8ClampedArray};
use lcd::LCDDriver;
use lcdc::{LCDControl, PPU_ENABLE_POS};
use lcds::{LCDStat, WorkMode};
use log::{debug, error, info, log, warn};
use std::{
    borrow::BorrowMut, collections::VecDeque, convert::TryInto, mem::size_of,
    ptr::slice_from_raw_parts,
};
use web_sys::{ImageData, OffscreenCanvasRenderingContext2d};

use crate::{
    types::{Addr, Word},
    utils::{bits::BitMap, bytes::as_bytes},
};

use super::{
    int_regs::{IRQ, IRQ_LCD_STAT, IRQ_NONE, IRQ_VBLANK},
    rams::{OAM, VRAM},
    BusDevice, Reset, Tick,
};

mod bgp;
mod fetcher;
pub mod graphic;
mod lcd;
mod lcdc;
mod lcds;

const LCDC_REG_ADDR: Addr = 0xFF40;
const LCDS_REG_ADDR: Addr = 0xFF41;
const SCY_REG_ADDR: Addr = 0xFF42;
const SCX_REG_ADDR: Addr = 0xFF43;
const LY_REG_ADDR: Addr = 0xFF44;
const LYC_REG_ADDR: Addr = 0xFF45;
const DMA_REG_ADDR: Addr = 0xFF46;
const BGP_REG_ADDR: Addr = 0xFF47;
const OBP0_REG_ADDR: Addr = 0xFF48;
const OBP1_REG_ADDR: Addr = 0xFF49;
const WX_REG_ADDR: Addr = 0xFF4A;
const WY_REG_ADDR: Addr = 0xFF4B;

pub const PPU_ADDR_LOW_BOUND: Addr = LCDC_REG_ADDR;
pub const PPU_ADDR_HIGH_BOUND_INCLUDED: Addr = WY_REG_ADDR + 1;
pub const PPU_ADDR_HIGH_BOUND: Addr = WY_REG_ADDR + 1;

#[repr(u8)]
pub enum TileAreaType {
    From8800To97FF = 0,
    From8000To8FFF = 1,
}

impl TileAreaType {
    /// 给定图块索引号，返回data_area数组下标
    pub fn addr(self, idx: Word) -> Addr {
        match self {
            TileAreaType::From8800To97FF => idx.wrapping_add(128) as Addr + 128,
            TileAreaType::From8000To8FFF => idx.into(),
        }
    }
}

#[repr(u8)]
pub enum MapAreaType {
    From9800To9BFF = 0,
    From9C00To9FFF = 1,
}

pub type TileAreaIdx = u8;
pub type MapArea = [TileAreaIdx; 1024];

pub struct PPU {
    // regs
    /// 0xFF40
    lcdc: LCDControl,
    /// 0xFF41
    lcds: LCDStat,
    /// 0xFF42
    scy: Word,
    /// 0xFF43
    scx: Word,
    /// 0xFF44
    ly: Word,
    /// 0xFF45
    lyc: Word,
    /// 0xFF46
    dma: Word,
    /// 0xFF47
    bgp: BGP,
    /// 0xFF48
    obp0: Word,
    /// 0xFF49
    obp1: Word,
    /// 0xFF4A
    wx: Word,
    /// 0xFF4B
    wy: Word,

    line_cycles: u32,

    pub oam: OAM,
    pub vram: VRAM,

    bgw_queue: VecDeque<RawPxiel>,
    fetcher: Fetcher,
    lcd_driver: LCDDriver,
    palette: Palette,

    tiles_canvas: Option<OffscreenCanvasRenderingContext2d>,
    tiles_buffer: Box<TilesBitmap>,
    screen_canvas: Option<OffscreenCanvasRenderingContext2d>,
    screen_buffer: Box<ScreenBitmap>,
}

impl PPU {
    pub fn new() -> Self {
        let tiles_buffer: Box<TilesBitmap> = Box::new([[NO_COLOR; 128]; 192]);
        let screen_buffer: Box<ScreenBitmap> = Box::new([[NO_COLOR; 160]; 144]);
        Self {
            lcdc: LCDControl(0b10010001),
            lcds: LCDStat(0x2),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: BGP(0xFC),
            obp0: 0xFF,
            obp1: 0xFF,
            wx: 0,
            wy: 0,
            line_cycles: 0,
            oam: OAM::new(),
            vram: VRAM::new(),
            tiles_canvas: None,
            palette: PALETTE,
            tiles_buffer,
            bgw_queue: VecDeque::new(),
            fetcher: Fetcher::new(),
            lcd_driver: LCDDriver::new(),
            screen_canvas: None,
            screen_buffer,
        }
    }

    fn enabled(&self) -> bool {
        self.lcdc.enabled()
    }

    fn disabled(&self) -> bool {
        !self.enabled()
    }

    fn mode(&self) -> WorkMode {
        self.lcds.mode()
    }

    fn set_mode(&mut self, mode: WorkMode) {
        self.lcds.set_mode(mode)
    }
}

impl PPU {
    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.screen_canvas = Some(canvas)
    }

    pub fn set_tiles_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.tiles_canvas = Some(canvas)
    }

    pub fn update_tiles(&mut self) {
        if let Some(canvas) = &self.tiles_canvas {
            decode_tiles(
                self.vram.tiles_area(),
                &self.palette,
                self.tiles_buffer.as_mut(),
            );
            let buffer = as_bytes::<TilesBitmap>(self.tiles_buffer.as_ref());
            let u8s = unsafe { Uint8ClampedArray::view(buffer) };
            let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(
                &u8s,
                TILES_WIDTH as _,
                TILES_HEIGHT as _,
            )
            .unwrap();
            canvas.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }
    pub fn update_screen(&mut self) {
        if let Some(canvas) = &self.screen_canvas {
            let buffer = as_bytes::<ScreenBitmap>(self.screen_buffer.as_ref());
            let u8s = unsafe { Uint8ClampedArray::view(buffer) };
            let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(
                &u8s,
                SCREEN_WIDTH as _,
                SCREEN_HEIGHT as _,
            )
            .unwrap();
            canvas.put_image_data(&image_data, 0.0, 0.0).unwrap()
        }
    }
}

impl BusDevice for PPU {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            LCDC_REG_ADDR => *self.lcdc,
            LCDS_REG_ADDR => *self.lcds,
            SCY_REG_ADDR => self.scy,
            SCX_REG_ADDR => self.scx,
            LY_REG_ADDR => self.ly,
            LYC_REG_ADDR => self.lyc,
            DMA_REG_ADDR => self.dma,
            BGP_REG_ADDR => self.bgp.0,
            OBP0_REG_ADDR => self.obp0,
            OBP1_REG_ADDR => self.obp1,
            WX_REG_ADDR => self.wx,
            WY_REG_ADDR => self.wy,
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            LCDC_REG_ADDR => {
                if self.enabled() && !data.test(PPU_ENABLE_POS) {
                    self.set_mode(WorkMode::HBlank);
                    self.ly = 0;
                    self.line_cycles = 0;
                }
                *self.lcdc = data
            }
            LCDS_REG_ADDR => self.lcds.write(data),
            SCY_REG_ADDR => self.scy = data,
            SCX_REG_ADDR => self.scx = data,
            // readonly
            LY_REG_ADDR => {}
            LYC_REG_ADDR => self.lyc = data,
            DMA_REG_ADDR => self.dma = data,
            BGP_REG_ADDR => self.bgp.0 = data,
            OBP0_REG_ADDR => self.obp0 = data,
            OBP1_REG_ADDR => self.obp1 = data,
            WX_REG_ADDR => self.wx = data,
            WY_REG_ADDR => self.wy = data,
            _ => {}
        }
    }
}

impl Reset for PPU {
    fn reset(&mut self) {
        todo!()
    }
}

impl Tick for PPU {
    fn tick(&mut self) -> IRQ {
        if self.disabled() {
            return IRQ_NONE;
        }
        self.line_cycles += 1;
        let mode = self.mode();
        match mode {
            WorkMode::HBlank => self.tick_hblank(),
            WorkMode::VBlank => self.tick_vblank(),
            WorkMode::OAMScan => self.tick_oam_scan(),
            WorkMode::Drawing => self.tick_drawing(),
        }
    }
}

impl PPU {
    fn tick_oam_scan(&mut self) -> IRQ {
        if self.line_cycles >= 80 {
            self.set_mode(WorkMode::Drawing);
            self.fetcher.fetch_type = FetchType::FetchBackground;
            self.fetcher.state = FetchState::Tile;
            self.fetcher.fetch_x = 0;
            self.fetcher.push_x = 0;
            self.lcd_driver.draw_x = 0;
            self.bgw_queue.clear();
        }
        IRQ_NONE
    }

    fn tick_drawing(&mut self) -> IRQ {
        let mut irq = IRQ_NONE;
        if self.line_cycles % 2 == 0 {
            self.fetcher_update();
            if self.lcd_driver.draw_x >= PPU_XRES {
                self.set_mode(WorkMode::HBlank);
                if self.lcds.hblank_int() {
                    irq |= IRQ_LCD_STAT;
                }
            }
        }
        self.lcd_draw_pixel();
        irq
    }

    fn tick_hblank(&mut self) -> IRQ {
        let mut irq = IRQ_NONE;
        if self.line_cycles >= PPU_CYCLES_PER_LINE {
            irq |= self.inc_ly();
            if self.ly >= PPU_YRES {
                self.set_mode(WorkMode::VBlank);
                irq |= IRQ_VBLANK;
                if self.lcds.vblank_int() {
                    irq |= IRQ_LCD_STAT;
                }
                self.update_screen();
            } else {
                self.set_mode(WorkMode::OAMScan);
                if self.lcds.oam_int() {
                    irq |= IRQ_LCD_STAT;
                }
            }
            self.line_cycles = 0;
        }
        irq
    }

    fn inc_ly(&mut self) -> IRQ {
        if self.window_visible()
            && self.ly >= self.wy
            && (self.ly as u16) < (self.wy as u16 + PPU_YRES as u16)
        {
            self.fetcher.window_line += 1;
        }
        self.ly += 1;
        if self.ly == self.lyc {
            self.lcds.lyc_flag().set();
            if self.lcds.lyc_int() {
                IRQ_LCD_STAT
            } else {
                IRQ_NONE
            }
        } else {
            self.lcds.lyc_flag().clear();
            IRQ_NONE
        }
    }

    fn tick_vblank(&mut self) -> IRQ {
        let mut irq = IRQ_NONE;
        if self.line_cycles >= PPU_CYCLES_PER_LINE {
            irq |= self.inc_ly();
            if self.ly >= PPU_LINES_PER_FRAME {
                self.set_mode(WorkMode::OAMScan);
                self.ly = 0;
                self.fetcher.window_line = 0;
                if self.lcds.oam_int() {
                    irq |= IRQ_LCD_STAT;
                }
            }
            self.line_cycles = 0;
        }
        irq
    }

    fn window_visible(&self) -> bool {
        self.lcdc.window_enable() && self.wx <= 166 && self.wy < PPU_YRES
    }
}
