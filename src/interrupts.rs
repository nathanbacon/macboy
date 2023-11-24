use crate::{utility::convenience}
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
      let flags = convenience::break_byte_into_flags(ie);
      self.vblank_enabled = flags[0];
      self.lcd_enabled = flags[1];
      self.timer_enabled = flags[2];
      self.serial_enabled = flags[3];
      self.joypad_enabled = flags[4];
    }

    pub fn get(&self) -> u8 {
      convenience::collapse_flags_into_byte([
        self.vblank_enabled,
        self.lcd_enabled,
        self.timer_enabled,
        self.serial_enabled,
        self.joypad_enabled,
        false,
        false,
        false,
        ])
    }
}

