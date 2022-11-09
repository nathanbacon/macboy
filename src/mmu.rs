
use std::ops::{Index,};

pub struct MMU {
  bank0: [u8; 0x4000],
  bank1: [u8; 0x4000],
}

impl MMU {
  pub fn new() -> MMU {
    MMU {
      bank0: [(); 0x4000].map(|_| 0),
      bank1: [(); 0x4000].map(|_| 0),
    }
  }

  pub fn read_16_bit_immediate(&self, address: u16) -> u16 {
    let lower = self.read(address) as u16;
    let upper = self.read(address + 1) as u16; 
    (upper << 8) | lower
  }

  pub fn read(&self, address: u16) -> u8 {
    let address = address as usize;
    match address {
      0x0000..=0x3FFF => self.bank0[address],
      0x4000..=0x7FFF => self.bank1[address],
      _ => 0
    }
  }

  pub fn write(&mut self, address: u16, value: u8) {
    let address = address as usize;
    match address {
      0x0000..=0x3FFF => self.bank0[address] = value,
      0x4000..=0x7FFF => self.bank1[address] = value,
      _ => {}
    }
  }
}
