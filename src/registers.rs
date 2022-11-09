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

  pub fn inc_b(&mut self) -> u8 {
    self.b += 1;
    self.b
  }

  pub fn dec_b(&mut self) -> u8 {
    self.b -= 1;
    self.b
  }

  pub fn get_af(&self) -> u16 {
      let a = (self.a as u16) << 8;
      let f = self.f as u16;
      a | f
  }

  pub fn set_af(&mut self, v: u16) {
      self.f = v as u8;
      self.a = (v >> 8) as u8;
  }

  pub fn get_bc(&self) -> u16 {
      let b = (self.b as u16) << 8;
      let c = self.c as u16;
      b | c
  }

  pub fn inc_bc(&mut self) -> u16 {
    self.set_bc(self.get_bc() + 1);
    self.get_bc()
  }

  pub fn set_bc(&mut self, v: u16) {
      self.c = v as u8;
      self.b = (v >> 8) as u8;
  }

  pub fn get_de(&self) -> u16 {
      let d = (self.d as u16) << 8;
      let e = self.e as u16;
      d | e
  }

  pub fn set_de(&mut self, v: u16) {
      self.d = v as u8;
      self.e = (v >> 8) as u8;
  }

  pub fn get_hl(&self) -> u16 {
      let h = (self.h as u16) << 8;
      let l = self.l as u16;
      h | l
  }

  pub fn set_hl(&mut self, v: u16) {
      self.h = v as u8;
      self.l = (v >> 8) as u8;
  }


}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bc() {
    let mut r = Registers::new();

    r.set_bc(0xABCD);
    let bc = r.get_bc();
    assert_eq!(bc, 0xABCD);

    assert_eq!(r.b, 0xAB);
    assert_eq!(r.c, 0xCD);
  }
}
