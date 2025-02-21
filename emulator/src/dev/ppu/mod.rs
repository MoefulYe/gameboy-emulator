use bgp::Palette;
use dma::DMA;
use fetcher::{FetchState, FetchType, Fetcher};
use graphic::{
    decode_tiles, RGBAPalette, ScreenBitmap, TilesBitmap, NO_COLOR, PALETTE, PPU_CYCLES_PER_LINE,
    PPU_LINES_PER_FRAME, PPU_XRES, PPU_YRES, SCREEN_HEIGHT, SCREEN_WIDTH, TILES_HEIGHT,
    TILES_WIDTH,
};
use js_sys::Uint8ClampedArray;
use lcd::LCDDriver;
use lcdc::{LCDControl, PPU_ENABLE_POS};
use lcds::{LCDStat, WorkMode};
use oam::{ObjectPixel, OAM};
use std::collections::VecDeque;
use vram::VRAM;
use web_sys::{ImageData, OffscreenCanvasRenderingContext2d};

use crate::{
    types::{Addr, Word},
    utils::{
        bits::BitMap,
        bytes::{as_bytes, as_bytes_mut},
    },
};

use super::{
    int_regs::{IRQ, IRQ_LCD_STAT, IRQ_NONE, IRQ_VBLANK},
    BusDevice, Reset, Tick,
};

mod bgp;
mod dma;
mod fetcher;
pub mod graphic;
mod lcd;
mod lcdc;
mod lcds;
mod oam;
mod vram;

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
#[allow(dead_code)]
pub const PPU_ADDR_HIGH_BOUND: Addr = WY_REG_ADDR + 1;

#[repr(u8)]
#[allow(dead_code)]
enum TileAreaType {
    From8800To97FF = 0,
    From8000To8FFF = 1,
}

impl TileAreaType {
    /// 给定图块索引号，返回data_area数组下标
    fn addr(self, idx: Word) -> Addr {
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

pub struct BGWPixel {
    color: Word,
    palette: Palette,
}

impl BGWPixel {
    pub fn final_color(&self) -> Word {
        let palette = self.palette.0;
        match self.color {
            0b00 => palette & 0b11,
            0b01 => (palette >> 2) & 0b11,
            0b10 => (palette >> 4) & 0b11,
            0b11 => (palette >> 6) & 0b11,
            _ => unreachable!(),
        }
    }
}

impl Default for BGWPixel {
    fn default() -> Self {
        Self {
            color: 0,
            palette: Palette(0),
        }
    }
}

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
    pub dma: DMA,
    /// 0xFF47
    bgp: Palette,
    /// 0xFF48
    obp0: Palette,
    /// 0xFF49
    obp1: Palette,
    /// 0xFF4A
    wx: Word,
    /// 0xFF4B
    wy: Word,

    line_cycles: u32,

    pub oam: OAM,
    pub vram: VRAM,

    bgw_queue: VecDeque<BGWPixel>,
    obj_queue: VecDeque<ObjectPixel>,

    fetcher: Fetcher,
    lcd_driver: LCDDriver,
    palette: RGBAPalette,

    tiles_canvas: Option<OffscreenCanvasRenderingContext2d>,
    tiles_buffer: Box<TilesBitmap>,
    screen_canvas: Option<OffscreenCanvasRenderingContext2d>,
    screen_buffers: [Box<ScreenBitmap>; 2],
    cur_buf: u8,
}

impl Reset for PPU {
    fn reset(&mut self) {
        self.lcdc.reset();
        self.lcds.reset();
        self.scy = 0;
        self.scx = 0;
        self.ly = 0;
        self.lyc = 0;
        self.dma.reset();
        self.bgp = Palette(0xFC);
        self.obp0 = Palette(0xFF);
        self.obp1 = Palette(0xFF);
        self.wx = 0;
        self.wy = 0;
        self.line_cycles = 0;
        self.oam.reset();
        self.vram.reset();
        self.bgw_queue.clear();
        self.obj_queue.clear();
        self.fetcher.reset();
        self.lcd_driver.reset();
        self.cur_buf = 0;
        unsafe {
            as_bytes_mut::<TilesBitmap>(&mut self.tiles_buffer).fill(0);
            as_bytes_mut::<ScreenBitmap>(&mut self.screen_buffers[0]).fill(0);
            as_bytes_mut::<ScreenBitmap>(&mut self.screen_buffers[1]).fill(0);
        }
        self.update_screen();
    }
}

impl PPU {
    pub fn new() -> Self {
        let tiles_buffer: Box<TilesBitmap> = Box::new([[NO_COLOR; 128]; 192]);
        let screen_buffers = [
            Box::new([[NO_COLOR; 160]; 144]),
            Box::new([[NO_COLOR; 160]; 144]),
        ];
        Self {
            lcdc: LCDControl(0b10010001),
            lcds: LCDStat(0x2),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: DMA::new(),
            bgp: Palette(0xFC),
            obp0: Palette(0xFF),
            obp1: Palette(0xFF),
            wx: 0,
            wy: 0,
            line_cycles: 0,
            oam: OAM::new(),
            vram: VRAM::new(),
            tiles_canvas: None,
            palette: PALETTE,
            tiles_buffer,
            bgw_queue: VecDeque::new(),
            obj_queue: VecDeque::new(),
            fetcher: Fetcher::new(),
            lcd_driver: LCDDriver::new(),
            screen_canvas: None,
            screen_buffers,
            cur_buf: 0,
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

    fn switch_buffer(&mut self) {
        self.cur_buf = 1 - self.cur_buf
    }

    fn current_buffer_mut(&mut self) -> &mut ScreenBitmap {
        unsafe { self.screen_buffers.get_unchecked_mut(self.cur_buf as usize) }
    }

    fn pred_buffer(&self) -> &ScreenBitmap {
        unsafe {
            self.screen_buffers
                .get_unchecked((1 - self.cur_buf) as usize)
        }
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
                self.vram.tiles_matrix(),
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
    pub fn update_screen(&self) {
        if let Some(canvas) = &self.screen_canvas {
            let buffer = as_bytes::<ScreenBitmap>(self.pred_buffer());
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
            DMA_REG_ADDR => self.dma.read(),
            BGP_REG_ADDR => self.bgp.read(),
            OBP0_REG_ADDR => self.obp0.read(),
            OBP1_REG_ADDR => self.obp1.read(),
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
            DMA_REG_ADDR => self.dma.write(data),
            BGP_REG_ADDR => self.bgp.write(data),
            OBP0_REG_ADDR => self.obp0.write(data),
            OBP1_REG_ADDR => self.obp1.write(data),
            WX_REG_ADDR => self.wx = data,
            WY_REG_ADDR => self.wy = data,
            _ => {}
        }
    }
}

impl Tick for PPU {
    fn tick(&mut self) -> IRQ {
        if self.disabled() {
            return IRQ_NONE;
        }
        self.line_cycles += 1;
        match self.mode() {
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
        } else if self.line_cycles == 1 {
            self.fetcher_oam_scan();
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
                self.obj_queue.clear();
                self.bgw_queue.clear();
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
                self.switch_buffer();
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
        self.lcdc.window_enabled() && self.wx <= 166 && self.wy < PPU_YRES
    }
}
