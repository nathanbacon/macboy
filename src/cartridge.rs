pub struct Catridge<T: MBC> {
  mbc: T,
}

impl<T: MBC> Catridge<T> {
  pub fn new(mbc: T) -> Catridge<T> {
    Catridge { mbc }
  }
}

pub trait WritableMemory {
  fn write(&mut self, address: usize, value: u8);
}

pub trait ReadableMemory {
  fn read(&self, address: usize) -> &u8;
}

pub trait MBC: WritableMemory + ReadableMemory {

}

pub struct MBC0 {
  rom: [u8; 0x8000]
}

pub struct MBC1 {
  rom_banks: [[u8; 0x2000]; 0xFFFFF]
}

pub struct MBC3 {
  rom_bank_select: u8,
  rom_banks: [[u8; 0x4000]; 0x80],
  ram_bank_select: u8,
  ram_banks: [[u8; 0x2000]; 0x04],
}

impl MBC3 {
  pub fn new() -> MBC3 {
    MBC3 { rom_bank_select: 1, rom_banks: [[0u8; 0x4000]; 0x80], ram_bank_select: 0, ram_banks: [[0u8; 0x2000]; 0x04] }
  }
}

impl ReadableMemory for MBC3 {
  fn read(&self, address: usize) -> &u8 {
    match address {
      0x0000..=0x3FFF => &self.rom_banks[0][address],
      0x4000..=0x7FFF => {
        let bank_select = self.rom_bank_select as usize;
        let physical_address = address & 0x3FFF;
        return &self.rom_banks[bank_select][physical_address];
      },
      0xA000..=0xBFFF => {
        let ram_bank_select = self.ram_bank_select as usize;
        let physical_address = address & 0x1FFF;
        return &self.ram_banks[ram_bank_select][physical_address];
      },
      _ => {
        panic!("invalid address to read in MBC3, this must be a programming error");
      }
    }
  }
}

impl WritableMemory for MBC3 {
  fn write(&mut self, address: usize, value: u8) {
    match address {
      0x0000..=0x1FFF => {
        // TODO: write protect RAM here, somehow
      },
      0x2000..=0x3FFF => {
        let rom_bank = value & 0x7F;
        if rom_bank == 0 {
          panic!("selecting this ROM bank in MBC3 is undefined");
        }
        self.rom_bank_select = rom_bank;
      },
      0x4000..=0x5FFF => {
        match value {
          0x00..=0x03 => {
            self.ram_bank_select = value;
          },
          0x08..=0x0C => {
            // TODO: handle counters
          },
          _ => {
            // not used, placeholder
          }
        }
      },
      0x6000..=0x7FFF => {
        // writing this region is not defined in the manual and will behave as noop
      },
      0xA000..=0xBFFF => {
        let ram_bank_select = self.ram_bank_select as usize;
        let physical_address = address & 0x1FFF;
        self.ram_banks[ram_bank_select][physical_address] = value;
      },
      _ => {
        // if this happens it must be a code error because the MMU should do something else
        panic!("invalid address to right to MBC3");
      }
    }
    let bank_select = self.rom_bank_select as usize;

    self.rom_banks[bank_select][address] = value;
  }
}

impl MBC for MBC3 {

}

