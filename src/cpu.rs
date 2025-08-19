use crate::{cartridge::MBC, interrupts::Interrupts, mmu::MMU, registers::Registers};

pub enum Interrupt {
    VBlank = 0x40isize,
    LCD = 0x48isize,
    Timer = 0x50isize,
    Joypad = 0x60isize,
}

pub struct CPU<'a, T>
where
    T: MBC,
{
    pub registers: Registers,
    pub mmu: &'a mut MMU<'a, T>,
    table: Vec<fn(&mut CPU<T>)>,
    extended_table: Vec<fn(&mut CPU<T>)>,
    interrupt_enabled: bool,
    prefix_mode: bool,
    pub ticks: u64,
    interrupts: Interrupts,
}

macro_rules! wide {
    ($reg:expr, $hi:ident, $lo:ident) => {
        (($reg.$hi as u16) << 8) | ($reg.$lo as u16)
    };
    ($reg:expr, $hi:ident, $lo:ident, $v:expr, $cpu:expr) => {
        $reg.$hi = ($v >> 8) as u8;
        $reg.$lo = $v as u8;
        $cpu.ticks += 4;
    };
}

impl<'a, T: MBC> CPU<'a, T> {
    pub fn toggle_interrupts(&mut self, enabled: bool) {
        self.interrupt_enabled = enabled;
    }

    pub fn new(mmu: &'a mut MMU<'a, T>) -> CPU<'a, T> {
        CPU {
            registers: Registers::new(),
            mmu,
            table: CPU::build(),
            extended_table: CPU::build_extended_table(),
            interrupt_enabled: true,
            prefix_mode: false,
            ticks: 0,
            interrupts: Interrupts::new(),
        }
    }

    pub fn exec_next_instruction(&mut self) -> u64 {
        self.ticks = 0;
        let pc = self.registers.pc;
        let byte_code = self.read_mem(pc);
        self.call(byte_code);
        self.ticks
    }

    pub fn request_interrupt(&self, interrupt: Interrupt) {
        if !self.interrupt_enabled {
            return;
        }

        // match interrupt {
        //   Interrupt::Joypad
        //     Interrupt::VBlank => todo!(),
        //     Interrupt::LCD => todo!(),
        //     Interrupt::Timer => todo!(),
        // }
    }

    fn push_pc(&mut self) {
        let mut sp = self.registers.sp;
        let pc = self.registers.pc;
        let p = (pc >> 8) as u8;
        let c = pc as u8;
        sp -= 1;
        self.write_mem(sp, p);
        sp -= 1;
        self.write_mem(sp, c);
        self.registers.sp = sp;
    }

    pub fn read_mem(&mut self, address: u16) -> u8 {
        self.ticks += 4;
        self.mmu.read(address)
    }

    pub fn read_mem_16(&mut self, address: u16) -> u16 {
        self.ticks += 8;
        let lower = self.mmu.read(address) as u16;
        let upper = self.mmu.read(address + 1) as u16;
        (upper << 8) | lower
    }

    pub fn write_mem(&mut self, address: u16, value: u8) {
        self.ticks += 4;
        self.mmu.write(address, value);
    }

    pub fn build_extended_table() -> Vec<fn(&mut CPU<T>)> {
        macro_rules! I {
            (RLC $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg as u16;
                    dest <<= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = dest >> 8;
                    cpu.registers.carry(carry_bit == 0x01);
                    dest |= carry_bit;
                    let dest = dest as u8;
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (RLC (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address) as u16;
                    dest <<= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = dest >> 8;
                    cpu.registers.carry(carry_bit == 0x01);
                    dest |= carry_bit;
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (RRC $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg as u16;
                    let mut carry_bit = dest & 0x01;
                    cpu.registers.carry(carry_bit == 0x01);
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);

                    carry_bit <<= 7;
                    dest |= carry_bit;
                    let dest = dest as u8;
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (RRC (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address) as u16;
                    let mut carry_bit = dest & 0x01;
                    cpu.registers.carry(carry_bit == 0x01);
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);

                    carry_bit <<= 7;
                    dest |= carry_bit;
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (RL $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg as u16;
                    dest <<= 1;
                    if cpu.registers.get_carry() {
                        dest |= 0x01;
                    }
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = 0x100 & dest;
                    cpu.registers.carry(carry_bit == 0x100);
                    let dest = dest as u8;
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (RL (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address) as u16;
                    dest <<= 1;
                    if cpu.registers.get_carry() {
                        dest |= 0x01;
                    }
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = 0x100 & dest;
                    cpu.registers.carry(carry_bit == 0x100);
                    let carry_bit = carry_bit >> 8;
                    dest |= carry_bit;
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (RR $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg;
                    let carry_bit = dest & 0x01;
                    dest >>= 1;
                    if cpu.registers.get_carry() {
                        dest |= 0x80;
                    }
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry_bit == 0x01);
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (RR (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address);
                    let carry_bit = dest & 0x01;
                    dest >>= 1;
                    if cpu.registers.get_carry() {
                        dest |= 0x80;
                    }
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry_bit == 0x01);
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (SLA $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg as u16;
                    dest <<= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = 0x100 & dest;
                    cpu.registers.carry(carry_bit == 0x100);
                    let dest = dest as u8;
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (SLA (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address) as u16;
                    dest <<= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = 0x100 & dest;
                    cpu.registers.carry(carry_bit == 0x100);
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (SRA $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest = cpu.registers.$reg;
                    let mut dest = dest as i8;
                    let carry = (0x01 & dest) == 0x01;
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry);
                    cpu.registers.$reg = dest as u8;
                }
                eval
            }};
            (SRA (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let dest = cpu.read_mem(dest_address) as u16;
                    let mut dest = dest as i8;
                    let carry = (0x01 & dest) == 0x01;
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry);
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (SRL $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut dest = cpu.registers.$reg;
                    let carry = (0x01 & dest) == 0x01;
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry);
                    cpu.registers.$reg = dest;
                }
                eval
            }};
            (SRL (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, h, l);
                    let mut dest = cpu.read_mem(dest_address);
                    let carry = (0x01 & dest) == 0x01;
                    dest >>= 1;
                    cpu.registers.zero(dest == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry);
                    let dest = dest as u8;
                    cpu.write_mem(dest_address, dest);
                }
                eval
            }};
            (SWAP $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let byte = cpu.registers.$reg;
                    let res = (byte >> 4) | (byte << 4);
                    cpu.registers.zero(res == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(false);
                    cpu.registers.$reg = res;
                }
                eval
            }};
            (SWAP (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let byte = cpu.read_mem(address);
                    let res = (byte >> 4) | (byte << 4);
                    cpu.registers.zero(res == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(false);
                    cpu.write_mem(address, byte);
                }
                eval
            }};
            (BIT $bit:expr, $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let byte = cpu.registers.$reg;
                    let bit_num = $bit as u8;
                    let test_bit = 0x01 << bit_num;
                    let is_set = byte & test_bit;
                    let is_set = is_set == test_bit;
                    cpu.registers.zero(!is_set);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(true);
                }
                eval
            }};
            (BIT $bit:expr, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let byte = cpu.read_mem(address);
                    let bit_num = $bit as u8;
                    let test_bit = 0x01 << bit_num;
                    let is_set = byte & test_bit;
                    let is_set = is_set == test_bit;
                    cpu.registers.zero(!is_set);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(true);
                }
                eval
            }};
            (RES $bit:expr, $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut byte = cpu.registers.$reg;
                    let bit_num = $bit as u8;
                    let bit = 0x01 << bit_num;
                    let mask = 0xFF ^ bit;
                    byte &= mask;
                    cpu.registers.$reg = byte;
                }
                eval
            }};
            (RES $bit:expr, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let mut byte = cpu.read_mem(address);
                    let bit_num = $bit as u8;
                    let bit = 0x01 << bit_num;
                    let mask = 0xFF ^ bit;
                    byte &= mask;
                    cpu.write_mem(address, byte);
                }
                eval
            }};
            (SET $bit:expr, $reg:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut byte = cpu.registers.$reg;
                    let bit_num = $bit as u8;
                    let bit = 0x01 << bit_num;
                    byte |= bit;
                    cpu.registers.$reg = byte;
                }
                eval
            }};
            (SET $bit:expr, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let mut byte = cpu.read_mem(address);
                    let bit_num = $bit as u8;
                    let bit = 0x01 << bit_num;
                    byte |= bit;
                    cpu.write_mem(address, byte);
                }
                eval
            }};
        }
        vec![
            I!(RLC b),
            I!(RLC c),
            I!(RLC d),
            I!(RLC e),
            I!(RLC h),
            I!(RLC l),
            I!(RLC(HL)),
            I!(RLC a),
            I!(RRC b),
            I!(RRC c),
            I!(RRC d),
            I!(RRC e),
            I!(RRC h),
            I!(RRC l),
            I!(RRC(HL)),
            I!(RRC a),
            /* */
            I!(RL b),
            I!(RL c),
            I!(RL d),
            I!(RL e),
            I!(RL h),
            I!(RL l),
            I!(RL(HL)),
            I!(RL a),
            I!(RR b),
            I!(RR c),
            I!(RR d),
            I!(RR e),
            I!(RR h),
            I!(RR l),
            I!(RR(HL)),
            I!(RR a),
            /* */
            I!(SLA b),
            I!(SLA c),
            I!(SLA d),
            I!(SLA e),
            I!(SLA h),
            I!(SLA l),
            I!(SLA(HL)),
            I!(SLA a),
            I!(SRA b),
            I!(SRA c),
            I!(SRA d),
            I!(SRA e),
            I!(SRA h),
            I!(SRA l),
            I!(SRA(HL)),
            I!(SRA a),
            /* ROW */
            I!(SWAP b),
            I!(SWAP c),
            I!(SWAP d),
            I!(SWAP e),
            I!(SWAP h),
            I!(SWAP l),
            I!(SWAP(HL)),
            I!(SWAP a),
            I!(SRL b),
            I!(SRL c),
            I!(SRL d),
            I!(SRL e),
            I!(SRL h),
            I!(SRL l),
            I!(SRL(HL)),
            I!(SRL a),
            /* ROW */
            I!(BIT 0, b),
            I!(BIT 0, c),
            I!(BIT 0, d),
            I!(BIT 0, e),
            I!(BIT 0, h),
            I!(BIT 0, l),
            I!(BIT 0, (HL)),
            I!(BIT 0, a),
            I!(BIT 1, b),
            I!(BIT 1, c),
            I!(BIT 1, d),
            I!(BIT 1, e),
            I!(BIT 1, h),
            I!(BIT 1, l),
            I!(BIT 1, (HL)),
            I!(BIT 1, a),
            /* ROW */
            I!(BIT 2, b),
            I!(BIT 2, c),
            I!(BIT 2, d),
            I!(BIT 2, e),
            I!(BIT 2, h),
            I!(BIT 2, l),
            I!(BIT 2, (HL)),
            I!(BIT 2, a),
            I!(BIT 3, b),
            I!(BIT 3, c),
            I!(BIT 3, d),
            I!(BIT 3, e),
            I!(BIT 3, h),
            I!(BIT 3, l),
            I!(BIT 3, (HL)),
            I!(BIT 3, a),
            /* ROW */
            I!(BIT 4, b),
            I!(BIT 4, c),
            I!(BIT 4, d),
            I!(BIT 4, e),
            I!(BIT 4, h),
            I!(BIT 4, l),
            I!(BIT 4, (HL)),
            I!(BIT 4, a),
            I!(BIT 5, b),
            I!(BIT 5, c),
            I!(BIT 5, d),
            I!(BIT 5, e),
            I!(BIT 5, h),
            I!(BIT 5, l),
            I!(BIT 5, (HL)),
            I!(BIT 5, a),
            /* ROW */
            I!(BIT 6, b),
            I!(BIT 6, c),
            I!(BIT 6, d),
            I!(BIT 6, e),
            I!(BIT 6, h),
            I!(BIT 6, l),
            I!(BIT 6, (HL)),
            I!(BIT 6, a),
            I!(BIT 7, b),
            I!(BIT 7, c),
            I!(BIT 7, d),
            I!(BIT 7, e),
            I!(BIT 7, h),
            I!(BIT 7, l),
            I!(BIT 7, (HL)),
            I!(BIT 7, a),
            /* ROW */
            I!(RES 0, b),
            I!(RES 0, c),
            I!(RES 0, d),
            I!(RES 0, e),
            I!(RES 0, h),
            I!(RES 0, l),
            I!(RES 0, (HL)),
            I!(RES 0, a),
            I!(RES 1, b),
            I!(RES 1, c),
            I!(RES 1, d),
            I!(RES 1, e),
            I!(RES 1, h),
            I!(RES 1, l),
            I!(RES 1, (HL)),
            I!(RES 1, a),
            /* ROW */
            I!(RES 2, b),
            I!(RES 2, c),
            I!(RES 2, d),
            I!(RES 2, e),
            I!(RES 2, h),
            I!(RES 2, l),
            I!(RES 2, (HL)),
            I!(RES 2, a),
            I!(RES 3, b),
            I!(RES 3, c),
            I!(RES 3, d),
            I!(RES 3, e),
            I!(RES 3, h),
            I!(RES 3, l),
            I!(RES 3, (HL)),
            I!(RES 3, a),
            /* ROW */
            I!(RES 4, b),
            I!(RES 4, c),
            I!(RES 4, d),
            I!(RES 4, e),
            I!(RES 4, h),
            I!(RES 4, l),
            I!(RES 4, (HL)),
            I!(RES 4, a),
            I!(RES 5, b),
            I!(RES 5, c),
            I!(RES 5, d),
            I!(RES 5, e),
            I!(RES 5, h),
            I!(RES 5, l),
            I!(RES 5, (HL)),
            I!(RES 5, a),
            /* ROW */
            I!(RES 6, b),
            I!(RES 6, c),
            I!(RES 6, d),
            I!(RES 6, e),
            I!(RES 6, h),
            I!(RES 6, l),
            I!(RES 6, (HL)),
            I!(RES 6, a),
            I!(RES 7, b),
            I!(RES 7, c),
            I!(RES 7, d),
            I!(RES 7, e),
            I!(RES 7, h),
            I!(RES 7, l),
            I!(RES 7, (HL)),
            I!(RES 7, a),
            /* ROW */
            I!(SET 0, b),
            I!(SET 0, c),
            I!(SET 0, d),
            I!(SET 0, e),
            I!(SET 0, h),
            I!(SET 0, l),
            I!(SET 0, (HL)),
            I!(SET 0, a),
            I!(SET 1, b),
            I!(SET 1, c),
            I!(SET 1, d),
            I!(SET 1, e),
            I!(SET 1, h),
            I!(SET 1, l),
            I!(SET 1, (HL)),
            I!(SET 1, a),
            /* ROW */
            I!(SET 2, b),
            I!(SET 2, c),
            I!(SET 2, d),
            I!(SET 2, e),
            I!(SET 2, h),
            I!(SET 2, l),
            I!(SET 2, (HL)),
            I!(SET 2, a),
            I!(SET 3, b),
            I!(SET 3, c),
            I!(SET 3, d),
            I!(SET 3, e),
            I!(SET 3, h),
            I!(SET 3, l),
            I!(SET 3, (HL)),
            I!(SET 3, a),
            /* ROW */
            I!(SET 4, b),
            I!(SET 4, c),
            I!(SET 4, d),
            I!(SET 4, e),
            I!(SET 4, h),
            I!(SET 4, l),
            I!(SET 4, (HL)),
            I!(SET 4, a),
            I!(SET 5, b),
            I!(SET 5, c),
            I!(SET 5, d),
            I!(SET 5, e),
            I!(SET 5, h),
            I!(SET 5, l),
            I!(SET 5, (HL)),
            I!(SET 5, a),
            /* ROW */
            I!(SET 6, b),
            I!(SET 6, c),
            I!(SET 6, d),
            I!(SET 6, e),
            I!(SET 6, h),
            I!(SET 6, l),
            I!(SET 6, (HL)),
            I!(SET 6, a),
            I!(SET 7, b),
            I!(SET 7, c),
            I!(SET 7, d),
            I!(SET 7, e),
            I!(SET 7, h),
            I!(SET 7, l),
            I!(SET 7, (HL)),
            I!(SET 7, a),
        ]
    }

    pub fn build() -> Vec<fn(&mut CPU<T>)> {
        fn sub_8_flags(dest: u8, src: u8) -> (u8, bool, bool) {
            let (res, carry) = dest.overflowing_sub(src);
            let (_, half_carry) = (0x0F & dest).overflowing_sub(0x0F & src);

            (res, carry, half_carry)
        }

        fn add_8_flags(dest: u8, src: u8) -> (u8, bool, bool) {
            let (res, carry) = src.overflowing_add(dest);
            let (_, half_carry) = (0xF0 | src).overflowing_add(0x0F & dest);

            let res = res as u8;
            (res, carry, half_carry)
        }

        fn sbc_8(registers: &mut Registers, dest: u8, src: u8) -> u8 {
            let carry = (registers.f >> 4) & 0x01;
            let (src, _) = src.overflowing_add(carry);
            let (res, carry) = dest.overflowing_sub(src);

            let (_, half_carry) = (0x0F & dest).overflowing_sub(src);

            registers.negative(true);
            registers.half_carry(!half_carry);
            registers.carry(!carry);
            let res = res as u8;
            registers.zero(res == 0);
            res
        }

        fn adc_8(registers: &mut Registers, dest: u8, src: u8) -> u8 {
            let carry = (registers.f >> 4) & 0x01;
            let carry = carry;
            let (res, carry_1) = src.overflowing_add(carry);
            let (res, carry_2) = res.overflowing_add(dest);
            let (half_sum, half_carry_1) = (0xF0 | src).overflowing_add(0x0F & carry);
            let (_, half_carry_2) = half_sum.overflowing_add(0x0F & dest);
            let half_carry = half_carry_1 || half_carry_2;
            let carry = carry_1 || carry_2;

            registers.negative(false);
            registers.half_carry(half_carry);
            registers.carry(carry);
            let res = res as u8;
            registers.zero(res == 0);
            res
        }

        fn or_8(registers: &mut Registers, dest: u8, src: u8) -> u8 {
            let res = dest | src;

            registers.zero(res == 0);
            registers.negative(false);
            registers.half_carry(false);
            registers.carry(false);
            res
        }

        macro_rules! I {
            (NOP) => {{
                fn eval<T>(_cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    println!("NOP");
                }
                eval
            }};
            (INC [$dest_hi:ident $dest_lo:ident]) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let res = wide!(cpu.registers, $dest_hi, $dest_lo);
                    let (res, _) = res.overflowing_add(1);
                    wide!(cpu.registers, $dest_hi, $dest_lo, res, cpu);
                }
                eval
            }};
            (INC $dest:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let res = cpu.registers.$dest;
                    let (res, _, half_carry) = add_8_flags(res, 1);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.zero(res == 0);
                    cpu.registers.$dest = res;
                }
                eval
            }};
            (INC (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let res = cpu.read_mem(address);
                    let (res, _, half_carry) = add_8_flags(res, 1);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.zero(res == 0);
                    cpu.write_mem(address, res);
                }
                eval
            }};
            (DEC $dest: ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let res: u8 = cpu.registers.$dest;
                    cpu.registers.half_carry(res & 0x0F == 0);
                    cpu.registers.negative(true);
                    let (res, _) = res.overflowing_sub(1);
                    cpu.registers.$dest = res;
                    cpu.registers.zero(cpu.registers.$dest == 0);
                }
                eval
            }};
            (DEC (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let res = cpu.read_mem(address);
                    cpu.registers.half_carry(res & 0x0F == 0);
                    cpu.registers.zero(res == 0x01);
                    cpu.registers.negative(true);
                    let (res, _) = res.overflowing_sub(1);
                    cpu.write_mem(address, res);
                }
                eval
            }};
            (LD SP, HL) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    wide!(cpu.registers, s, p, hl, cpu);
                }
                eval
            }};
            (LD [$dest_hi:ident $dest_lo:ident], u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let v = cpu.read_mem_16(cpu.registers.pc);
                    cpu.registers.pc += 2;
                    wide!(cpu.registers, $dest_hi, $dest_lo, v, cpu);
                    cpu.ticks -= 4; // compensate for overlapping register write w/ mem read
                }
                eval
            }};
            (LD sp, u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let v = cpu.read_mem_16(cpu.registers.pc);
                    cpu.registers.pc += 2;
                    wide!(cpu.registers, s, p, v, cpu);
                }
                eval
            }};
            (LD $dest:ident, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let v = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    cpu.registers.$dest = v;
                }
                eval
            }};
            (LD $dest:ident, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let address = wide!(cpu.registers, h, l);
                    let v = cpu.read_mem(address);
                    cpu.registers.pc += 1;
                    cpu.registers.$dest = v;
                }
                eval
            }};
            (LD $dest:ident, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.registers.$dest = cpu.registers.$src;
                }
                eval
            }};
            (LD (u16), SP) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = cpu.read_mem_16(cpu.registers.pc);
                    cpu.registers.pc += 2;
                    let sp = wide!(cpu.registers, s, p);
                    let lower = (sp & 0x00FF) as u8;
                    let upper = (sp >> 8) as u8;
                    cpu.write_mem(dest_address, lower);
                    cpu.write_mem(dest_address + 1, upper);
                }
                eval
            }};
            (LD A, (u16)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src_address = cpu.read_mem_16(cpu.registers.pc);
                    cpu.registers.pc += 2;
                    let src = cpu.read_mem(src_address);
                    cpu.registers.a = src;
                }
                eval
            }};
            (LD (u16), A) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = cpu.read_mem_16(cpu.registers.pc);
                    cpu.registers.pc += 2;
                    let src = cpu.registers.a;
                    cpu.write_mem(dest_address, src);
                }
                eval
            }};
            (LD (HL), u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.pc;
                    let src = cpu.read_mem(src);
                    cpu.registers.pc += 1;
                    let dest_address = wide!(cpu.registers, h, l);
                    cpu.write_mem(dest_address, src);
                }
                eval
            }};
            (LD (FF00+u8), A) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.a;
                    let immed = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let immed = immed as u16;
                    let dest_address = 0xFF00 | immed;
                    cpu.write_mem(dest_address, src);
                }
                eval
            }};
            (LD A, (FF00+u8)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let src = immed as u16;
                    let src = 0xFF00 | src;
                    let src = cpu.read_mem(src);

                    cpu.registers.a = src;
                }
                eval
            }};
            (LD (FF00+C), A) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.a;
                    let dest_address = 0xFF00 | (cpu.registers.c as u16);
                    cpu.write_mem(dest_address, src);
                }
                eval
            }};
            (LD A, (FF00+C)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = 0xFF00 | (cpu.registers.c as u16);
                    let src = cpu.read_mem(src);

                    cpu.registers.a = src;
                }
                eval
            }};
            (LD ([$dest_hi:ident $dest_lo:ident]), $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let dest_address = wide!(cpu.registers, $dest_hi, $dest_lo);
                    let src = cpu.registers.$src;
                    cpu.write_mem(dest_address, src);
                }
                eval
            }};
            (RLCA) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut a = cpu.registers.a as u16;
                    a <<= 1;
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry_bit = a >> 8;
                    cpu.registers.carry(carry_bit == 0x01);
                    a |= carry_bit;
                    cpu.registers.zero(a == 0);
                    cpu.registers.a = a as u8;
                }
                eval
            }};
            (RLA) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut a = cpu.registers.a;
                    let carry_bit = 0x80 & a;
                    a <<= 1;
                    if cpu.registers.get_carry() {
                        a |= 0x01;
                    }
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry_bit == 0x80);
                    cpu.registers.zero(a == 0);
                    cpu.registers.a = a;
                }
                eval
            }};
            (RRA) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut a = cpu.registers.a;
                    let carry_bit = a & 0x01;
                    a >>= 1;
                    if cpu.registers.get_carry() {
                        a |= 0x80;
                    }
                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(carry_bit == 0x01);
                    cpu.registers.a = a;
                }
                eval
            }};
            (RRCA) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut a = cpu.registers.a;
                    let carry_bit = a & 0x01;
                    cpu.registers.carry(carry_bit == 0x01);
                    a >>= 1;
                    a |= (carry_bit << 7);
                    cpu.registers.zero(false);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.a = a;
                }
                eval
            }};
            (ADD [$hi_d:ident $lo_d:ident], [$hi_s:ident $lo_s:ident]) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, $hi_s, $lo_s) as u16;
                    let dest = wide!(cpu.registers, $hi_d, $lo_d) as u16;
                    let (res, carry) = src.overflowing_add(dest);
                    let (_, half_carry) = (0xF000 | src).overflowing_add(0x0FFF & dest);

                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.carry(carry);

                    wide!(cpu.registers, $hi_d, $lo_d, res, cpu);
                }
                eval
            }};
            (ADD SP, i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.read_mem(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let src = src as i8;
                    let src = i16::from(src);
                    let src = src as u16;

                    let sp = wide!(cpu.registers, s, p);
                    let (res, carry) = sp.overflowing_add(src);
                    let (_, half_carry) = (sp | 0xF000).overflowing_add(src & 0x0FFF);

                    cpu.registers.zero(false);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.carry(carry);

                    wide!(cpu.registers, s, p, res, cpu);
                    cpu.ticks += 4;
                }
                eval
            }};
            (LD HL, SP+i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.read_mem(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let src = src as i8;
                    let src = i16::from(src);
                    let src = src as u16;

                    let sp = wide!(cpu.registers, s, p);
                    let (res, carry) = sp.overflowing_add(src);
                    let (_, half_carry) = (sp | 0xF000).overflowing_add(src & 0x0FFF);

                    cpu.registers.zero(false);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.carry(carry);

                    wide!(cpu.registers, h, l, res, cpu);
                }
                eval
            }};
            (ADD $dest:ident, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let dest = cpu.registers.$dest as u8;
                    let (res, carry, half_carry) = add_8_flags(dest, src);
                    cpu.registers.carry(carry);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.zero(res == 0);
                    cpu.registers.negative(false);

                    cpu.registers.$dest = res;
                }
                eval
            }};
            (ADD $dest:ident, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src as u8;
                    let dest = cpu.registers.$dest as u8;
                    let (res, carry, half_carry) = add_8_flags(dest, src);

                    cpu.registers.negative(false);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.carry(carry);
                    cpu.registers.zero(res == 0);
                    cpu.registers.$dest = res;
                }
                eval
            }};
            (SUB A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.read_mem(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let dest = cpu.registers.a;
                    let (res, carry, half_carry) = sub_8_flags(dest, src);
                    cpu.registers.carry(carry);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.negative(true);
                    cpu.registers.zero(res == 0);

                    cpu.registers.a = res;
                }
                eval
            }};
            (SUB A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src as u8;
                    let dest = cpu.registers.a;
                    let (res, carry, half_carry) = sub_8_flags(dest, src);
                    cpu.registers.carry(carry);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.negative(true);
                    cpu.registers.zero(res == 0);

                    cpu.registers.a = res;
                }
                eval
            }};
            (SUB A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.a;
                    let (res, carry, half_carry) = sub_8_flags(dest, src);
                    cpu.registers.carry(carry);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.negative(true);
                    cpu.registers.zero(res == 0);
                    cpu.registers.a = res;
                }
                eval
            }};
            (SBC A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.read_mem(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let dest = cpu.registers.a;
                    let res = sbc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (SBC A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src;
                    let dest = cpu.registers.a;
                    let res = sbc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (SBC A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.a;
                    let res = sbc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (ADC A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.read_mem(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let dest = cpu.registers.a;
                    let res = adc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (ADC A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src;
                    let dest = cpu.registers.a;
                    let res = adc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (ADD $dest:ident, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.$dest;
                    let (res, carry, half_carry) = add_8_flags(dest, src);
                    cpu.registers.zero(res == 0);
                    cpu.registers.carry(carry);
                    cpu.registers.half_carry(half_carry);
                    cpu.registers.$dest = res;
                }
                eval
            }};
            (ADC A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.a;
                    let res = adc_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (AND A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let mut a = cpu.registers.a;
                    a &= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(true);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (AND A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src as u8;
                    let mut a = cpu.registers.a;
                    a &= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(true);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (AND A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let mut a = cpu.registers.a;
                    a &= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(true);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (XOR A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let mut a = cpu.registers.a;
                    a ^= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (XOR A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src as u8;
                    let mut a = cpu.registers.a;
                    a ^= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (XOR A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let mut a = cpu.registers.a;
                    a ^= src;
                    cpu.registers.a = a;

                    cpu.registers.zero(a == 0);
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(false);
                }
                eval
            }};
            (OR A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;

                    let dest = cpu.registers.a;
                    let res = or_8(&mut cpu.registers, dest, src);

                    cpu.registers.a = res;
                }
                eval
            }};
            (OR A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src as u8;
                    let dest = cpu.registers.a;
                    let res = or_8(&mut cpu.registers, dest, src);

                    cpu.registers.a = res;
                }
                eval
            }};
            (OR A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.a;
                    let res = or_8(&mut cpu.registers, dest, src);
                    cpu.registers.a = res;
                }
                eval
            }};
            (CP A, u8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.mmu.read(cpu.registers.pc);
                    cpu.registers.pc += 1;
                    let dest = cpu.registers.a;
                    let (res, carry) = dest.overflowing_sub(src);
                    let (_, half_carry) = (0x0F & dest).overflowing_sub(0x0F & src);

                    cpu.registers.negative(true);
                    cpu.registers.half_carry(!half_carry);
                    cpu.registers.carry(!carry);
                    cpu.registers.zero(res == 0);
                }
                eval
            }};
            (CP A, $src:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = cpu.registers.$src;
                    let dest = cpu.registers.a;
                    let (res, carry) = dest.overflowing_sub(src);
                    let (_, half_carry) = (0x0F & dest).overflowing_sub(0x0F & src);

                    cpu.registers.negative(true);
                    cpu.registers.half_carry(!half_carry);
                    cpu.registers.carry(!carry);
                    let res = res as u8;
                    cpu.registers.zero(res == 0);
                }
                eval
            }};
            (CP A, (HL)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src = wide!(cpu.registers, h, l);
                    let src = cpu.read_mem(src);
                    let dest = cpu.registers.a;
                    let (res, carry) = dest.overflowing_sub(src);
                    let (_, half_carry) = (0x0F & dest).overflowing_sub(0x0F & src);

                    cpu.registers.negative(true);
                    cpu.registers.half_carry(!half_carry);
                    cpu.registers.carry(!carry);
                    cpu.registers.zero(res == 0);
                }
                eval
            }};
            (LD $dest:ident, ([$src_hi:ident $src_lo:ident])) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let src_addr = wide!(cpu.registers, $src_hi, $src_lo);
                    let src = cpu.read_mem(src_addr);
                    cpu.registers.$dest = src;
                }
                eval
            }};
            (DEC [$src_hi:ident $src_lo:ident]) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let res = wide!(cpu.registers, b, c);
                    let (res, _) = res.overflowing_sub(1);
                    wide!(cpu.registers, b, c, res, cpu);
                }
                eval
            }};
            (STOP) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    println!("STOP");
                }
                eval
            }};
            (JR i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.read_mem(cpu.registers.pc) as i8;
                    cpu.registers.pc += 1;
                    let immed = i16::from(immed);
                    let signed_pc = cpu.registers.pc as i16;
                    let new_pc = signed_pc + immed;
                    cpu.registers.pc = (new_pc as u16);
                    cpu.ticks += 4;
                }
                eval
            }};
            (JR NZ, i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.read_mem(cpu.registers.pc) as i8;
                    cpu.registers.pc += 1;
                    if cpu.registers.get_zero() {
                        return;
                    }

                    let immed = i16::from(immed);
                    let signed_pc = cpu.registers.pc as i16;
                    let new_pc = signed_pc + immed;
                    cpu.registers.pc = (new_pc as u16);
                    cpu.ticks += 4;
                }
                eval
            }};
            (JR NC, i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.read_mem(cpu.registers.pc) as i8;
                    cpu.registers.pc += 1;
                    if cpu.registers.get_carry() {
                        return;
                    }

                    let immed = i16::from(immed);
                    let signed_pc = cpu.registers.pc as i16;
                    let new_pc = signed_pc + immed;
                    cpu.registers.pc = (new_pc as u16);
                    cpu.ticks += 4;
                }
                eval
            }};
            (JR Z, i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.read_mem(cpu.registers.pc) as i8;
                    cpu.registers.pc += 1;
                    let zero = cpu.registers.get_zero();
                    if !zero {
                        return;
                    }

                    let immed = i16::from(immed);
                    let signed_pc = cpu.registers.pc as i16;
                    let new_pc = signed_pc + immed;
                    cpu.registers.pc = (new_pc as u16);
                    cpu.ticks += 4;
                }
                eval
            }};
            (JR C, i8) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let immed = cpu.mmu.read(cpu.registers.pc) as i8;
                    cpu.registers.pc += 1;
                    let carry = cpu.registers.get_carry();
                    if carry {
                        return;
                    }

                    let immed = i16::from(immed);
                    let signed_pc = cpu.registers.pc as i16;
                    let new_pc = signed_pc + immed;
                    cpu.registers.pc = (new_pc as u16);
                    cpu.ticks += 4;
                }
                eval
            }};
            (LD (HL+), a) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    let a = cpu.registers.a;
                    cpu.write_mem(hl, a);
                    wide!(cpu.registers, h, l, hl + 1, cpu);
                }
                eval
            }};
            (LD (HL-), a) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    let a = cpu.registers.a;
                    cpu.write_mem(hl, a);
                    wide!(cpu.registers, h, l, hl - 1, cpu);
                }
                eval
            }};
            (LD a, (HL+)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    let v = cpu.read_mem(hl);
                    cpu.registers.a = v;
                    wide!(cpu.registers, h, l, hl + 1, cpu);
                }
                eval
            }};
            (LD a, (HL-)) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    let v = cpu.read_mem(hl);
                    cpu.registers.a = v;
                    wide!(cpu.registers, h, l, hl - 1, cpu);
                }
                eval
            }};
            (DAA) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut a = u16::from(cpu.registers.a);
                    let mut should_carry = false;
                    if !cpu.registers.get_negative() {
                        if cpu.registers.get_half_carry() || (a & 0x0F) >= 0x0A {
                            a += 0x06;
                        }
                        if cpu.registers.get_carry() || a >= 0xA0 {
                            a += 0x60;
                            should_carry = true;
                        }
                    } else {
                        if cpu.registers.get_half_carry() {
                            let (res, _) = a.overflowing_sub(0x06);
                            a = res & 0xFF;
                        }

                        if cpu.registers.get_carry() {
                            let (res, _) = a.overflowing_sub(0x60);
                            a = res & 0xFF;
                            should_carry = true;
                        }
                    }

                    cpu.registers.half_carry(false);
                    cpu.registers.carry(should_carry);
                    a &= 0xFF;
                    cpu.registers.zero(a == 0);
                    cpu.registers.a = a as u8;
                }
                eval
            }};
            (HALT) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                }
                eval
            }};
            (CPL) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.registers.a = !cpu.registers.a;
                    cpu.registers.negative(true);
                    cpu.registers.half_carry(true);
                }
                eval
            }};
            (SCF) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    cpu.registers.carry(true);
                }
                eval
            }};
            (CCF) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.registers.negative(false);
                    cpu.registers.half_carry(false);
                    let carry = cpu.registers.get_carry();
                    cpu.registers.carry(!carry);
                }
                eval
            }};
            (POP [$src_hi:ident $src_lo:ident]) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut sp = wide!(cpu.registers, s, p);
                    let lo = cpu.read_mem(sp);
                    sp += 1;
                    let hi = cpu.read_mem(sp);
                    sp += 1;
                    cpu.registers.$src_hi = hi;
                    cpu.registers.$src_lo = lo;
                    wide!(cpu.registers, s, p, sp, cpu);
                    cpu.ticks -= 4; // compensate for pipelined timing
                }
                eval
            }};
            (PUSH [$src_hi:ident $src_lo:ident]) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut sp = wide!(cpu.registers, s, p);
                    let hi = cpu.registers.$src_hi;
                    let lo = cpu.registers.$src_lo;
                    sp -= 1;
                    cpu.write_mem(sp, hi);
                    sp -= 1;
                    cpu.write_mem(sp, lo);
                    wide!(cpu.registers, s, p, sp, cpu);
                }
                eval
            }};
            (RET) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut sp = wide!(cpu.registers, s, p);
                    let lo = cpu.read_mem(sp) as u16;
                    sp += 1;
                    let hi = cpu.read_mem(sp) as u16;
                    let hi = hi << 8;
                    sp += 1;
                    let pc = hi | lo;

                    cpu.registers.pc = pc;

                    wide!(cpu.registers, s, p, sp, cpu);
                }
                eval
            }};
            (RETI) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut sp = wide!(cpu.registers, s, p);
                    let lo = cpu.read_mem(sp) as u16;
                    sp += 1;
                    let hi = cpu.read_mem(sp) as u16;
                    let hi = hi << 8;
                    sp += 1;
                    let pc = hi | lo;

                    cpu.registers.pc = pc;

                    wide!(cpu.registers, s, p, sp, cpu);

                    cpu.toggle_interrupts(true);
                }
                eval
            }};
            (EI) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.toggle_interrupts(true);
                }
                eval
            }};
            (DI) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.toggle_interrupts(false);
                }
                eval
            }};
            (RET $condition:ident) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    if (!cpu.registers.$condition()) {
                        cpu.ticks += 4;
                        return;
                    }

                    let mut sp = wide!(cpu.registers, s, p);
                    let lo = cpu.read_mem(sp) as u16;
                    sp += 1;
                    let hi = cpu.read_mem(sp) as u16;
                    let hi = hi << 8;
                    sp += 1;
                    let pc = hi | lo;

                    cpu.registers.pc = pc;
                    cpu.ticks += 4;
                    wide!(cpu.registers, s, p, sp, cpu);
                }
                eval
            }};
            (JP $condition:ident, u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    if (!cpu.registers.$condition()) {
                        // skip past the immediate
                        cpu.registers.pc += 2;
                        cpu.ticks += 8;
                        return;
                    }

                    let mut pc = cpu.registers.pc;
                    let lo = cpu.read_mem(pc) as u16;
                    pc += 1;
                    let mut hi = cpu.read_mem(pc) as u16;
                    hi = hi << 8;
                    let pc = hi | lo;

                    cpu.registers.pc = pc;
                    cpu.ticks += 4;
                }
                eval
            }};
            (CALL u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    // read immediate into call_address
                    let mut pc = cpu.registers.pc;
                    let lo = cpu.read_mem(pc) as u16;
                    pc += 1;
                    let mut hi = cpu.read_mem(pc) as u16;
                    pc += 1;
                    hi = hi << 8;
                    let call_address = hi | lo;
                    cpu.registers.pc = call_address;

                    // push pc onto the stack
                    let mut sp = wide!(cpu.registers, s, p);
                    sp -= 1;
                    let hi = (pc >> 8) as u8;
                    cpu.write_mem(sp, hi);
                    sp -= 1;
                    let lo = pc as u8;
                    cpu.write_mem(sp, lo);

                    wide!(cpu.registers, s, p, sp, cpu);
                }
                eval
            }};
            (CALL $condition:ident, u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    if (!cpu.registers.$condition()) {
                        // skip past the immediate
                        cpu.registers.pc += 2;
                        return;
                    }

                    // read immediate into call_address
                    let mut pc = cpu.registers.pc;
                    let lo = cpu.read_mem(pc) as u16;
                    pc += 1;
                    let mut hi = cpu.read_mem(pc) as u16;
                    hi = hi << 8;
                    let call_address = hi | lo;
                    cpu.registers.pc = call_address;

                    // push pc onto the stack
                    let mut sp = wide!(cpu.registers, s, p);
                    sp -= 1;
                    let hi = (pc >> 8) as u8;
                    cpu.write_mem(sp, hi);
                    sp -= 1;
                    let lo = pc as u8;
                    cpu.write_mem(sp, lo);

                    wide!(cpu.registers, s, p, sp, cpu);
                }
                eval
            }};
            (JP u16) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let mut pc = cpu.registers.pc;
                    let lo = cpu.read_mem(pc) as u16;
                    pc += 1;
                    let mut hi = cpu.read_mem(pc) as u16;
                    hi = hi << 8;
                    let pc = hi | lo;

                    cpu.registers.pc = pc;
                    cpu.ticks += 4
                }
                eval
            }};
            (JP HL) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let hl = wide!(cpu.registers, h, l);
                    cpu.registers.pc = hl;
                }
                eval
            }};
            (RST $nn:expr) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    let pc = cpu.registers.pc;
                    let mut sp = wide!(cpu.registers, s, p);
                    sp -= 1;
                    let hi = (pc >> 8) as u8;
                    cpu.write_mem(sp, hi);
                    sp -= 1;
                    let lo = pc as u8;
                    cpu.write_mem(sp, lo);

                    wide!(cpu.registers, s, p, sp, cpu);

                    let address = $nn;
                    let address = address as u16;
                    cpu.registers.pc = address;
                }
                eval
            }};
            (PREFIX CB) => {{
                fn eval<T>(cpu: &mut CPU<T>)
                where
                    T: MBC,
                {
                    cpu.prefix_mode = true;
                }
                eval
            }};
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
            I!(LD(u16), SP),
            I!(ADD [h l], [b c]),
            I!(LD a, ([b c])),
            I!(DEC [b c]),
            I!(INC c),
            I!(DEC c),
            I!(LD c, u8),
            I!(RRCA),
            /* ROW */
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
            /* ROW */
            I!(JR NZ, i8),
            I!(LD [h l], u16),
            I!(LD(HL+), a),
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
            /* ROW */
            I!(JR NC, i8),
            I!(LD sp, u16),
            I!(LD (HL-), a),
            I!(INC [s p]),
            I!(INC(HL)),
            I!(DEC(HL)),
            I!(LD(HL), u8),
            I!(SCF),
            I!(JR C, i8),
            I!(ADD [h l], [s p]),
            I!(LD a, (HL-)),
            I!(DEC [s p]),
            I!(INC a),
            I!(DEC a),
            I!(LD a, u8),
            I!(CCF),
            /* ROW */
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
            /* ROW */
            I!(LD d, b),
            I!(LD d, c),
            I!(LD d, d),
            I!(LD d, e),
            I!(LD d, h),
            I!(LD d, l),
            I!(LD d, (HL)),
            I!(LD d, a),
            I!(LD e, b),
            I!(LD e, c),
            I!(LD e, d),
            I!(LD e, e),
            I!(LD e, h),
            I!(LD e, l),
            I!(LD e, (HL)),
            I!(LD e, a),
            /* ROW */
            I!(LD h, b),
            I!(LD h, c),
            I!(LD h, d),
            I!(LD h, e),
            I!(LD h, h),
            I!(LD h, l),
            I!(LD h, (HL)),
            I!(LD h, a),
            I!(LD l, b),
            I!(LD l, c),
            I!(LD l, d),
            I!(LD l, e),
            I!(LD l, h),
            I!(LD l, l),
            I!(LD l, (HL)),
            I!(LD l, a),
            /* ROW */
            I!(LD ([h l]), b),
            I!(LD ([h l]), c),
            I!(LD ([h l]), d),
            I!(LD ([h l]), e),
            I!(LD ([h l]), h),
            I!(LD ([h l]), l),
            I!(HALT),
            I!(LD ([h l]), a),
            I!(LD a, b),
            I!(LD a, c),
            I!(LD a, d),
            I!(LD a, e),
            I!(LD a, h),
            I!(LD a, l),
            I!(LD a, (HL)),
            I!(LD a, a),
            /* ROW */
            I!(ADD a, b),
            I!(ADD a, c),
            I!(ADD a, d),
            I!(ADD a, e),
            I!(ADD a, h),
            I!(ADD a, l),
            I!(ADD a, (HL)),
            I!(ADD a, a),
            I!(ADC A, b),
            I!(ADC A, c),
            I!(ADC A, d),
            I!(ADC A, e),
            I!(ADC A, h),
            I!(ADC A, l),
            I!(ADC A, (HL)),
            I!(ADC A, a),
            /* ROW */
            I!(SUB A, b),
            I!(SUB A, c),
            I!(SUB A, d),
            I!(SUB A, e),
            I!(SUB A, h),
            I!(SUB A, l),
            I!(SUB A, (HL)),
            I!(SUB A, a),
            I!(SBC A, b),
            I!(SBC A, c),
            I!(SBC A, d),
            I!(SBC A, e),
            I!(SBC A, h),
            I!(SBC A, l),
            I!(SBC A, (HL)),
            I!(SBC A, a),
            /* ROW */
            I!(AND A, b),
            I!(AND A, c),
            I!(AND A, d),
            I!(AND A, e),
            I!(AND A, h),
            I!(AND A, l),
            I!(AND A, (HL)),
            I!(AND A, a),
            I!(XOR A, b),
            I!(XOR A, c),
            I!(XOR A, d),
            I!(XOR A, e),
            I!(XOR A, h),
            I!(XOR A, l),
            I!(XOR A, (HL)),
            I!(XOR A, a),
            /* ROW */
            I!(OR A, b),
            I!(OR A, c),
            I!(OR A, d),
            I!(OR A, e),
            I!(OR A, h),
            I!(OR A, l),
            I!(OR A, (HL)),
            I!(OR A, a),
            I!(CP A, b),
            I!(CP A, c),
            I!(CP A, d),
            I!(CP A, e),
            I!(CP A, h),
            I!(CP A, l),
            I!(CP A, (HL)),
            I!(CP A, a),
            /* ROW */
            I!(RET get_not_zero),
            I!(POP [b c]),
            I!(JP get_not_zero, u16),
            I!(JP u16),
            I!(CALL get_not_zero, u16),
            I!(PUSH [b c]),
            I!(ADD a, u8),
            I!(RST 0x00),
            I!(RET get_zero),
            I!(RET),
            I!(JP get_zero, u16),
            I!(PREFIX CB),
            I!(CALL get_zero, u16),
            I!(CALL u16),
            I!(ADC A, u8),
            I!(RST 0x08),
            /* ROW */
            I!(RET get_not_carry),
            I!(POP [d e]),
            I!(JP get_not_carry, u16),
            I!(NOP),
            I!(CALL get_not_carry, u16),
            I!(PUSH [d e]),
            I!(SUB A, u8),
            I!(RST 0x10),
            I!(RET get_carry),
            I!(RETI),
            I!(JP get_carry, u16),
            I!(NOP),
            I!(CALL get_carry, u16),
            I!(NOP),
            I!(SBC A, u8),
            I!(RST 0x18),
            /* ROW */
            I!(LD(FF00 + u8), A),
            I!(POP [h l]),
            I!(LD(FF00 + C), A),
            I!(NOP),
            I!(NOP),
            I!(PUSH [h l]),
            I!(AND A, u8),
            I!(RST 0x20),
            I!(ADD SP, i8),
            I!(JP HL),
            I!(LD(u16), A),
            I!(NOP),
            I!(NOP),
            I!(NOP),
            I!(XOR A, u8),
            I!(RST 0x28),
            /* ROW */
            I!(LD A,(FF00+u8)),
            I!(POP [a f]),
            I!(LD A, (FF00+C)),
            I!(DI),
            I!(NOP),
            I!(PUSH [a f]),
            I!(OR A, u8),
            I!(RST 0x30),
            I!(LD HL, SP+i8),
            I!(LD SP, HL),
            I!(LD A, (u16)),
            I!(EI),
            I!(NOP),
            I!(NOP),
            I!(CP A, u8),
            I!(RST 0x38),
        ]
    }

    pub fn call(&mut self, opcode: u8) {
        {
            let opcode = opcode as usize;
            if !self.prefix_mode {
                self.table[opcode](self);
            } else {
                self.extended_table[opcode](self);
                self.prefix_mode = false;
            }

            self.ticks += 4;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{cartridge::MBC3, gpu::VRAM};

    use super::*;

    #[test]
    fn test_ld_bc_word() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);

        let mut cpu = CPU {
            registers: Registers {
                pc: 0xA000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x01);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(wide!(cpu.registers, b, c), 0xBEEF);
    }

    #[test]
    fn test_ld_b_u8() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x69);

        let mut cpu = CPU {
            registers: Registers {
                pc: 0xA000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x06);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x69);
    }

    #[test]
    fn test_daa() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x11,
                b: 0x19,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x80);
        assert_eq!(cpu.registers.a, 0x2A);
        assert_eq!(cpu.ticks, 4);
        cpu.call(0x27);
        assert_eq!(cpu.registers.a, 0x30);

        assert_eq!(cpu.ticks, 4 + 4);
    }

    #[test]
    fn test_daa_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x01,
                b: 0x99,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x80);
        assert_eq!(cpu.registers.a, 0x9A);
        assert_eq!(cpu.ticks, 4);
        cpu.call(0x27);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_carry());

        assert_eq!(cpu.ticks, 4 + 4);
    }

    #[test]
    fn test_ld_b_c() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0x42,
                c: 0x69,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x41);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x69);
    }

    #[test]
    fn test_ld_mem_bc_a() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0x0000, 0x34);
        mmu.write(0x0001, 0xA2);

        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                a: 0x69,
                b: 0xA2,
                c: 0x34,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x02);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.mmu.read(0xA234), 0x69);
    }

    #[test]
    fn test_word_reg_inc() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                b: 0x68,
                c: 0xFF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x03);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(wide!(cpu.registers, b, c), 0x6900);
    }

    #[test]
    fn test_inc_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                b: 0x68,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x04);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x69);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_zero());
    }

    #[test]
    fn test_inc_b_overflowing() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                b: 0xFF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x04);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x00);
        assert!(!cpu.registers.get_negative());
        assert!(cpu.registers.get_zero());
    }

    #[test]
    fn test_inc_b_half_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                b: 0x0F,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x04);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x10);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_zero());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_inc_mem_hl() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA234, 0x69 - 1);
        let mut cpu = CPU {
            registers: Registers {
                h: 0xA2,
                l: 0x34,
                f: 0xF0,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x34);

        assert_eq!(cpu.ticks, 12);
        let byte = cpu.mmu.read(0xA234);
        assert_eq!(byte, 0x69);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_zero());
    }

    #[test]
    fn test_dec_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x0000,
                b: 0x6A,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x05);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.b, 0x69);
    }

    #[test]
    fn test_rlca() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b11000000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x07);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10000001,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10000001
        );
    }

    #[test]
    fn test_rla() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b11000000,
                f: 0,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x17);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10000000,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10000000
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_rla_with_incoming_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0b11000000,
            ..Registers::new()
        };

        registers.carry(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x17);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10000001,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10000001
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_rla_without_outgoing_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0b01000000,
            ..Registers::new()
        };

        registers.carry(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x17);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10000001,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10000001
        );
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_rra() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10000001,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x1F);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b01000000,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b11000000
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_rrca() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0b00010001,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x0F);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10001000,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10001000
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_ld_u16_sp() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x69);
        mmu.write(0xA001, 0xA0);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0xA000,
                s: 0xBE,
                p: 0xEF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x08);

        assert_eq!(cpu.ticks, 20);
        assert_eq!(cpu.mmu.read(0xA069), 0xEF);
        assert_eq!(cpu.mmu.read(0xA06A), 0xBE);
    }

    #[test]
    fn test_add_hl_bc() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x00,
                h: 0x11,
                l: 0x11,
                b: 0xBE,
                c: 0xEF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x09);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.h, 0xD0,
            "{:#02x} != {:#02x}",
            cpu.registers.h, 0xD0
        );
        assert_eq!(cpu.registers.l, 0x00);
    }

    #[test]
    fn test_ld_a_bc_() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA234, 0x69);

        let mut cpu = CPU {
            registers: Registers {
                pc: 0x00,
                a: 0,
                b: 0xA2,
                c: 0x34,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x0A);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.a, 0x69,
            "{:#02x} != {:#02x}",
            cpu.registers.a, 0x69
        );
    }

    #[test]
    fn test_dec_bc() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0x00,
                a: 0,
                b: 0xA2,
                c: 0x00,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x0B);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0xA1,
            "{:#02x} != {:#02x}",
            cpu.registers.b, 0xA1
        );
        assert_eq!(
            cpu.registers.c, 0xFF,
            "{:#02x} != {:#02x}",
            cpu.registers.c, 0xFF
        );
    }

    #[test]
    fn test_jr_i8() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, 0x05);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0xA010,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x18);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(
            cpu.registers.pc, 0xA016,
            "{:#04x} != {:#04x}",
            cpu.registers.pc, 0x0016
        );
    }

    #[test]
    fn test_jr_i8_negative() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, i8::from(-5) as u8);
        let mut cpu = CPU {
            registers: Registers {
                pc: 0xA010,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x18);

        assert_eq!(
            cpu.registers.pc,
            0xA010 - 5 + 1,
            "{:#04x} != {:#04x}",
            cpu.registers.pc,
            0xA010 - 5 + 1
        );
    }

    #[test]
    fn test_jr_nz_i8_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, 0x05);
        let mut registers = Registers {
            pc: 0xA010,
            ..Registers::new()
        };
        registers.zero(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x20);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.pc, 0xA011,
            "{:#04x} != {:#04x}",
            cpu.registers.pc, 0xA011
        );
    }

    #[test]
    fn test_jr_nz_i8_not_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, 0x05);
        let mut registers = Registers {
            pc: 0xA010,
            ..Registers::new()
        };
        registers.zero(false);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x20);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(
            cpu.registers.pc, 0xA016,
            "{:#04x} != {:#04x}",
            cpu.registers.pc, 0xA016
        );
    }

    #[test]
    fn test_jr_nz_i8_not_zero_subtraction() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, -5i8 as u8);
        let mut registers = Registers {
            pc: 0xA010,
            ..Registers::new()
        };
        registers.zero(false);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x20);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(
            cpu.registers.pc,
            0xA010 - 5 + 1,
            "{:#04x} != {:#04x}",
            cpu.registers.pc,
            0xA010 - 5 + 1
        );
    }

    #[test]
    fn test_jr_z_i8_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, 0x05);
        let mut registers = Registers {
            pc: 0xA010,
            ..Registers::new()
        };
        registers.zero(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x28);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(
            cpu.registers.pc, 0xA016,
            "{:#04x} != {:#04x}",
            cpu.registers.pc, 0xA016
        );
    }

    #[test]
    fn test_jr_z_i8_zero_subtraction() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA010, -5i8 as u8);
        let mut registers = Registers {
            pc: 0xA010,
            ..Registers::new()
        };
        registers.zero(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x28);

        assert_eq!(cpu.ticks, 12);
        assert_eq!(
            cpu.registers.pc,
            0xA010 - 5 + 1,
            "{:#04x} != {:#04x}",
            cpu.registers.pc,
            0xA010 - 5 + 1
        );
    }

    #[test]
    fn test_jr_z_i8_not_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0x0010, 0x05);
        let mut registers = Registers {
            pc: 0x0010,
            ..Registers::new()
        };
        registers.zero(false);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x28);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.pc, 0x0011,
            "{:#04x} != {:#04x}",
            cpu.registers.pc, 0x0011
        );
    }

    #[test]
    fn test_cpl() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10101010,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x2F);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b01010101);
        assert!(cpu.registers.get_negative());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_ccf() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers::new();
        registers.carry(true);
        registers.half_carry(true);
        registers.negative(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x3F);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.get_carry(), false);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
    }

    #[test]
    fn test_scf() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers::new();
        registers.carry(false);
        registers.half_carry(true);
        registers.negative(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x37);

        assert_eq!(cpu.ticks, 4);
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
    }

    #[test]
    fn test_add_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x13,
                b: 0x56,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x80);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x69);
        assert!(!cpu.registers.get_zero())
    }

    #[test]
    fn test_add_a_b_half_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x08,
                b: 0x08,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x80);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x10);
        assert!(!cpu.registers.get_zero());
        assert!(cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_add_a_b_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x80,
                b: 0x80,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x80);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_half_carry());
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_add_a_hl() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA234, 0x13);

        let mut cpu = CPU {
            registers: Registers {
                a: 0x56,
                h: 0xA2,
                l: 0x34,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x86);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.a, 0x69);
    }

    #[test]
    fn test_add_a_l_flags() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10001001,
                l: 0b10001001,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x85);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b00010010);
        assert!(cpu.registers.get_carry());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_add_a_l_flags_off() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00000001,
                l: 0b00000001,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x85);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b00000010);
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
    }

    #[test]
    fn test_adc_a_b_with_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0b00001001,
            b: 0b10001001,
            ..Registers::new()
        };

        registers.carry(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x88);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(
            cpu.registers.a, 0b10010011,
            "{:#010b} != {:#010b}",
            cpu.registers.a, 0b10010011
        );
        assert!(cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_sub_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0x69,
                b: 0x33,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x90);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x36);
    }

    #[test]
    fn test_cp_a_b_half_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00010000,
                b: 1,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xB8);

        assert_eq!(cpu.ticks, 4);
        assert!(!cpu.registers.get_half_carry());
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_sub_a_b_half_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00010000,
                b: 1,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x90);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 15);
        assert!(cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_sub_a_b_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00000000,
                b: 0b00010000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x90);

        assert_eq!(cpu.ticks, 4);
        assert!(!cpu.registers.get_half_carry());
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_sub_a_b_both_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00000000,
                b: 0b00001000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x90);

        assert_eq!(cpu.ticks, 4);
        assert!(cpu.registers.get_half_carry());
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_cp_a_b_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00000000,
                b: 0b00010000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xB8);

        assert_eq!(cpu.ticks, 4);
        assert!(cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_sbc_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0x69,
            b: 0x33,
            ..Registers::new()
        };
        registers.carry(true);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x98);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x35);
    }

    #[test]
    fn test_sbc_a_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            a: 0x69,
            b: 0x33,
            ..Registers::new()
        };
        registers.carry(false);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0x98);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0x36);
    }

    #[test]
    fn test_and_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10101010,
                b: 0b00001111,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xA0);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b00001010);
        assert!(cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_or_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10101010,
                b: 0b00001111,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xB0);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b10101111);
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_xor_a_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10101010,
                b: 0b01011111,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xA8);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0b11110101);
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_zero());
    }

    #[test]
    fn test_xor_a_a() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                a: 0b10101010,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xAF);

        assert_eq!(cpu.ticks, 4);
        assert_eq!(cpu.registers.a, 0);
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
        assert!(cpu.registers.get_zero());
    }

    #[test]
    fn test_ret() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let registers = Registers {
            s: 0xA0,
            p: 0x00,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC9);

        assert_eq!(cpu.ticks, 16);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA002);
    }

    #[test]
    fn test_ret_nz() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let mut registers = Registers {
            s: 0xA0,
            p: 0x00,
            ..Registers::new()
        };
        registers.zero(false);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC0);

        assert_eq!(cpu.ticks, 20);
        assert_eq!(cpu.registers.pc, 0xBEEF);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA002);
    }

    #[test]
    fn test_ret_nz_while_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let mut registers = Registers {
            s: 0xA0,
            p: 0x00,
            pc: 0x4200,
            ..Registers::new()
        };
        registers.zero(true);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC0);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.pc, 0x4200);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA000);
    }

    #[test]
    fn test_pop_bc() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let registers = Registers {
            s: 0xA0,
            p: 0x00,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC1);

        assert_eq!(cpu.ticks, 12);
        let bc = wide!(cpu.registers, b, c);
        assert_eq!(bc, 0xBEEF, "{:#06x} != {:#06x}", bc, 0xBEEF);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA002, "{:#06x} != {:#06x}", sp, 0xA002);
    }

    #[test]
    fn test_push_bc() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let registers = Registers {
            s: 0xA0,
            p: 0x02,
            b: 0xBE,
            c: 0xEF,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC5);

        assert_eq!(cpu.ticks, 16);
        let hi = cpu.mmu.read(0xA001) as u16;
        let lo = cpu.mmu.read(0xA000) as u16;
        let v = (hi << 8) | lo;
        assert_eq!(v, 0xBEEF, "{:#06x} != {:#06x}", v, 0xBEEF);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA000, "{:#06x} != {:#06x}", sp, 0xA000);
    }

    #[test]
    fn test_jp_u16() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let registers = Registers {
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC3);

        assert_eq!(cpu.ticks, 16);
        let pc = cpu.registers.pc;
        assert_eq!(pc, 0xBEEF, "{:#06x} != {:#06x}", pc, 0xBEEF);
    }

    #[test]
    fn test_jp_nz_u16() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let mut registers = Registers {
            pc: 0xA000,
            ..Registers::new()
        };

        registers.zero(false);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC2);

        assert_eq!(cpu.ticks, 16);
        let pc = cpu.registers.pc;
        assert_eq!(pc, 0xBEEF, "{:#06x} != {:#06x}", pc, 0xBEEF);
    }

    #[test]
    fn test_jp_nz_u16_while_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let mut registers = Registers {
            pc: 0xA000,
            ..Registers::new()
        };

        registers.zero(true);
        let registers = registers;

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xC2);

        assert_eq!(cpu.ticks, 12);
        let pc = cpu.registers.pc;
        assert_eq!(pc, 0xA002, "{:#06x} != {:#06x}", pc, 0xA002);
    }

    #[test]
    fn test_rst_18h() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let registers = Registers {
            pc: 0xBEEF,
            s: 0xA0,
            p: 0x02,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xDF);

        assert_eq!(cpu.ticks, 16);

        assert_eq!(cpu.registers.pc, 0x0018);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xA000);

        let lo = cpu.mmu.read(0xA000) as u16;
        let hi = cpu.mmu.read(0xA001) as u16;
        let v = (hi << 8) | lo;
        assert_eq!(v, 0xBEEF);
    }

    #[test]
    fn test_call() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0xEF);
        mmu.write(0xA001, 0xBE);
        let registers = Registers {
            s: 0xB0,
            p: 0x02,
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCD);

        assert_eq!(cpu.ticks, 24);

        assert_eq!(cpu.registers.pc, 0xBEEF);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xB000);

        let lo = cpu.mmu.read(0xB000) as u16;
        let hi = cpu.mmu.read(0xB001) as u16;
        let old_pc = (hi << 8) | lo;
        assert_eq!(old_pc, 0xA002);
    }

    #[test]
    fn test_add_sp_i8() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x11);

        let registers = Registers {
            s: 0xAB,
            p: 0x22,
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xE8);

        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xAB33);
        assert_eq!(cpu.registers.pc, 0xA001);
    }

    #[test]
    fn test_add_sp_i8_sub() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, (-0x11 as i8) as u8);

        let registers = Registers {
            s: 0xAB,
            p: 0x22,
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xE8);

        assert_eq!(cpu.ticks, 16);
        let sp = wide!(cpu.registers, s, p);
        assert_eq!(sp, 0xAB11);
        assert_eq!(cpu.registers.pc, 0xA001);
    }

    #[test]
    fn test_ld_hl_sp_i8() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x11);

        let registers = Registers {
            s: 0xAB,
            p: 0x22,
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xF8);

        assert_eq!(cpu.ticks, 12);
        let hl = wide!(cpu.registers, h, l);
        assert_eq!(hl, 0xAB33, "{:#06x} != {:#06x}", hl, 0xAB33);
        assert_eq!(
            cpu.registers.pc, 0xA001,
            "{:#06x} != {:#06x}",
            cpu.registers.pc, 0xA001
        );
    }

    #[test]
    fn test_ld_a_mem_u16() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x22);
        mmu.write(0xA001, 0xA1);
        mmu.write(0xA122, 0x69);

        let registers = Registers {
            pc: 0xA000,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xFA);

        assert_eq!(cpu.ticks, 16);
        let a = cpu.registers.a;
        assert_eq!(a, 0x69, "{:#04x} != {:#04x}", a, 0x69);
        assert_eq!(
            cpu.registers.pc, 0xA002,
            "{:#06x} != {:#06x}",
            cpu.registers.pc, 0xA002
        );
    }

    #[test]
    fn test_ld_mem_u16_a() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA000, 0x22);
        mmu.write(0xA001, 0xA1);
        mmu.write(0xA122, 0x33);

        let registers = Registers {
            pc: 0xA000,
            a: 0x69,
            ..Registers::new()
        };

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xEA);

        assert_eq!(cpu.ticks, 16);
        let dest = cpu.mmu.read(0xA122);
        assert_eq!(dest, 0x69, "{:#04x} != {:#04x}", dest, 0x69);
        assert_eq!(
            cpu.registers.pc, 0xA002,
            "{:#06x} != {:#06x}",
            cpu.registers.pc, 0xA002
        );
    }

    #[test]
    fn test_rlc_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b11000000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x00);

        assert_eq!(
            cpu.registers.b, 0b10000001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b10000001
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_zero());
    }

    #[test]
    fn test_rlc_b_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b00000000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x00);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0
        );
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_rlc_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b00000100,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x00);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b00001000,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b00001000
        );
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_zero());
    }

    #[test]
    fn test_rrc_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b10000001,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x08);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b11000000,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b11000000
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_rl_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b10000010,
            ..Registers::new()
        };

        registers.carry(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x10);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b00000101,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b00000101
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_rl_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b10000010,
            ..Registers::new()
        };

        registers.carry(false);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x10);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b00000100,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b00000100
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_rr_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b10000011,
            ..Registers::new()
        };

        registers.carry(true);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x18);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b11000001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b11000001
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_rr_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);

        let mut registers = Registers {
            b: 0b10000010,
            ..Registers::new()
        };

        registers.carry(false);

        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x18);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b01000001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b01000001
        );
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_sla_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b11000001,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x20);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b10000010,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b10000010
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_sla_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b01000001,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x20);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b10000010,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b10000010
        );
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_sra_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b11000011,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x28);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b11100001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b11100001
        );
        assert!(cpu.registers.get_carry());
    }

    #[test]
    fn test_srl_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b11000011,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x38);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b01100001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b01100001
        );
        assert!(cpu.registers.get_carry());
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_srl_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b11000010,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x38);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b01100001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b01100001
        );
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_srl_b_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b00000001,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x38);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0
        );
        assert!(cpu.registers.get_carry());
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
    }

    #[test]
    fn test_sra_b_no_carry() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut registers = Registers {
            b: 0b01000010,
            ..Registers::new()
        };

        registers.carry(true);
        let mut cpu = CPU {
            registers,
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x28);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(
            cpu.registers.b, 0b00100001,
            "{:#010b} != {:#010b}",
            cpu.registers.b, 0b00100001
        );
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_swap_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b11101000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x30);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0b10001110);
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_swap_b_zero() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x30);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0);
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
        assert!(!cpu.registers.get_carry());
    }

    #[test]
    fn test_bit_0_b_hi() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b10000001,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x40);

        assert_eq!(cpu.ticks, 8);
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_bit_0_b_lo() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b10000000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x40);

        assert_eq!(cpu.ticks, 8);
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_bit_4_b_hi() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b00010000,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x60);

        assert_eq!(cpu.ticks, 8);
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_bit_4_b_lo() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0b11101111,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x60);

        assert_eq!(cpu.ticks, 8);
        assert!(cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(cpu.registers.get_half_carry());
    }

    #[test]
    fn test_res_0_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0xFF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0x80);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0xFE);
    }

    #[test]
    fn test_res_4_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0xFF,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0xA0);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0xEF);
    }

    #[test]
    fn test_res_4_hl() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        mmu.write(0xA234, 0xFF);
        let mut cpu = CPU {
            registers: Registers {
                h: 0xA2,
                l: 0x34,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0xA6);

        assert_eq!(cpu.ticks, 16);
        let byte = cpu.mmu.read(0xA234);
        assert_eq!(byte, 0xEF);
    }

    #[test]
    fn test_set_0_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0x00,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0xC0);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0x01);
    }

    #[test]
    fn test_set_4_b() {
        let mut gpu = VRAM::new();
        let mut mmu = MMU::<MBC3>::new_with_mbc3(&mut gpu);
        let mut cpu = CPU {
            registers: Registers {
                b: 0x00,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0xE0);

        assert_eq!(cpu.ticks, 8);
        assert_eq!(cpu.registers.b, 0x10);
    }

    #[test]
    fn test_set_4_hl() {
        let mut gpu = VRAM::new();
        let rom_banks = Box::new([(); 0x80].map(|_| Box::new([0u8; 0x4000])));
        let mbc3 = MBC3::new(rom_banks);
        let mut gpu = VRAM::new();
        let mut mmu = MMU::new(&mut gpu, mbc3);
        mmu.write(0xA234, 0x00);
        let mut cpu = CPU {
            registers: Registers {
                h: 0xA2,
                l: 0x34,
                ..Registers::new()
            },
            ..CPU::<MBC3>::new(&mut mmu)
        };

        cpu.call(0xCB);
        cpu.call(0xE6);

        assert_eq!(cpu.ticks, 16);
        let byte = cpu.mmu.read(0xA234);
        assert_eq!(byte, 0x10);
    }
}
