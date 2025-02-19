use std::ops::{Deref, DerefMut};

use crate::{
    dev::{
        bus::{OAM_LOW_BOUND, OAM_SIZE},
        BusDevice,
    },
    types::{Addr, Word},
    utils::{
        bits::BitMap,
        bytes::{from_bytes, from_bytes_mut},
    },
};

use super::bgp::Palette;

pub struct OAM(Box<[Word; OAM_SIZE]>);

impl Default for OAM {
    fn default() -> Self {
        Self(Box::new([0; OAM_SIZE]))
    }
}

impl Deref for OAM {
    type Target = [Word; OAM_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OAM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BusDevice for OAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - OAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - OAM_LOW_BOUND) as usize) } = data
    }
}

impl OAM {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_objs(&self) -> &Objects {
        unsafe { from_bytes(self.as_ref()) }
    }

    pub fn as_objs_mut(&mut self) -> &mut Objects {
        unsafe { from_bytes_mut(self.as_mut()) }
    }
}

#[repr(u8)]
pub enum ObjectPaletteSelect {
    OBP0 = 0,
    OBP1 = 1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Object {
    pub x: Word,
    pub y: Word,
    pub tile_idx: Word,
    pub flags: Word,
}

pub type Objects = [Object; 40];

impl Object {
    pub fn palette(&self) -> ObjectPaletteSelect {
        unsafe { std::mem::transmute(self.flags.at(4)) }
    }

    pub fn x_flip(&self) -> bool {
        self.flags.test(5)
    }

    pub fn y_flip(&self) -> bool {
        self.flags.test(6)
    }

    pub fn priority(&self) -> bool {
        self.flags.test(7)
    }
}

pub struct ObjectPixel {
    pub color: Word,
    pub palette: Palette,
    pub bg_priority: bool,
}

impl ObjectPixel {
    pub fn final_color(&self) -> Word {
        self.palette.apply(self.color)
    }
}

impl Default for ObjectPixel {
    fn default() -> Self {
        Self {
            color: 0,
            palette: Palette(0),
            bg_priority: true,
        }
    }
}

impl ObjectPixel {
    pub fn new() -> Self {
        Default::default()
    }
}
