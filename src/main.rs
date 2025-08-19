use crate::cartridge::MBC3;
use crate::cpu::CPU;
use crate::gpu::VRAM;
use crate::mmu::MMU;

use std::env;
use std::fs::File;
use std::io::Read;

mod cartridge;
mod cpu;
mod cpu_comprehensive_tests;
mod gameboy;
mod gpu;
mod mmu;
mod opcodes;
mod registers;
mod run_loop;
mod sprite;
mod utility {
    pub(crate) mod convenience;
    pub mod ui_state;
}
pub mod interrupts;
use std::sync::mpsc;
use utility::ui_state::UIState;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Provide file path");
        return;
    }

    let file_path = &args[1];

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    let mut buffer = Vec::new();
    if let Err(error) = file.read_to_end(&mut buffer) {
        eprintln!("Error reading file: {}", error);
        return;
    }

    let mut buffer_offset = 0;

    let rom_banks = buffer
        .chunks(0x4000)
        .map(|c| {
            let mut sized_array = [0u8; 0x4000];
            sized_array.copy_from_slice(c);
            Box::new(sized_array)
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let mbc3 = MBC3::new(rom_banks);
    let mut gpu = VRAM::new();
    let mut mmu = MMU::new(&mut gpu, mbc3);
    let cpu = CPU::new(&mut mmu);

    let (tx, rx) = mpsc::channel::<UIState>();
}
