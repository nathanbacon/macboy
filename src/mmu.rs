use std::ops::{Index, IndexMut};

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

  pub fn read(&self, address: u16) -> u8 {
    let address = address as usize;
    match address {
      0x0000..=0x3FFF => self.bank0[address],
      0x4000..=0x7FFF => self.bank1[address],
      0x8000..=0x9FFF => 0,
      _ => 0
    }
  }
}

impl Index<usize> for MMU {
  type Output = u8;

  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0x0000..=0x3FFF => &self.bank0[index],
      0x4000..=0x7FFF => &self.bank1[index],
      _ => &self.bank0[index],
    }
  }
}

impl IndexMut<usize> for MMU {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0x0000..=0x3FFF => &mut self.bank0[index],
      0x4000..=0x7FFF => &mut self.bank1[index],
      _ => &mut self.bank0[index],
    }
  }
}
