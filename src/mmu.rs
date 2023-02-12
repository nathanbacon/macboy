use std::ops::{Index, IndexMut};

use crate::gpu::GPU;
use crate::cartridge::{Catridge, MBC, MBC3};

pub struct MMU<T> where T: MBC {
  gpu: GPU,
  mbc: T,
}

impl<T: MBC> MMU<T> {
  pub fn new(mbc: T) -> MMU<T> {
    MMU {
      gpu: GPU::new(),
      mbc,
    }
  }
  
  pub fn new_with_mbc3() -> MMU<MBC3> {
    let mbc3 = MBC3::new();

    MMU::new(mbc3)
  }

  pub fn read(&self, address: u16) -> u8 {
    let address = address as usize;
    match address {
      0x0000..=0x7FFF => self.mbc.read(address),
      0xA000..=0xBFFF => self.mbc.read(address),
      _ => panic!("unimplemented address space"),
    }
  }

  pub fn write(&mut self, address: u16, value: u8) {
    let address = address as usize;
    match address {
      0x0000..=0x7FFF => self.mbc.write(address, value),
      0xA000..=0xBFFF => self.mbc.write(address, value),
      _ => panic!("unimplemented address space!"),
    }
  }
}
