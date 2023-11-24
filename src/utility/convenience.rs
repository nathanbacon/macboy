pub fn break_byte_into_flags(byte: u8) -> [bool; 8] {
  let mut flags: [bool; 8] = [false, false, false, false, false, false, false, false];
  let mut select: u8 = 0x01;

  for i in 0..8 {
    flags[i] = (byte & select) > 0;
    select <<= 1;
  }

  flags
}

pub fn collapse_flags_into_byte(flags: [bool; 8]) -> u8 {
  let mut byte: u8 = 0;
  for flag in flags {
    byte >>= 1;
    if flag {
      byte |= 0x80;
    }
  }
  byte
}
