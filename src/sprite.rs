#[derive(Debug)]
pub struct Sprite {
  pub x: u8,
  pub y: u8,
  pub tile_number: u8,
  pub priority_flag: bool,
  pub y_flip_flag: bool,
  pub x_flip_flag: bool,
  pub pallete_number: u8,
}

impl Sprite {
  pub fn new() -> Sprite {
    Sprite {
      x: 0,
      y: 0,
      tile_number: 0,
      priority_flag: false,
      y_flip_flag: false,
      x_flip_flag: false,
      pallete_number: 0,
    }
  }

  pub fn write_flags(&mut self, value: u8) {
    let priority_flag = value & 0x80 == 0x80;
    let y_flip_flag = value & 0x40 == 0x40;
    let x_flip_flag = value & 0x20 == 0x20;
    let pallete_number = value & 0x10 as u8;
    self.priority_flag = priority_flag;
    self.y_flip_flag = y_flip_flag;
    self.x_flip_flag = x_flip_flag;
    self.pallete_number = pallete_number;
  }

  pub fn write_sprite_address(&mut self, address: usize, value: u8) {
    let address = address % 4;
    match address {
      0 => self.x = value,
      1 => self.y = value,
      2 => self.tile_number = value,
      3 => {
        self.write_flags(value);
      },
      _ => panic!("can't happen"),
    }
  }
}
