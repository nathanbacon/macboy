use crate::registers::Registers;
use crate::mmu::MMU;
use crate::cpu::CPU;

mod registers;
mod cpu;
mod mmu;

fn main() {

    let mut registers = Registers {
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
    };

    registers.set_bc(0xABCD);
    let bc = registers.get_bc();

    println!("Hello, world!");
    println!("{:#04x}", bc);
    println!("{:#02x}", registers.b);
    println!("{:#02x}", registers.c);
}
