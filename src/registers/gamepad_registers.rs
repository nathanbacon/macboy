struct GamepadRegisters {
  pub select_dpad: bool, // this is !p14
  pub select_buttons: bool, // this is !p15
  pub right: bool,
  pub left: bool,
  pub up: bool,
  pub down: bool,
  pub start: bool,
  pub select: bool,
  pub a: bool,
  pub b: bool,
}

impl GamepadRegisters {
  fn new(value: u8) -> GamepadRegisters {
    let select_dpad = (value & 0x10) == 0;
    let select_buttons = (value & 0x20) == 0;

    GamepadRegisters {
      select_dpad,
      select_buttons,
      right: false,
      left: false,
      up: false,
      down: false,
      start: false,
      select: false,
      a: false,
      b: false,
    }
  }

  fn read(self) -> u8 {
    if self.select_dpad {
      let mut v = 0x0F;
      if self.down {
        v ^= 0x08;
      }
      if self.up {
        v ^= 0x04;
      }
      if self.left {
        v ^= 0x02;
      }
      if self.right {
        v ^= 0x01;
      }
      v
    } else if self.select_buttons {
      let mut v = 0x0F;
      if self.start {
        v ^= 0x08;
      }
      if self.select {
        v ^= 0x04;
      }
      if self.b {
        v ^= 0x02;
      }
      if self.a {
        v ^= 0x01;
      }
      v
    } else {
      0x0F
    }
  }

  fn write(mut self, value: u8) {
    let select_dpad = (value & 0x10) > 0;
    let select_buttons = (value & 0x20) > 0;

    self.select_dpad = select_dpad;
    self.select_dpad = select_buttons;
  }
}
