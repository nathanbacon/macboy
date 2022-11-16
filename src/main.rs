use crate::registers::Registers;
use crate::mmu::MMU;
use crate::cpu::CPU;

mod registers;
mod cpu;
mod mmu;

fn main() {
    println!("Hello, world!");
    let (r, o) = 0x2u8.overflowing_shr(9);
    println!("{} {}", r, o);
}
