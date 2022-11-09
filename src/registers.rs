pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub pc: u16,
}

macro_rules! wide {
  ($reg:expr, $hi:ident, $lo:ident) => {
      (($reg.$hi as u16) << 8) | ($reg.$lo as u16) 
  };
  ($reg:expr, $hi:ident, $lo:ident, $v:expr) => {
    $reg.$hi = ($v >> 8) as u8;
    $reg.$lo = $v as u8;
  };
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0x01,
      f: 0x00,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xd8,
      h: 0x01,
      l: 0x4d,
      sp: 0xFFFE,
      pc: 0x0100,
    }
  }

  pub fn zero(&mut self, is_zero: bool) {

  }

  pub fn negative(&mut self, is_negative: bool) {

  }

  pub fn carry(&mut self, is_carry: bool) {

  }

  pub fn half_carry(&mut self, is_half_carry: bool) {

  }
}

#[cfg(test)]
mod tests {
  use super::*;

}
