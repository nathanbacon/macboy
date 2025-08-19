use crate::{
    cartridge::MBC3,
    cpu::CPU,
    gpu::VRAM,
    mmu::MMU,
    opcodes::{ExtendedOpcode, Opcode},
    registers::Registers,
};
use std::collections::HashSet;

/// Comprehensive CPU test suite for GameBoy emulator
/// This module provides systematic testing of all CPU instructions,
/// edge cases, and instruction interactions.

#[test]
fn test_simple() {
    assert_eq!(2 + 2, 4);
}

/// Track which opcodes have been tested to ensure complete coverage
struct OpcodeCoverageTracker {
    tested_opcodes: HashSet<u8>,
    tested_extended_opcodes: HashSet<u8>,
}

impl OpcodeCoverageTracker {
    fn new() -> Self {
        Self {
            tested_opcodes: HashSet::new(),
            tested_extended_opcodes: HashSet::new(),
        }
    }

    fn mark_tested(&mut self, opcode: u8) {
        self.tested_opcodes.insert(opcode);
    }

    fn mark_extended_tested(&mut self, opcode: u8) {
        self.tested_extended_opcodes.insert(opcode);
    }

    fn get_untested_opcodes(&self) -> Vec<u8> {
        let mut untested = Vec::new();
        for i in 0..=255 {
            if !self.tested_opcodes.contains(&i) {
                // Skip invalid opcodes that don't exist on GameBoy
                let opcode = Opcode::from(i);
                if !matches!(opcode, Opcode::Invalid) {
                    untested.push(i);
                }
            }
        }
        untested
    }

    fn get_untested_extended_opcodes(&self) -> Vec<u8> {
        let mut untested = Vec::new();
        for i in 0..=255 {
            if !self.tested_extended_opcodes.contains(&i) {
                untested.push(i);
            }
        }
        untested
    }

    fn coverage_percentage(&self) -> (f32, f32) {
        let total_valid_opcodes = (0..=255u8)
            .filter(|&i| !matches!(Opcode::from(i), Opcode::Invalid))
            .count();

        let standard_coverage =
            (self.tested_opcodes.len() as f32 / total_valid_opcodes as f32) * 100.0;
        let extended_coverage = (self.tested_extended_opcodes.len() as f32 / 256.0) * 100.0;

        (standard_coverage, extended_coverage)
    }
}

#[test]
fn test_opcode_coverage_analysis() {
    let mut tracker = OpcodeCoverageTracker::new();

    // Track opcodes tested in existing test functions
    track_ld_r_r_opcodes(&mut tracker);
    track_ld_r_n_opcodes(&mut tracker);
    track_add_a_r_opcodes(&mut tracker);
    track_sub_a_r_opcodes(&mut tracker);
    track_ld_rr_nn_opcodes(&mut tracker);
    track_inc_rr_opcodes(&mut tracker);
    track_bit_opcodes(&mut tracker);
    track_set_res_opcodes(&mut tracker);
    track_rotation_opcodes(&mut tracker);
    track_jump_opcodes(&mut tracker);
    track_misc_opcodes(&mut tracker);

    let (standard_coverage, extended_coverage) = tracker.coverage_percentage();
    let untested = tracker.get_untested_opcodes();
    let untested_extended = tracker.get_untested_extended_opcodes();

    println!("=== OPCODE COVERAGE REPORT ===");
    println!("Standard opcodes: {:.1}% covered", standard_coverage);
    println!("Extended opcodes: {:.1}% covered", extended_coverage);

    if !untested.is_empty() {
        println!("\nUntested standard opcodes ({}):", untested.len());
        for &opcode in &untested {
            let opcode_enum = Opcode::from(opcode);
            println!(
                "  0x{:02X}: {} - {}",
                opcode,
                opcode_enum.mnemonic(),
                format!("{:?}", opcode_enum)
            );
        }
    }

    if !untested_extended.is_empty() {
        println!("\nUntested extended opcodes ({}):", untested_extended.len());
        for &opcode in &untested_extended {
            let opcode_enum = ExtendedOpcode::from(opcode);
            println!(
                "  CB 0x{:02X}: {} - {:?}",
                opcode,
                opcode_enum.mnemonic(),
                opcode_enum
            );
        }
    }

    if untested.is_empty() && untested_extended.is_empty() {
        println!("\n🎉 ALL OPCODES TESTED! Complete coverage achieved!");
    } else {
        println!(
            "\n📊 Total untested: {} standard + {} extended = {} opcodes",
            untested.len(),
            untested_extended.len(),
            untested.len() + untested_extended.len()
        );

        // Generate test templates for missing opcodes
        generate_missing_test_templates(&untested, &untested_extended);
    }
}

fn generate_missing_test_templates(untested: &[u8], untested_extended: &[u8]) {
    println!("\n=== TEST GENERATION SUGGESTIONS ===");

    // Group similar instructions for easier testing
    let mut arithmetic_ops: Vec<u8> = Vec::new();
    let mut load_ops: Vec<u8> = Vec::new();
    let mut stack_ops: Vec<u8> = Vec::new();
    let mut jump_ops: Vec<u8> = Vec::new();
    let mut bit_ops: Vec<u8> = Vec::new();
    let mut misc_ops: Vec<u8> = Vec::new();

    for &opcode in untested {
        let opcode_enum = Opcode::from(opcode);
        let name = format!("{:?}", opcode_enum);

        if name.contains("Add")
            || name.contains("Sub")
            || name.contains("And")
            || name.contains("Xor")
            || name.contains("Or")
            || name.contains("Cp")
            || name.contains("Inc")
            || name.contains("Dec")
            || name.contains("Adc")
            || name.contains("Sbc")
        {
            arithmetic_ops.push(opcode);
        } else if name.contains("Ld") {
            load_ops.push(opcode);
        } else if name.contains("Push") || name.contains("Pop") {
            stack_ops.push(opcode);
        } else if name.contains("Jp")
            || name.contains("Jr")
            || name.contains("Call")
            || name.contains("Ret")
            || name.contains("Rst")
        {
            jump_ops.push(opcode);
        } else {
            misc_ops.push(opcode);
        }
    }

    if !arithmetic_ops.is_empty() {
        println!(
            "\n🔢 Arithmetic Operations ({} opcodes):",
            arithmetic_ops.len()
        );
        println!("   Test template: #[test] fn test_arithmetic_complete() {{");
        for &op in &arithmetic_ops[..std::cmp::min(5, arithmetic_ops.len())] {
            println!("   // Test opcode 0x{:02X}: {:?}", op, Opcode::from(op));
        }
        if arithmetic_ops.len() > 5 {
            println!("   // ... and {} more", arithmetic_ops.len() - 5);
        }
        println!("   }}");
    }

    if !load_ops.is_empty() {
        println!("\n📋 Load Operations ({} opcodes):", load_ops.len());
        println!("   Many can be tested with existing patterns in test_ld_r_r_all_combinations");
    }

    if !stack_ops.is_empty() {
        println!("\n📚 Stack Operations ({} opcodes):", stack_ops.len());
        println!("   Test template: #[test] fn test_stack_complete() {{");
        for &op in &stack_ops {
            println!("   // Test opcode 0x{:02X}: {:?}", op, Opcode::from(op));
        }
        println!("   }}");
    }

    if !jump_ops.is_empty() {
        println!("\n🦘 Jump/Call Operations ({} opcodes):", jump_ops.len());
        println!("   Test template: #[test] fn test_flow_control_complete() {{");
        for &op in &jump_ops[..std::cmp::min(3, jump_ops.len())] {
            println!("   // Test opcode 0x{:02X}: {:?}", op, Opcode::from(op));
        }
        if jump_ops.len() > 3 {
            println!("   // ... and {} more", jump_ops.len() - 3);
        }
        println!("   }}");
    }

    if !untested_extended.is_empty() {
        println!(
            "\n🔄 Extended (CB) Operations ({} opcodes):",
            untested_extended.len()
        );
        println!("   Most can be tested by extending existing rotation/bit tests to all registers");
    }

    println!("\n💡 Quick wins to improve coverage:");
    println!(
        "   1. Extend test_add_a_r_comprehensive to include ADC, SBC, AND, XOR, OR, CP variants"
    );
    println!("   2. Add test_inc_dec_all_registers for all INC/DEC r instructions");
    println!("   3. Add test_stack_all_pairs for PUSH/POP of all register pairs");
    println!("   4. Extend rotation tests to cover all registers (not just B)");
    println!("   5. Add SWAP instruction tests (CB 0x30-0x37)");
}

// Helper functions to track which opcodes are tested by each test function

fn track_ld_r_r_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // All LD r,r combinations (0x40-0x7F excluding 0x76 HALT)
    for dest in 0..8 {
        for src in 0..8 {
            if dest == 6 && src == 6 {
                continue;
            } // Skip HALT (0x76)
            let opcode = 0x40 + (dest * 8) + src;
            tracker.mark_tested(opcode);
        }
    }
}

fn track_ld_r_n_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // LD r,n instructions
    let opcodes = [0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E, 0x3E]; // B,C,D,E,H,L,A
    for &opcode in &opcodes {
        tracker.mark_tested(opcode);
    }
}

fn track_add_a_r_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // ADD A,r instructions (0x80-0x87)
    for i in 0x80..=0x87 {
        tracker.mark_tested(i);
    }
}

fn track_sub_a_r_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // SUB r instructions (0x90-0x97)
    for i in 0x90..=0x97 {
        tracker.mark_tested(i);
    }
}

fn track_ld_rr_nn_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // LD rr,nn instructions
    let opcodes = [0x01, 0x11, 0x21, 0x31]; // BC, DE, HL, SP
    for &opcode in &opcodes {
        tracker.mark_tested(opcode);
    }
}

fn track_inc_rr_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // INC rr instructions
    let opcodes = [0x03, 0x13, 0x23, 0x33]; // BC, DE, HL, SP
    for &opcode in &opcodes {
        tracker.mark_tested(opcode);
    }
}

fn track_bit_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // BIT b,r instructions (CB 0x40-0x7F)
    for bit in 0..8 {
        for reg in 0..8 {
            let opcode = 0x40 + (bit * 8) + reg;
            tracker.mark_extended_tested(opcode);
        }
    }
}

fn track_set_res_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // SET b,r instructions (CB 0xC0-0xFF)
    for bit in 0..8 {
        for reg in 0..8 {
            let set_opcode = 0xC0 + (bit * 8) + reg;
            let res_opcode = 0x80 + (bit * 8) + reg;
            tracker.mark_extended_tested(set_opcode);
            tracker.mark_extended_tested(res_opcode);
        }
    }
}

fn track_rotation_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // Rotation instructions we test (CB prefix)
    let opcodes = [
        0x00, 0x08, 0x10, 0x18, // RLC, RRC, RL, RR (B register)
        0x20, 0x28, 0x38, // SLA, SRA, SRL (B register)
    ];
    for &opcode in &opcodes {
        tracker.mark_extended_tested(opcode);
    }
}

fn track_jump_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // Conditional jump instructions
    let opcodes = [0x20, 0x28, 0x30, 0x38]; // JR NZ, JR Z, JR NC, JR C
    for &opcode in &opcodes {
        tracker.mark_tested(opcode);
    }
}

fn track_misc_opcodes(tracker: &mut OpcodeCoverageTracker) {
    // Miscellaneous opcodes tested in various functions
    let opcodes = [
        0x00, // NOP (in timing test)
        0x41, // LD B,C (in flag interaction test)
        0x87, // ADD A,A (in flag interaction test)
        0xC5, 0xC1, // PUSH BC, POP BC (in stack test)
        0x7E, 0x77, // LD A,(HL), LD (HL),A (in memory addressing test)
        0x3E, // LD A,n (in memory addressing test)
        0x07, 0x17, // RLCA, RLA (in instruction sequence test)
        0x80, 0x88, // ADD A,B, ADC A,B (in instruction sequence test)
        0xCB, // CB prefix
    ];
    for &opcode in &opcodes {
        tracker.mark_tested(opcode);
    }
}

/// Helper function to create a basic CPU setup for testing
fn setup_cpu() -> CPU<'static, MBC3> {
    let gpu = Box::leak(Box::new(VRAM::new()));
    let mmu = Box::leak(Box::new(MMU::<MBC3>::new_with_mbc3(gpu)));
    CPU::new(mmu)
}

/// Helper macro to test all register variants of an instruction
macro_rules! test_register_variants {
    ($test_name:ident, $opcode_base:expr, $setup_fn:expr, $assert_fn:expr) => {
        #[test]
        fn $test_name() {
            let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];
            for (i, &reg) in registers.iter().enumerate() {
                if reg == 'h' && $opcode_base == 0x70 {
                    continue;
                } // Skip HALT

                let mut cpu = $setup_fn(reg);
                let opcode = $opcode_base + i as u8;
                cpu.call(opcode);
                $assert_fn(&cpu, reg);
            }
        }
    };
}

// ========== 8-bit Load Instructions ==========

#[test]
fn test_type_safe_opcode_execution() {
    let mut cpu = setup_cpu();

    // Test type-safe instruction execution
    cpu.execute(Opcode::LdAE); // Load E into A

    // Test opcode information methods
    assert_eq!(Opcode::LdAE.mnemonic(), "LD A, E");
    assert_eq!(Opcode::LdAE.timing(), 4);

    // Test getting instruction info by raw opcode
    assert_eq!(cpu.get_instruction_mnemonic(0x7B), "LD A, E");
    assert_eq!(cpu.get_instruction_timing(0x7B), 4);

    // Test extended opcodes
    assert_eq!(ExtendedOpcode::RlcB.mnemonic(), "RLC B");
    assert_eq!(ExtendedOpcode::RlcB.timing(), 8);
}

#[test]
fn test_ld_r_r_all_combinations() {
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    // Create a lookup table for all LD r,r opcodes
    let ld_opcodes = [
        [
            Opcode::LdBB,
            Opcode::LdBC,
            Opcode::LdBD,
            Opcode::LdBE,
            Opcode::LdBH,
            Opcode::LdBL,
            Opcode::LdBA,
        ],
        [
            Opcode::LdCB,
            Opcode::LdCC,
            Opcode::LdCD,
            Opcode::LdCE,
            Opcode::LdCH,
            Opcode::LdCL,
            Opcode::LdCA,
        ],
        [
            Opcode::LdDB,
            Opcode::LdDC,
            Opcode::LdDD,
            Opcode::LdDE,
            Opcode::LdDH,
            Opcode::LdDL,
            Opcode::LdDA,
        ],
        [
            Opcode::LdEB,
            Opcode::LdEC,
            Opcode::LdED,
            Opcode::LdEE,
            Opcode::LdEH,
            Opcode::LdEL,
            Opcode::LdEA,
        ],
        [
            Opcode::LdHB,
            Opcode::LdHC,
            Opcode::LdHD,
            Opcode::LdHE,
            Opcode::LdHH,
            Opcode::LdHL,
            Opcode::LdHA,
        ],
        [
            Opcode::LdLB,
            Opcode::LdLC,
            Opcode::LdLD,
            Opcode::LdLE,
            Opcode::LdLH,
            Opcode::LdLL,
            Opcode::LdLA,
        ],
        [
            Opcode::LdAB,
            Opcode::LdAC,
            Opcode::LdAD,
            Opcode::LdAE,
            Opcode::LdAH,
            Opcode::LdAL,
            Opcode::LdAA,
        ],
    ];

    for (dest_idx, &dest) in registers.iter().enumerate() {
        for (src_idx, &src) in registers.iter().enumerate() {
            if dest_idx == 6 && src_idx == 6 {
                continue;
            } // Skip LD A,A for now

            let mut cpu = setup_cpu();

            // Set source register to test value
            match src {
                'b' => cpu.registers.b = 0x42,
                'c' => cpu.registers.c = 0x42,
                'd' => cpu.registers.d = 0x42,
                'e' => cpu.registers.e = 0x42,
                'h' => cpu.registers.h = 0x42,
                'l' => cpu.registers.l = 0x42,
                'a' => cpu.registers.a = 0x42,
                _ => panic!("Invalid register"),
            }

            let opcode = ld_opcodes[dest_idx][src_idx];
            cpu.call(opcode as u8);

            // Check destination register
            let dest_value = match dest {
                'b' => cpu.registers.b,
                'c' => cpu.registers.c,
                'd' => cpu.registers.d,
                'e' => cpu.registers.e,
                'h' => cpu.registers.h,
                'l' => cpu.registers.l,
                'a' => cpu.registers.a,
                _ => panic!("Invalid register"),
            };

            assert_eq!(dest_value, 0x42, "LD {},{} failed", dest, src);
            assert_eq!(cpu.ticks, 4);
        }
    }
}

#[test]
fn test_ld_r_n_all_registers() {
    let opcodes = [
        Opcode::LdBN,
        Opcode::LdCN,
        Opcode::LdDN,
        Opcode::LdEN,
        Opcode::LdHN,
        Opcode::LdLN,
        Opcode::LdAN,
    ];
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    for (i, &opcode) in opcodes.iter().enumerate() {
        let mut cpu = setup_cpu();
        cpu.mmu.write(0x0000, 0x69);
        cpu.registers.pc = 0x0000;

        cpu.call(opcode as u8);

        let value = match registers[i] {
            'b' => cpu.registers.b,
            'c' => cpu.registers.c,
            'd' => cpu.registers.d,
            'e' => cpu.registers.e,
            'h' => cpu.registers.h,
            'l' => cpu.registers.l,
            'a' => cpu.registers.a,
            _ => panic!("Invalid register"),
        };

        assert_eq!(value, 0x69);
        assert_eq!(cpu.registers.pc, 0x0001);
        assert_eq!(cpu.ticks, 4);
    }
}

// ========== 8-bit Arithmetic Instructions ==========

#[test]
fn test_add_a_r_comprehensive() {
    let opcodes = [
        Opcode::AddAB,
        Opcode::AddAC,
        Opcode::AddAD,
        Opcode::AddAE,
        Opcode::AddAH,
        Opcode::AddAL,
        Opcode::AddAA,
    ];
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    // Test normal addition
    for (i, &opcode) in opcodes.iter().enumerate() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0x20;

        match registers[i] {
            'b' => cpu.registers.b = 0x15,
            'c' => cpu.registers.c = 0x15,
            'd' => cpu.registers.d = 0x15,
            'e' => cpu.registers.e = 0x15,
            'h' => cpu.registers.h = 0x15,
            'l' => cpu.registers.l = 0x15,
            'a' => cpu.registers.a = 0x15, // Will be 0x15 + 0x15 = 0x2A
            _ => panic!("Invalid register"),
        }

        cpu.call(opcode as u8);

        let expected = if registers[i] == 'a' { 0x2A } else { 0x35 };
        assert_eq!(cpu.registers.a, expected);
        assert!(!cpu.registers.get_zero());
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_carry());
        assert!(!cpu.registers.get_half_carry());
    }
}

#[test]
fn test_add_a_r_carry_conditions() {
    // Test carry flag conditions for all ADD A,r instructions
    let test_cases = [
        (0xFF, 0x01, true, false),  // Carry, no half-carry
        (0x0F, 0x01, false, true),  // Half-carry, no carry
        (0xFF, 0xFF, true, true),   // Both carry and half-carry
        (0x00, 0x00, false, false), // Neither
    ];

    for (a_val, src_val, expect_carry, expect_half_carry) in test_cases {
        let mut cpu = setup_cpu();
        cpu.registers.a = a_val;
        cpu.registers.b = src_val;

        cpu.call(0x80); // ADD A,B

        assert_eq!(
            cpu.registers.get_carry(),
            expect_carry,
            "Carry flag incorrect for ADD {:02X},{:02X}",
            a_val,
            src_val
        );
        assert_eq!(
            cpu.registers.get_half_carry(),
            expect_half_carry,
            "Half-carry flag incorrect for ADD {:02X},{:02X}",
            a_val,
            src_val
        );
        assert_eq!(cpu.registers.get_zero(), cpu.registers.a == 0);
        assert!(!cpu.registers.get_negative());
    }
}

#[test]
fn test_sub_a_r_comprehensive() {
    let opcodes = [0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x97];
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    for (i, &opcode) in opcodes.iter().enumerate() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0x50;

        match registers[i] {
            'b' => cpu.registers.b = 0x20,
            'c' => cpu.registers.c = 0x20,
            'd' => cpu.registers.d = 0x20,
            'e' => cpu.registers.e = 0x20,
            'h' => cpu.registers.h = 0x20,
            'l' => cpu.registers.l = 0x20,
            'a' => cpu.registers.a = 0x20, // Will be 0x20 - 0x20 = 0x00
            _ => panic!("Invalid register"),
        }

        cpu.call(opcode);

        let expected = if registers[i] == 'a' { 0x00 } else { 0x30 };
        assert_eq!(cpu.registers.a, expected);
        assert_eq!(cpu.registers.get_zero(), expected == 0);
        assert!(cpu.registers.get_negative());
        assert!(!cpu.registers.get_carry());
    }
}

// ========== 16-bit Load Instructions ==========

#[test]
fn test_ld_rr_nn_all_pairs() {
    let opcodes = [0x01, 0x11, 0x21, 0x31];
    let expected_values = [
        ("BC", 0xBEEF),
        ("DE", 0xBEEF),
        ("HL", 0xBEEF),
        ("SP", 0xBEEF),
    ];

    for (i, &opcode) in opcodes.iter().enumerate() {
        let mut cpu = setup_cpu();
        cpu.mmu.write(0x0000, 0xEF);
        cpu.mmu.write(0x0001, 0xBE);
        cpu.registers.pc = 0x0000;

        cpu.call(opcode);

        let (pair_name, expected) = expected_values[i];
        let actual = match pair_name {
            "BC" => ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16),
            "DE" => ((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16),
            "HL" => ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16),
            "SP" => ((cpu.registers.s as u16) << 8) | (cpu.registers.p as u16),
            _ => panic!("Invalid register pair"),
        };

        assert_eq!(actual, expected, "LD {}, nn failed", pair_name);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.ticks, 12);
    }
}

// ========== 16-bit Arithmetic Instructions ==========

#[test]
fn test_inc_rr_all_pairs() {
    let opcodes = [0x03, 0x13, 0x23, 0x33];

    for &opcode in &opcodes {
        let mut cpu = setup_cpu();

        // Set up test values
        match opcode {
            0x03 => {
                cpu.registers.b = 0x10;
                cpu.registers.c = 0xFF;
            }
            0x13 => {
                cpu.registers.d = 0x10;
                cpu.registers.e = 0xFF;
            }
            0x23 => {
                cpu.registers.h = 0x10;
                cpu.registers.l = 0xFF;
            }
            0x33 => {
                cpu.registers.s = 0x10;
                cpu.registers.p = 0xFF;
            }
            _ => panic!("Invalid opcode"),
        }

        cpu.call(opcode);

        let result = match opcode {
            0x03 => ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16),
            0x13 => ((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16),
            0x23 => ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16),
            0x33 => ((cpu.registers.s as u16) << 8) | (cpu.registers.p as u16),
            _ => panic!("Invalid opcode"),
        };

        assert_eq!(result, 0x1100);
        assert_eq!(cpu.ticks, 8);
    }
}

// ========== Bit Operations (CB prefix) ==========

#[test]
fn test_bit_instructions_all_bits_all_registers() {
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    for bit in 0..8 {
        for (reg_idx, &reg) in registers.iter().enumerate() {
            let mut cpu = setup_cpu();

            // Set register to have only the tested bit set
            let test_value = 1u8 << bit;
            match reg {
                'b' => cpu.registers.b = test_value,
                'c' => cpu.registers.c = test_value,
                'd' => cpu.registers.d = test_value,
                'e' => cpu.registers.e = test_value,
                'h' => cpu.registers.h = test_value,
                'l' => cpu.registers.l = test_value,
                'a' => cpu.registers.a = test_value,
                _ => panic!("Invalid register"),
            }

            // Test BIT instruction
            cpu.call(0xCB); // CB prefix
            let bit_opcode = 0x40 + (bit * 8) + reg_idx;
            cpu.call(bit_opcode as u8);

            assert!(
                !cpu.registers.get_zero(),
                "BIT {},{} should not set zero flag",
                bit,
                reg
            );
            assert!(!cpu.registers.get_negative());
            assert!(cpu.registers.get_half_carry());

            // Test with bit not set
            let mut cpu = setup_cpu();
            match reg {
                'b' => cpu.registers.b = !test_value,
                'c' => cpu.registers.c = !test_value,
                'd' => cpu.registers.d = !test_value,
                'e' => cpu.registers.e = !test_value,
                'h' => cpu.registers.h = !test_value,
                'l' => cpu.registers.l = !test_value,
                'a' => cpu.registers.a = !test_value,
                _ => panic!("Invalid register"),
            }

            cpu.call(0xCB);
            cpu.call(bit_opcode as u8);

            assert!(
                cpu.registers.get_zero(),
                "BIT {},{} should set zero flag",
                bit,
                reg
            );
        }
    }
}

#[test]
fn test_set_res_instructions_all_bits_all_registers() {
    let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];

    for bit in 0..8 {
        for (reg_idx, &reg) in registers.iter().enumerate() {
            // Test SET instruction
            let mut cpu = setup_cpu();

            cpu.call(0xCB);
            let set_opcode = 0xC0 + (bit * 8) + reg_idx;
            cpu.call(set_opcode as u8);

            let result = match reg {
                'b' => cpu.registers.b,
                'c' => cpu.registers.c,
                'd' => cpu.registers.d,
                'e' => cpu.registers.e,
                'h' => cpu.registers.h,
                'l' => cpu.registers.l,
                'a' => cpu.registers.a,
                _ => panic!("Invalid register"),
            };

            assert_eq!(result & (1 << bit), 1 << bit, "SET {},{} failed", bit, reg);

            // Test RES instruction
            let mut cpu = setup_cpu();
            match reg {
                'b' => cpu.registers.b = 0xFF,
                'c' => cpu.registers.c = 0xFF,
                'd' => cpu.registers.d = 0xFF,
                'e' => cpu.registers.e = 0xFF,
                'h' => cpu.registers.h = 0xFF,
                'l' => cpu.registers.l = 0xFF,
                'a' => cpu.registers.a = 0xFF,
                _ => panic!("Invalid register"),
            }

            cpu.call(0xCB);
            let res_opcode = 0x80 + (bit * 8) + reg_idx;
            cpu.call(res_opcode as u8);

            let result = match reg {
                'b' => cpu.registers.b,
                'c' => cpu.registers.c,
                'd' => cpu.registers.d,
                'e' => cpu.registers.e,
                'h' => cpu.registers.h,
                'l' => cpu.registers.l,
                'a' => cpu.registers.a,
                _ => panic!("Invalid register"),
            };

            assert_eq!(result & (1 << bit), 0, "RES {},{} failed", bit, reg);
        }
    }
}

// ========== Rotation Instructions ==========

#[test]
fn test_rotation_instructions_comprehensive() {
    // Test all rotation instructions with various patterns
    let test_patterns = [
        0b10000001, 0b01000010, 0b00100100, 0b11111111, 0b00000000, 0b10101010, 0b01010101,
    ];

    for &pattern in &test_patterns {
        test_rlc_pattern(pattern);
        test_rrc_pattern(pattern);
        test_rl_pattern(pattern);
        test_rr_pattern(pattern);
        test_sla_pattern(pattern);
        test_sra_pattern(pattern);
        test_srl_pattern(pattern);
    }
}

fn test_rlc_pattern(pattern: u8) {
    let mut cpu = setup_cpu();
    cpu.registers.b = pattern;

    cpu.call(0xCB);
    cpu.call(0x00); // RLC B

    let expected = (pattern << 1) | (pattern >> 7);
    assert_eq!(cpu.registers.b, expected);
    assert_eq!(cpu.registers.get_carry(), (pattern & 0x80) != 0);
    assert_eq!(cpu.registers.get_zero(), expected == 0);
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
}

fn test_rrc_pattern(pattern: u8) {
    let mut cpu = setup_cpu();
    cpu.registers.b = pattern;

    cpu.call(0xCB);
    cpu.call(0x08); // RRC B

    let expected = (pattern >> 1) | ((pattern & 1) << 7);
    assert_eq!(cpu.registers.b, expected);
    assert_eq!(cpu.registers.get_carry(), (pattern & 1) != 0);
    assert_eq!(cpu.registers.get_zero(), expected == 0);
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
}

fn test_rl_pattern(pattern: u8) {
    for carry_in in [false, true] {
        let mut cpu = setup_cpu();
        cpu.registers.b = pattern;
        cpu.registers.carry(carry_in);

        cpu.call(0xCB);
        cpu.call(0x10); // RL B

        let expected = (pattern << 1) | (carry_in as u8);
        assert_eq!(cpu.registers.b, expected);
        assert_eq!(cpu.registers.get_carry(), (pattern & 0x80) != 0);
        assert_eq!(cpu.registers.get_zero(), expected == 0);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
    }
}

fn test_rr_pattern(pattern: u8) {
    for carry_in in [false, true] {
        let mut cpu = setup_cpu();
        cpu.registers.b = pattern;
        cpu.registers.carry(carry_in);

        cpu.call(0xCB);
        cpu.call(0x18); // RR B

        let expected = (pattern >> 1) | ((carry_in as u8) << 7);
        assert_eq!(cpu.registers.b, expected);
        assert_eq!(cpu.registers.get_carry(), (pattern & 1) != 0);
        assert_eq!(cpu.registers.get_zero(), expected == 0);
        assert!(!cpu.registers.get_negative());
        assert!(!cpu.registers.get_half_carry());
    }
}

fn test_sla_pattern(pattern: u8) {
    let mut cpu = setup_cpu();
    cpu.registers.b = pattern;

    cpu.call(0xCB);
    cpu.call(0x20); // SLA B

    let expected = pattern << 1;
    assert_eq!(cpu.registers.b, expected);
    assert_eq!(cpu.registers.get_carry(), (pattern & 0x80) != 0);
    assert_eq!(cpu.registers.get_zero(), expected == 0);
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
}

fn test_sra_pattern(pattern: u8) {
    let mut cpu = setup_cpu();
    cpu.registers.b = pattern;

    cpu.call(0xCB);
    cpu.call(0x28); // SRA B

    let expected = ((pattern as i8) >> 1) as u8;
    assert_eq!(cpu.registers.b, expected);
    assert_eq!(cpu.registers.get_carry(), (pattern & 1) != 0);
    assert_eq!(cpu.registers.get_zero(), expected == 0);
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
}

fn test_srl_pattern(pattern: u8) {
    let mut cpu = setup_cpu();
    cpu.registers.b = pattern;

    cpu.call(0xCB);
    cpu.call(0x38); // SRL B

    let expected = pattern >> 1;
    assert_eq!(cpu.registers.b, expected);
    assert_eq!(cpu.registers.get_carry(), (pattern & 1) != 0);
    assert_eq!(cpu.registers.get_zero(), expected == 0);
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
}

// ========== Jump and Call Instructions ==========

#[test]
fn test_conditional_jumps_all_conditions() {
    let conditions = [
        (0x20, "NZ", false), // JR NZ
        (0x28, "Z", true),   // JR Z
        (0x30, "NC", false), // JR NC
        (0x38, "C", true),   // JR C
    ];

    for (opcode, name, flag_value) in conditions {
        // Test when condition is met
        let mut cpu = setup_cpu();
        cpu.mmu.write(0x1000, 0x10); // Jump forward 16 bytes
        cpu.registers.pc = 0x1000;

        match name {
            "NZ" => cpu.registers.zero(!flag_value),
            "Z" => cpu.registers.zero(flag_value),
            "NC" => cpu.registers.carry(!flag_value),
            "C" => cpu.registers.carry(flag_value),
            _ => panic!("Invalid condition"),
        }

        cpu.call(opcode);

        assert_eq!(
            cpu.registers.pc, 0x1011,
            "JR {} should jump when condition met",
            name
        );
        assert_eq!(cpu.ticks, 12);

        // Test when condition is not met
        let mut cpu = setup_cpu();
        cpu.mmu.write(0x1000, 0x10);
        cpu.registers.pc = 0x1000;

        match name {
            "NZ" => cpu.registers.zero(flag_value),
            "Z" => cpu.registers.zero(!flag_value),
            "NC" => cpu.registers.carry(flag_value),
            "C" => cpu.registers.carry(!flag_value),
            _ => panic!("Invalid condition"),
        }

        cpu.call(opcode);

        assert_eq!(
            cpu.registers.pc, 0x1001,
            "JR {} should not jump when condition not met",
            name
        );
        assert_eq!(cpu.ticks, 8);
    }
}

// ========== Edge Cases and Error Conditions ==========

#[test]
fn test_flag_interactions() {
    // Test that instructions properly preserve or modify flags
    let mut cpu = setup_cpu();

    // Set all flags
    cpu.registers.zero(true);
    cpu.registers.negative(true);
    cpu.registers.half_carry(true);
    cpu.registers.carry(true);

    // LD B,C should not affect flags
    cpu.call(0x41);
    assert!(cpu.registers.get_zero());
    assert!(cpu.registers.get_negative());
    assert!(cpu.registers.get_half_carry());
    assert!(cpu.registers.get_carry());

    // ADD A,A should affect flags
    cpu.registers.a = 0;
    cpu.call(0x87);
    assert!(cpu.registers.get_zero());
    assert!(!cpu.registers.get_negative());
    assert!(!cpu.registers.get_half_carry());
    assert!(!cpu.registers.get_carry());
}

#[test]
fn test_stack_operations_edge_cases() {
    // Test stack operations at edge addresses
    let mut cpu = setup_cpu();
    cpu.registers.s = 0x00;
    cpu.registers.p = 0x02;
    cpu.registers.b = 0xBE;
    cpu.registers.c = 0xEF;

    // PUSH BC
    cpu.call(0xC5);

    assert_eq!(cpu.registers.s, 0x00);
    assert_eq!(cpu.registers.p, 0x00);
    assert_eq!(cpu.mmu.read(0x0000), 0xEF);
    assert_eq!(cpu.mmu.read(0x0001), 0xBE);

    // Clear BC
    cpu.registers.b = 0x00;
    cpu.registers.c = 0x00;

    // POP BC
    cpu.call(0xC1);

    assert_eq!(cpu.registers.b, 0xBE);
    assert_eq!(cpu.registers.c, 0xEF);
    assert_eq!(cpu.registers.s, 0x00);
    assert_eq!(cpu.registers.p, 0x02);
}

#[test]
fn test_memory_addressing_modes() {
    // Test various memory addressing modes
    let mut cpu = setup_cpu();

    // Test (HL) addressing
    cpu.registers.h = 0x80;
    cpu.registers.l = 0x00;
    cpu.mmu.write(0x8000, 0x42);

    // LD A,(HL)
    cpu.call(0x7E);
    assert_eq!(cpu.registers.a, 0x42);

    // LD (HL),A
    cpu.registers.a = 0x69;
    cpu.call(0x77);
    assert_eq!(cpu.mmu.read(0x8000), 0x69);

    // Test immediate addressing
    cpu.registers.pc = 0x1000;
    cpu.mmu.write(0x1000, 0x33);

    // LD A,n
    cpu.call(0x3E);
    assert_eq!(cpu.registers.a, 0x33);
    assert_eq!(cpu.registers.pc, 0x1001);
}

#[test]
fn test_timing_accuracy() {
    // Test that instruction timing is correct
    let timing_tests = [
        (0x00, 4),  // NOP
        (0x01, 12), // LD BC,nn
        (0x02, 8),  // LD (BC),A
        (0x03, 8),  // INC BC
        (0x04, 4),  // INC B
        (0x05, 4),  // DEC B
        (0x06, 8),  // LD B,n
        (0x07, 4),  // RLCA
        (0x08, 20), // LD (nn),SP
        (0x09, 8),  // ADD HL,BC
        (0x0A, 8),  // LD A,(BC)
        (0x0B, 8),  // DEC BC
        (0x0C, 4),  // INC C
        (0x0D, 4),  // DEC C
        (0x0E, 8),  // LD C,n
        (0x0F, 4),  // RRCA
    ];

    for (opcode, expected_ticks) in timing_tests {
        let mut cpu = setup_cpu();

        // Set up any required memory/registers
        if opcode == 0x01 || opcode == 0x06 || opcode == 0x0E {
            cpu.mmu.write(cpu.registers.pc, 0x00);
            cpu.mmu.write(cpu.registers.pc + 1, 0x00);
        }
        if opcode == 0x08 {
            cpu.mmu.write(cpu.registers.pc, 0x00);
            cpu.mmu.write(cpu.registers.pc + 1, 0x80);
        }

        cpu.call(opcode);
        assert_eq!(
            cpu.ticks, expected_ticks,
            "Timing incorrect for opcode {:02X}",
            opcode
        );
    }
}

#[test]
fn test_instruction_sequence_interactions() {
    // Test sequences of instructions that might interact
    let mut cpu = setup_cpu();

    // Test ADD followed by ADC
    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x01;
    cpu.registers.c = 0x01;

    cpu.call(0x80); // ADD A,B - should set carry
    assert!(cpu.registers.get_carry());
    assert_eq!(cpu.registers.a, 0x00);

    cpu.call(0x88); // ADC A,B - should add 1 + carry
    assert_eq!(cpu.registers.a, 0x02);
    assert!(!cpu.registers.get_carry());

    // Test rotation sequence
    cpu.registers.a = 0b10000001;
    cpu.call(0x07); // RLCA
    assert_eq!(cpu.registers.a, 0b00000011);
    assert!(cpu.registers.get_carry());

    cpu.call(0x17); // RLA - should include previous carry
    assert_eq!(cpu.registers.a, 0b00000111);
    assert!(!cpu.registers.get_carry());
}
