use crate::{registers::Registers, mmu::MMU, };


pub struct CPU {
  registers: Registers,
  mmu: MMU,
  table: Vec<fn(&mut Registers, &mut MMU)>,
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
    macro_rules! BC {
        ($registers:expr) => {
          $registers.bc;
        };
        ($registers:expr, $val:expr) => {
          $registers.bc = $val;
        };
    }
    macro_rules! A {
        () => {
          {
            fn read(registers: &Registers) -> u8 {
              registers.a
            }
            read
          }
        };
        ($val:expr) => {
          {
            fn write(registers: &mut Registers) {
              registers.a = $val;
            }
            write
          }
        };
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

    macro_rules! reg {
        ($registers: expr, $reg:ident) => {
          $registers.$reg;
        };
        ($registers:expr, $reg:ident, $val:expr) => {
          $registers.$reg = $val;
        };
    }

    macro_rules! I {
      (NOP) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            println!("NOP");
          }
          eval
        }
      };
      (INC_W $dest:ident) => {
        {
          fn eval(registers: &mut Registers, _mmu: &mut MMU) {
            let res = registers.$dest();
            registers.half_carry(res == 0);
            registers.zero(res == 0);
            registers.negative(false);
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
      (LD $dest:ident, u16) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let v = mmu.read_16_bit_immediate(registers.pc);
            registers.$dest(v);
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
      (LD $dest:ident, $src:expr) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {

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
      (LD ($dest:ident), $src:ident) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let dest_address = registers.$dest();
            let src = registers.$src;
            mmu.write(dest_address, src);
          }
          eval
        }
      };
      (RLCA) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let mut a = registers.a as u16;
            a <<= 1;
            registers.zero(false);
            registers.negative(false);
            registers.half_carry(false);
            let half_carry_bit = a >> 8;
            registers.carry(half_carry_bit == 0x01);
            a |= half_carry_bit;
            registers.a = a as u8;
          }
          eval
        }
      };
      (ADD $hi_d:ident $lo_d:ident, $hi_s:ident $lo_s:ident) => {
        {
          fn eval(registers: &mut Registers, _: &mut MMU) {
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
      (LD $dest:ident, ($src_hi:ident $src_lo:ident)) => {
        {
          fn eval(registers: &mut Registers, mmu: &mut MMU) {
            let src_addr = $src();
            let src = mmu.read(src_addr);
          }
          eval
        }
      };
    }

    vec![
      I!(NOP),
      I!(LD set_bc, u16),
      I!(LD (get_bc), a),
      I!(INC_W inc_bc),
      I!(INC b),
      I!(DEC b),
      I!(LD b, u8),
      I!(RLCA),
      I!(LD (u16), SP),
      I!(ADD h l, b c),
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
      if self.table.len() > opcode {
        self.table[opcode](&mut self.registers, &mut self.mmu);
        return;
      }
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

    assert_eq!(cpu.registers.get_bc(), 0xBEEF);
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

    assert_eq!(cpu.registers.get_bc(), 0x6900);
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
        h: 0,
        l: 0,
        b: 0xBE,
        c: 0xEF,
        ..Registers::new()
      },
      ..CPU::new()
    }; 

    cpu.call(0x09);

    assert_eq!(cpu.registers.h, 0xBE, "{:#02x} != {:#02x}", cpu.registers.h, 0xBE);
    assert_eq!(cpu.registers.l, 0xEF);
  }
}