pub struct Catridge {
  active_rom_bank_number: usize,
  rom_banks: Vec<[u8; 0x2000]>,
  active_ram_bank_number: usize,
  ram_banks: [[u8; 0x2000]; 4],
}

pub struct MBC0 {
  rom: [u8; 0x8000]
}

pub struct MBC1 {
  rom_banks: [[u8; 0x2000]; 0xFFFFF]
}

pub struct MBC3 {
  rom_bank_select: u8,
  rom_banks: [[u8; 0x4000]; 0x7F],
  ram_bank_select: u8,
}

impl ReadableMemory for MBC3 {
  fn read(&self, address: usize) -> &u8 {
    let bank_select = self.rom_bank_select as usize;
    &self.rom_banks[bank_select][address]
  }
}

impl WritableMemory for MBC3 {
  fn write(&mut self, address: usize, value: u8) {
      let bank_select = self.rom_bank_select as usize;
      self.rom_banks[bank_select][address] = value;
  }
}

trait WritableMemory {
  fn write(&mut self, address: usize, value: u8);
}

trait ReadableMemory {
  fn read(&self, address: usize) -> &u8;
}

impl Catridge {
}
