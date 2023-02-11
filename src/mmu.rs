use std::ops::{Index, IndexMut};

use crate::gpu::GPU;
use crate::cartridge::{Catridge, MBC, MBC3};

pub struct MMU<T> where T: MBC {
  bank0: [u8; 0x4000],
  bank1: [u8; 0x4000],
  gpu: GPU,
  mbc: T,
}

impl<T: MBC> MMU<T> {
  pub fn new(mbc: T) -> MMU<T> {
    MMU {
      bank0: [(); 0x4000].map(|_| 0),
      bank1: [(); 0x4000].map(|_| 0),
      gpu: GPU::new(),
      mbc,
    }
  }
  
  pub fn new_with_mbc3() -> MMU<T> {
    MMU {
      ..MMU::new(MBC3::new())
    }
  }

  pub fn read(&self, address: u16) -> u8 {
    let address = address as usize;
    match address {
      0x0000..=0x7FFF => self.mbc.read(address),
      _ => panic!("unimplemented address space"),
    }
  }

  pub fn write(&mut self, address: u16, value: u8) {
    let address = address as usize;
    match address {
      0x0000..=0x7FFF => self.mbc.write(address, value),
      _ => panic!("unimplemented address space!"),
    }
  }
}
