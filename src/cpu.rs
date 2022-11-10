use crate::{registers::Registers, mmu::MMU, };

pub struct CPU {
  registers: Registers,
  mmu: MMU,
  table: Vec<fn(&mut Registers, &mut MMU)>,
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

impl CPU {

  pub fn new() -> CPU {
    CPU {
      registers: Registers::new(),
      mmu: MMU::new(),
      table: CPU::build(),
    }
  }

  pub fn build() -> Vec<fn(&mut Registers, &mut MMU)> {
    macro_rules! I {
      (NOP) => {
        {
          fn eval(_registers: &mut Registers, _mmu: &mut MMU) {
            println!("NOP");
          }
          eval
        }
      };
      (INC [$dest_hi:ident $dest_lo:ident]) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let res = wide!(registers, $dest_hi, $dest_lo);
            wide!(registers, $dest_hi, $dest_lo, res + 1);
          }
          eval
        }
      };
      (INC SP) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            registers.sp += 1;
          }
          eval
        }
      };
      (INC $dest:ident) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let res: u8 = registers.$dest;
            registers.half_carry(res & 0x0F == 0x0F);
            registers.zero(res == 0xFF);
            registers.negative(false);
            registers.$dest = res + 1;
          }
          eval
        }
      };
      (INC (HL)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let address = wide!(registers, h, l);
            let res = mmu.read(address);
            registers.half_carry(res & 0x0F == 0x0F);
            registers.zero(res == 0xFF);
            registers.negative(false);
            mmu.write(address, res + 1);
          }
          eval
        }
      };
      (DEC $dest: ident) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let res: u8 = registers.$dest;
            registers.half_carry(res & 0x0F == 0);
            registers.negative(true);
            registers.$dest = res - 1;
            registers.zero(registers.$dest == 0);
          }
          eval
        }
      };
      (DEC (HL)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let address = wide!(registers, h, l);
            let res = mmu.read(address);
            registers.half_carry(res & 0x0F == 0);
            registers.zero(res == 0x01);
            registers.negative(true);
            mmu.write(address, res - 1);
          }
          eval
        }
      };
      (LD [$dest_hi:ident $dest_lo:ident], u16) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let v = mmu.read_16_bit_immediate(registers.pc);
            registers.pc += 2;
            wide!(registers, $dest_hi, $dest_lo, v);
          }
          eval
        }
      };
      (LD sp, u16) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let v = mmu.read_16_bit_immediate(registers.pc);
            registers.pc += 2;
            registers.sp = v;
          }
          eval
        }
      };
      (LD $dest:ident, u8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let v = mmu.read(registers.pc);
            registers.pc += 1;
            registers.$dest = v;
          }
          eval
        }
      };
      (LD $dest:ident, (HL)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let address = wide!(registers, h, l);
            let v = mmu.read(address);
            registers.pc += 1;
            registers.$dest = v;
          }
          eval
        }
      };
      (LD $dest:ident, $src:ident) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            registers.$dest = registers.$src;
          }
          eval
        }
      };
      (LD (u16), SP) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let dest_address = mmu.read_16_bit_immediate(registers.pc);
            registers.pc += 2;
            let src = registers.sp;
            let lower = (src & 0x00FF) as u8;
            let upper = (src >> 8) as u8;
            mmu.write(dest_address, lower);
            mmu.write(dest_address + 1, upper);
          }
          eval
        }
      };
      (LD (HL), u8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let src = registers.pc;
            let src = mmu.read(src);
            registers.pc += 1;
            let dest_address = wide!(registers, h, l);
            mmu.write(dest_address, src);
          }
          eval
        }
      };
      (LD ([$dest_hi:ident $dest_lo:ident]), $src:ident) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let dest_address = wide!(registers, $dest_hi, $dest_lo);
            let src = registers.$src;
            mmu.write(dest_address, src);
          }
          eval
        }
      };
      (RLCA) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let mut a = registers.a as u16;
            a <<= 1;
            registers.zero(false);
            registers.negative(false);
            registers.half_carry(false);
            let carry_bit = a >> 8;
            registers.carry(carry_bit == 0x01);
            a |= carry_bit;
            registers.a = a as u8;
          }
          eval
        }
      };
      (RLA) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let mut a = registers.a as u16;
            a <<= 1;
            registers.zero(false);
            registers.negative(false);
            registers.half_carry(false);
            let carry_bit = 0x100 & a;
            registers.carry(carry_bit == 0x100);
            registers.a = a as u8;
          }
          eval
        }
      };
      (RRA) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let mut a = registers.a;
            let carry_bit = a & 0x01;
            a >>= 1;
            registers.zero(false);
            registers.negative(false);
            registers.half_carry(false);
            registers.carry(carry_bit == 0x01);
            registers.a = a;
          }
          eval
        }
      };
      (RRCA) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let mut a = registers.a;
            let mut carry_bit = a & 0x01;
            registers.carry(carry_bit == 0x01);
            a >>= 1;
            registers.zero(false);
            registers.negative(false);
            registers.half_carry(false);
            
            carry_bit <<= 7;
            a |= carry_bit;
            registers.a = a;
          }
          eval
        }
      };
      (ADD [$hi_d:ident $lo_d:ident], [$hi_s:ident $lo_s:ident]) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let src = wide!(registers, $hi_s, $lo_s) as u32;
            let dest = wide!(registers, $hi_d, $lo_d) as u32;
            let res = src + dest;

            registers.negative(false);
            let half_carry = ((0x0FFF & src) + (0x0FFF & dest) & 0x1000) == 0x1000;
            registers.half_carry(half_carry);
            registers.carry((res & 0x10000) == 0x10000);

            wide!(registers, $hi_d, $lo_d, res);
          }
          eval
        }
      };
      (LD $dest:ident, ([$src_hi:ident $src_lo:ident])) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let src_addr = wide!(registers, $src_hi, $src_lo);
            let src = mmu.read(src_addr);
            registers.$dest = src;
          }
          eval
        }
      };
      (DEC [$src_hi:ident $src_lo:ident]) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let res = wide!(registers, b, c);
            wide!(registers, b, c, res - 1);
          }
          eval
        }
      };
      (STOP) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            println!("STOP");
          }
          eval
        }
      };
      (JR i8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let immed = mmu.read(registers.pc) as i8;
            registers.pc += 1;
            let immed = i16::from(immed);
            let signed_pc = registers.pc as i16;
            let new_pc = signed_pc + immed;
            registers.pc = (new_pc as u16);
          }
          eval
        }
      };
      (JR NZ, i8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let immed = mmu.read(registers.pc) as i8;
            registers.pc += 1;
            let not_zero = !registers.get_zero();
            if !not_zero {
              return;
            }

            let immed = i16::from(immed);
            let signed_pc = registers.pc as i16;
            let new_pc = signed_pc + immed;
            registers.pc = (new_pc as u16);
          }
          eval
        }
      };
      (JR NC, i8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let immed = mmu.read(registers.pc) as i8;
            registers.pc += 1;
            let not_carry = !registers.get_carry();
            if !not_carry {
              return;
            }

            let immed = i16::from(immed);
            let signed_pc = registers.pc as i16;
            let new_pc = signed_pc + immed;
            registers.pc = (new_pc as u16);
          }
          eval
        }
      };
      (JR Z, i8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let immed = mmu.read(registers.pc) as i8;
            registers.pc += 1;
            let zero = registers.get_zero();
            if !zero {
              return;
            }

            let immed = i16::from(immed);
            let signed_pc = registers.pc as i16;
            let new_pc = signed_pc + immed;
            registers.pc = (new_pc as u16);
          }
          eval
        }
      };
      (JR C, i8) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let immed = mmu.read(registers.pc) as i8;
            registers.pc += 1;
            let carry = registers.get_carry();
            if carry {
              return;
            }

            let immed = i16::from(immed);
            let signed_pc = registers.pc as i16;
            let new_pc = signed_pc + immed;
            registers.pc = (new_pc as u16);
          }
          eval
        }
      };
      (LD (HL+), a) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let hl = wide!(registers, h, l);
            let a = registers.a;
            mmu.write(hl, a);
            wide!(registers, h, l, hl + 1);
          }
          eval
        }
      };
      (LD (HL-), a) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let hl = wide!(registers, h, l);
            let a = registers.a;
            mmu.write(hl, a);
            wide!(registers, h, l, hl - 1);
          }
          eval
        }
      };
      (LD a, (HL+)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let hl = wide!(registers, h, l);
            let v = mmu.read(hl);
            registers.a = v;
            wide!(registers, h, l, hl + 1);
          }
          eval
        }
      };
      (LD a, (HL-)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let hl = wide!(registers, h, l);
            let v = mmu.read(hl);
            registers.a = v;
            wide!(registers, h, l, hl - 1);
          }
          eval
        }
      };
      (DAA) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
          }
          eval
        }
      };
      (CPL) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            registers.a = !registers.a;
            registers.negative(true);
            registers.half_carry(true);
          }
          eval
        }
      }; 
      (SCF) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            registers.negative(false);
            registers.half_carry(false);
            registers.carry(true);
          }
          eval
        }
      };
      (CCF) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            registers.negative(false);
            registers.half_carry(false);
            let carry = registers.get_carry();
            registers.carry(!carry);
          }
          eval
        }
      };
    }

    vec![
      I!(NOP),
      I!(LD [b c], u16),
      I!(LD ([b c]), a),
      I!(INC [b c]),
      I!(INC b),
      I!(DEC b),
      I!(LD b, u8),
      I!(RLCA),
      I!(LD (u16), SP),
      I!(ADD [h l], [b c]),
      I!(LD a, ([b c])),
      I!(DEC [b c]),
      I!(INC c),
      I!(DEC c),
      I!(LD c, u8),
      I!(RRCA),

      I!(STOP),
      I!(LD [d e], u16),
      I!(LD ([d e]), a),
      I!(INC [d e]),
      I!(INC d),
      I!(DEC d),
      I!(LD d, u8),
      I!(RLA),
      I!(JR i8),
      I!(ADD [h l], [d e]),
      I!(LD a, ([d e])),
      I!(DEC [d e]),
      I!(INC e),
      I!(DEC e),
      I!(LD e, u8),
      I!(RRA),

      I!(JR NZ, i8),
      I!(LD [h l], u16),
      I!(LD (HL+), a),
      I!(INC [h l]),
      I!(INC h),
      I!(DEC h),
      I!(LD h, u8),
      I!(DAA),
      I!(JR Z, i8),
      I!(ADD [h l], [h l]),
      I!(LD a, (HL+)),
      I!(DEC [h l]),
      I!(INC l),
      I!(DEC l),
      I!(LD l, u8),
      I!(CPL),

      I!(JR NC, i8),
      I!(LD sp, u16),
      I!(LD (HL-), a),
      I!(INC SP),
      I!(INC (HL)),
      I!(DEC (HL)),
      I!(LD (HL), u8),
      I!(SCF),
      I!(JR C, i8),
      I!(ADD [h l], [s p]),
      I!(LD a, (HL-)),
      I!(DEC [s p]),
      I!(INC a),
      I!(DEC a),
      I!(LD a, u8),
      I!(CCF),

      I!(LD b, b),
      I!(LD b, c),
      I!(LD b, d),
      I!(LD b, e),
      I!(LD b, h),
      I!(LD b, l),
      I!(LD b, (HL)),
      I!(LD b, a),
      I!(LD c, b),
      I!(LD c, c),
      I!(LD c, d),
      I!(LD c, e),
      I!(LD c, h),
      I!(LD c, l),
      I!(LD c, (HL)),
      I!(LD c, a),
      
    ]
  }

  fn read_16_bit_immediate(&mut self) -> u16 {
    let lower = self.mmu.read(self.registers.pc) as u16;
    let upper = self.mmu.read(self.registers.pc + 1) as u16; 
    self.registers.pc += 2;
    (upper << 8) | lower
  }

  pub fn call(&mut self, opcode: u8) {
    {
      let opcode = opcode as usize;
      self.table[opcode](&mut self.registers, &mut self.mmu);
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ld_bc_word() {
    let mut mmu = MMU::new();
    mmu.write(0x0000, 0xEF);
    mmu.write(0x0001, 0xBE);

    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x01);

    assert_eq!(wide!(cpu.registers, b, c), 0xBEEF);
  }

  #[test]
  fn test_ld_b_u8() {
    let mut mmu = MMU::new();
    mmu.write(0x0000, 0x69);

    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x06);

    assert_eq!(cpu.registers.b, 0x69);
  }

  #[test]
  fn test_ld_b_c() {

    let mut cpu = CPU { 
      registers: Registers {
        b: 0x42,
        c: 0x69,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x41);

    assert_eq!(cpu.registers.b, 0x69);
  }

  #[test]
  fn test_ld_mem_bc_a() {
    let mut mmu = MMU::new();
    mmu.write(0x0000, 0x34);
    mmu.write(0x0001, 0x12);

    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        a: 0x69,
        b: 0x12,
        c: 0x34,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x02);

    assert_eq!(cpu.mmu.read(0x1234), 0x69);
  }

  #[test]
  fn test_word_reg_inc() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        b: 0x68,
        c: 0xFF,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x03);

    assert_eq!(wide!(cpu.registers, b, c), 0x6900);
  }

  #[test]
  fn test_inc_b() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        b: 0x68,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x04);

    assert_eq!(cpu.registers.b, 0x69);
  }

  #[test]
  fn test_dec_b() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0000,
        b: 0x6A,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x05);

    assert_eq!(cpu.registers.b, 0x69);
  }

  #[test]
  fn test_rlca() {
    let mut cpu = CPU { 
      registers: Registers {
        a: 0b11000000,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x07);

    assert_eq!(cpu.registers.a, 0b10000001, "{:#010b} != {:#010b}", cpu.registers.a, 0b10000001);
  }

  #[test]
  fn test_rla() {
    let mut cpu = CPU { 
      registers: Registers {
        a: 0b11000000,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x17);

    assert_eq!(cpu.registers.a, 0b10000000, "{:#010b} != {:#010b}", cpu.registers.a, 0b10000000);
  }

  #[test]
  fn test_rra() {
    let mut cpu = CPU { 
      registers: Registers {
        a: 0b10000001,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x1F);

    assert_eq!(cpu.registers.a, 0b01000000, "{:#010b} != {:#010b}", cpu.registers.a, 0b01000000);
  }

  #[test]
  fn test_rrca() {
    let mut cpu = CPU { 
      registers: Registers {
        a: 0b10000001,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x0F);

    assert_eq!(cpu.registers.a, 0b11000000, "{:#010b} != {:#010b}", cpu.registers.a, 0b11000000);
  }

  #[test]
  fn test_ld_u16_sp() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x00,
        sp: 0xBEEF,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x08);

    assert_eq!(cpu.mmu.read(0x0000), 0xEF);
    assert_eq!(cpu.mmu.read(0x0001), 0xBE);
  }

  #[test]
  fn test_ld_hl_bc() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x00,
        h: 0x11,
        l: 0x11,
        b: 0xBE,
        c: 0xEF,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x09);

    assert_eq!(cpu.registers.h, 0xD0, "{:#02x} != {:#02x}", cpu.registers.h, 0xD0);
    assert_eq!(cpu.registers.l, 0x00);
  }

  #[test]
  fn test_ld_a_bc_() {
    let mut mmu = MMU::new();
    mmu.write(0x1234, 0x69);

    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x00,
        a: 0,
        b: 0x12,
        c: 0x34,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x0A);

    assert_eq!(cpu.registers.a, 0x69, "{:#02x} != {:#02x}", cpu.registers.a, 0x69);
  }

  #[test]
  fn test_dec_bc() {
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x00,
        a: 0,
        b: 0xA2,
        c: 0x00,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x0B);

    assert_eq!(cpu.registers.b, 0xA1, "{:#02x} != {:#02x}", cpu.registers.b, 0xA1);
    assert_eq!(cpu.registers.c, 0xFF, "{:#02x} != {:#02x}", cpu.registers.c, 0xFF);
  }

  #[test]
  fn test_jr_i8() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, 0x05);
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0010,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x18);

    assert_eq!(cpu.registers.pc, 0x0016, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0016);
  }

  #[test]
  fn test_jr_i8_negative() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, i8::from(-5) as u8);
    let mut cpu = CPU { 
      registers: Registers {
        pc: 0x0010,
        ..Registers::new()
      },
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x18);

    assert_eq!(cpu.registers.pc, 0x0010 - 5 + 1, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0010 - 5 + 1);
  }

  #[test]
  fn test_jr_nz_i8_zero() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, 0x05);
    let mut registers = Registers {
      pc: 0x0010,
      ..Registers::new()
    };
    registers.zero(true);

    let mut cpu = CPU { 
      registers,
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x20);

    assert_eq!(cpu.registers.pc, 0x0011, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0011);
  }

  #[test]
  fn test_jr_nz_i8_not_zero() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, 0x05);
    let mut registers = Registers {
      pc: 0x0010,
      ..Registers::new()
    };
    registers.zero(false);

    let mut cpu = CPU { 
      registers,
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x20);

    assert_eq!(cpu.registers.pc, 0x0016, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0016);
  }

  #[test]
  fn test_jr_z_i8_zero() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, 0x05);
    let mut registers = Registers {
      pc: 0x0010,
      ..Registers::new()
    };
    registers.zero(true);

    let mut cpu = CPU { 
      registers,
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x28);

    assert_eq!(cpu.registers.pc, 0x0016, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0016);
  }

  #[test]
  fn test_jr_z_i8_not_zero() {
    let mut mmu = MMU::new();
    mmu.write(0x0010, 0x05);
    let mut registers = Registers {
      pc: 0x0010,
      ..Registers::new()
    };
    registers.zero(false);

    let mut cpu = CPU { 
      registers,
      mmu,
      ..CPU::new()
    }; 

    cpu.call(0x28);

    assert_eq!(cpu.registers.pc, 0x0011, "{:#04x} != {:#04x}", cpu.registers.pc, 0x0011);
  }

  #[test]
  fn test_cpl() {
    let mut cpu = CPU { 
      registers: Registers {
        a: 0b10101010,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x2F);

    assert_eq!(cpu.registers.a, 0b01010101);
  }

  #[test]
  fn test_ccf() {
    let mut registers = Registers::new();
    registers.carry(true);
    let mut cpu = CPU { 
      registers,
      ..CPU::new()
    }; 

    cpu.call(0x3F);

    assert_eq!(cpu.registers.get_carry(), false);
  }

}