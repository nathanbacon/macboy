pub struct GPU {
  memory: [u8; 0x2000],
}

impl GPU {
  pub fn subscript(&mut self, index: usize) -> &mut u8 {
    &mut self.memory[index]
  }
}