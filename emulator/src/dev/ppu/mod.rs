use std::{mem::size_of, ptr::slice_from_raw_parts};

use graphic::{decode_tiles, Palette, TilesBitmap, TILES_HEIGHT, TILES_WIDTH};
use js_sys::Uint8ClampedArray;
use lcdc::{LCDControl, PPU_ENABLE_POS};
use lcds::{LCDStat, WorkMode};
use web_sys::{ImageData, OffscreenCanvasRenderingContext2d};

use crate::{
    types::{Addr, Word},
    utils::bits::BitMap,
};

use super::{
    int_regs::{IRQ, IRQ_LCD_STAT, IRQ_NONE, IRQ_VBLANK},
    rams::{OAM, VRAM},
    BusDevice, Reset, Tick,
};

mod graphic;
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

const PPU_LINES_PER_FRAME: u8 = 154;
const PPU_CYCLES_PER_LINE: u32 = 456;
const PPU_YRES: Word = 144;
const PPU_XRES: Word = 160;
const PLAETTE: Palette = [
    [0xff, 0xff, 0xff, 0xff],
    [0xaa, 0xaa, 0xaa, 0xff],
    [0x55, 0x55, 0x55, 0xff],
    [0x00, 0x00, 0x00, 0xff],
];

pub struct PPU {
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
    bgp: Word,
    /// 0xFF48
    obp0: Word,
    /// 0xFF49
    obp1: Word,
    /// 0xFF4A
    wx: Word,
    /// 0xFF4B
    wy: Word,
    ticks: u32,
    palette: Palette,
    oam: OAM,
    vram: VRAM,
    canvas: Option<OffscreenCanvasRenderingContext2d>,
    tiles_canvas: Option<OffscreenCanvasRenderingContext2d>,
    tiles_buffer: Box<TilesBitmap>,
    tiles_image_data: ImageData,
}

impl PPU {
    pub fn new() -> Self {
        let tiles_buffer = Box::new([[[0; 4]; 128]; 192]);
        let _tiles_buffer = unsafe {
            &*slice_from_raw_parts(tiles_buffer.as_ptr() as *const u8, size_of::<TilesBitmap>())
        };
        let u8s = unsafe { Uint8ClampedArray::view(_tiles_buffer) };
        let tiles_image_data = ImageData::new_with_js_u8_clamped_array_and_sh(
            &u8s,
            TILES_WIDTH as _,
            TILES_HEIGHT as _,
        )
        .unwrap();
        Self {
            lcdc: LCDControl::new(0b10010001),
            lcds: LCDStat::new(0x2),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wx: 0,
            wy: 0,
            ticks: 0,
            oam: OAM::new(),
            vram: VRAM::new(),
            canvas: None,
            tiles_canvas: None,
            palette: PLAETTE,
            tiles_buffer,
            tiles_image_data,
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

    pub fn oam(&self) -> &OAM {
        &self.oam
    }

    pub fn vram(&self) -> &VRAM {
        &self.vram
    }

    pub fn oam_mut(&mut self) -> &mut OAM {
        &mut self.oam
    }

    pub fn vram_mut(&mut self) -> &mut VRAM {
        &mut self.vram
    }

    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.canvas = Some(canvas)
    }

    pub fn set_tiles_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.tiles_canvas = Some(canvas)
    }

    pub fn decode_tiles(&mut self) {
        if let Some(canvas) = &self.tiles_canvas {
            decode_tiles(self.vram.0.as_ref(), &self.palette, &mut self.tiles_buffer);
            canvas
                .put_image_data(&self.tiles_image_data, 0.0, 0.0)
                .unwrap();
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
            BGP_REG_ADDR => self.bgp,
            OBP0_REG_ADDR => self.obp0,
            OBP1_REG_ADDR => self.obp1,
            WX_REG_ADDR => self.wx,
            WY_REG_ADDR => self.wy,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            LCDC_REG_ADDR => {
                if self.enabled() && !data.test(PPU_ENABLE_POS) {
                    self.set_mode(WorkMode::HBlank);
                    self.ly = 0;
                    self.ticks = 0;
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
            BGP_REG_ADDR => self.bgp = data,
            OBP0_REG_ADDR => self.obp0 = data,
            OBP1_REG_ADDR => self.obp1 = data,
            WX_REG_ADDR => self.wx = data,
            WY_REG_ADDR => self.wy = data,
            _ => unreachable!(),
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
        self.ticks += 1;
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
        // TODO
        if self.ticks >= 80 {
            self.set_mode(WorkMode::Drawing)
        }
        IRQ_NONE
    }

    fn tick_drawing(&mut self) -> IRQ {
        if self.ticks >= 369 {
            self.set_mode(WorkMode::HBlank);
            if self.lcds.hblank_int() {
                IRQ_LCD_STAT
            } else {
                IRQ_NONE
            }
        } else {
            IRQ_NONE
        }
    }

    fn tick_hblank(&mut self) -> IRQ {
        if self.ticks >= PPU_CYCLES_PER_LINE {
            let mut irq = self.inc_ly();
            if self.ly >= PPU_YRES {
                self.set_mode(WorkMode::VBlank);
                irq |= IRQ_VBLANK;
                if self.lcds.vblank_int() {
                    irq |= IRQ_LCD_STAT
                }
            } else {
                self.set_mode(WorkMode::OAMScan);
                if self.lcds.oam_int() {
                    irq |= IRQ_LCD_STAT
                }
            }
            self.ticks = 0;
            irq
        } else {
            IRQ_NONE
        }
    }

    fn inc_ly(&mut self) -> IRQ {
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
        if self.ticks >= PPU_CYCLES_PER_LINE {
            let mut irq = self.inc_ly();
            if self.ly >= PPU_LINES_PER_FRAME {
                self.set_mode(WorkMode::OAMScan);
                self.ly = 0;
                if self.lcds.oam_int() {
                    irq |= IRQ_LCD_STAT;
                }
            }
            self.ticks = 0;
            irq
        } else {
            IRQ_NONE
        }
    }
}
