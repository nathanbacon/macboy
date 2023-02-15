use std::ops::{Index, IndexMut};

use crate::gpu::GPU;
use crate::cartridge::{Catridge, MBC, MBC3};

pub struct MMU<T> where T: MBC {
  gpu: GPU,
  mbc: T,
  working_memory: Box<[u8; 0x2000]>,
}

impl<T: MBC> MMU<T> {
  pub fn new(mbc: T) -> MMU<T> {
    MMU {
      gpu: GPU::new(),
      mbc,
      working_memory: Box::new([0u8; 0x2000]),
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
      0x8000..=0x9FFF => self.gpu.read(address),
      0xA000..=0xBFFF => self.mbc.read(address),
      0xC000..=0xDFFF => {
        let address = 0x1FFF & address;
        self.working_memory[address]
      },
      0xE000..=0xFDFF => {
        // forbidden according to manual but in actuality, it's a echo of working ram
        panic!("unimplemented read to 0xE000..=0xFDFF")
      },
      0xFE00..=0xFE9F => {
        // TODO: implement OAM access here
        panic!("unimplemented OAM read");
      },
      0xFEA0..=0xFEFF => {
        panic!("unimplemented");
      },
      0xFF00..=0xFF7F => {
        panic!("unimplemented");
      },
      0xFF80..=0xFFFE => {
        panic!("unimplemented");
      },
      0xFFFF => {
        panic!("unimplemented");
      },
      _ => panic!("unimplemented address space"),
    }
  }

  pub fn write(&mut self, address: u16, value: u8) {
    let address = address as usize;
    match address {
      0x0000..=0x7FFF => self.mbc.write(address, value),
      0x8000..=0x9FFF => self.gpu.write(address, value),
      0xA000..=0xBFFF => self.mbc.write(address, value),
      0xC000..=0xDFFF => {
        let address = address & 0x1FFF;
        self.working_memory[address] = value;
      },
      0xE000..=0xFDFF => {
        // forbidden according to manual but in actuality, it's a echo of working ram
        panic!("unimplemented write to 0xE000..=0xFDFF")
      },
      0xFE00..=0xFE9F => {
        // TODO: implement OAM access here
        panic!("unimplemented OAM write");
      },
      0xFEA0..=0xFEFF => {
        panic!("unimplemented");
      },
      0xFF00..=0xFF7F => {
        panic!("unimplemented");
      },
      0xFF80..=0xFFFE => {
        panic!("unimplemented");
      },
      0xFFFF => {
        panic!("unimplemented");
      },
      _ => panic!("unimplemented address space!"),
    }
  }
}
