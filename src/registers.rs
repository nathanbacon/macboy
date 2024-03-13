pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub s: u8,
  pub p: u8,
  pub sp: u16,
  pub pc: u16,
}

const ZERO: u8 = 0x80;
const NEGATIVE: u8 = 0x40;
const HALF_CARRY: u8 = 0x20;
const CARRY: u8 = 0x10;

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
      s: 0xFF,
      p: 0xFE,
      sp: 0xFFFE,
      pc: 0x0100,
    }
  }

  pub fn get_zero(&self) -> bool {
    self.f & ZERO == ZERO
  }

  pub fn get_not_zero(&self) -> bool {
    !self.get_zero()
  }

  pub fn zero(&mut self, is_zero: bool) {
    if is_zero {
      self.f |= ZERO;
    } else {
      self.f &= ZERO ^ 0xFF;
    }
  }

  pub fn negative(&mut self, is_negative: bool) {
    if is_negative {
      self.f |= NEGATIVE;
    } else {
      self.f &= NEGATIVE ^ 0xFF;
    }
  }

  pub fn get_negative(&self) -> bool {
    self.f & NEGATIVE == NEGATIVE
  }

  pub fn carry(&mut self, is_carry: bool) {
    if is_carry {
      self.f |= CARRY;
    } else {
      self.f &= CARRY ^ 0xFF;
    }
  }

  pub fn get_carry(&self) -> bool {
    self.f & CARRY == CARRY
  }

  pub fn get_not_carry(&self) -> bool {
    !self.get_carry()
  }

  pub fn get_half_carry(&self) -> bool {
    self.f & HALF_CARRY == HALF_CARRY
  }

  pub fn half_carry(&mut self, is_half_carry: bool) {
    if is_half_carry {
      self.f |= HALF_CARRY;
    } else {
      self.f &= HALF_CARRY ^ 0xFF;
    }
  }

}

#[cfg(test)]
mod tests {
  

}
