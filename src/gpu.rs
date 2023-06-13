pub enum GpuEvent {
  None,
  LCD,
  VBlank,
}
pub struct GPU {
  memory: Box<[u8; 0x2000]>,
  oam: Box<[u8; 0xA0]>,
  ticks: u64,
}

impl GPU {
  pub fn new() -> GPU {
    GPU {
      memory: Box::new([0u8; 0x2000]),
      oam: Box::new([0u8; 0xA0]),
      ticks: 0,
    }
  }

  pub fn go(&mut self, ticks: u64) -> GpuEvent {
    self.ticks = self.ticks + ticks;
    GpuEvent::None
  }

  pub fn read(&self, address: usize) -> u8 {
    let address = address & 0x1FFF;
    self.memory[address]
  }

  pub fn write(&mut self, address: usize, value: u8) {
    let address = address & 0x1FFF;
    self.memory[address] = value;
  }

  pub fn subscript(&mut self, index: usize) -> &mut u8 {
    &mut self.memory[index]
  }
}
