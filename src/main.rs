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
        s: 0xFF,
        p: 0xFE,
        sp: 0xFFFE,
        pc: 0x0100,
    };

    println!("Hello, world!");
}
