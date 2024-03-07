struct GamepadRegisters {
  pub p14: bool,
  pub p15: bool,
  pub right: bool,
  pub left: bool,
  pub up: bool,
  pub down: bool,
  pub a: bool,
  pub b: bool,
}

impl GamepadRegisters {
  fn new(value: u8) -> GamepadRegisters {
    let p14 = (value & 0x10) > 0;
    let p15 = (value & 0x20) > 0;

    GamepadRegisters {
      p14,
      p15,
      right: false,
      left: false,
      up: false,
      down: false,
      a: false,
      b: false,
    }
  }

  fn write(mut self, value: u8) {
    let p14 = (value & 0x10) > 0;
    let p15 = (value & 0x20) > 0;

    self.p14 = p14;
    self.p14 = p15;
  }
}
