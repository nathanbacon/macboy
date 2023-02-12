pub struct GPU {
  memory: Box<[u8; 0x2000]>,
}

impl GPU {
  pub fn new() -> GPU {
    GPU {
      memory: Box::new([0u8; 0x2000]),
    }
  }
  pub fn subscript(&mut self, index: usize) -> &mut u8 {
    &mut self.memory[index]
  }
}