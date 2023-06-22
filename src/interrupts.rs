pub struct Interrupts {
  vblank_interrupt: bool,
  vblank_enabled: bool,
  lcd_interrupt: bool,
  lcd_enabled: bool,
  timer_interrupt: bool,
  timer_enabled: bool,
  serial_interrupt: bool,
  serial_enabled: bool,
  joypad_interrupt: bool,
  joypad_enabled: bool,
}

impl Interrupts {
    pub fn new() -> Interrupts {
      Interrupts {
        vblank_interrupt: false,
        vblank_enabled: false,
        lcd_interrupt: false,
        lcd_enabled: false,
        timer_interrupt: false,
        timer_enabled: false,
        serial_interrupt: false,
        serial_enabled: false,
        joypad_interrupt: false,
        joypad_enabled: false,
      }
    }

    pub fn set(&mut self, ie: u8) {
      self.vblank_enabled = (ie & 0x01) == 0x01;
      self.lcd_enabled = (ie & 0x02) == 0x02;
      self.timer_enabled = (ie & 0x04) == 0x04;
      self.serial_enabled = (ie & 0x08) == 0x08;
      self.joypad_enabled = (ie & 0x10) == 0x10;
    }

    pub fn get(&self) -> u8 {
      let vblank_bit: u8 = if self.vblank_enabled {
        0x01
      } else {
        0
      };

      let lcd_bit: u8 = if self.lcd_enabled {
        0x02
      } else {
        0
      };

      let timer_bit: u8 = if self.timer_enabled {
        0x04
      } else {
        0
      };

      let serial_bit: u8 = if self.serial_enabled {
        0x08
      } else {
        0
      };

      let joypad_bit: u8 = if self.joypad_enabled {
        0x10
      } else {
        0
      };

      joypad_bit |
      serial_bit |
      timer_bit |
      lcd_bit |
      vblank_bit
    }
}

