pub struct Interrupts {
  vblank_interrupt: bool,
  vblank_enabled: bool,
  lcd_interrupt: bool,
  lcd_enabled: bool,
  timer_interrupt: bool,
  timer_enabled: bool,
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
        joypad_interrupt: false,
        joypad_enabled: false,
      }
    }
}

