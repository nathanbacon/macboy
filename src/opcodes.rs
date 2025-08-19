/// Complete enumeration of all GameBoy CPU opcodes
/// This provides type-safe access to all CPU instructions and eliminates magic numbers

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // 8-bit Load Instructions
    LdBB = 0x40, LdBC = 0x41, LdBD = 0x42, LdBE = 0x43, LdBH = 0x44, LdBL = 0x45, LdBHLPtr = 0x46, LdBA = 0x47,
    LdCB = 0x48, LdCC = 0x49, LdCD = 0x4A, LdCE = 0x4B, LdCH = 0x4C, LdCL = 0x4D, LdCHLPtr = 0x4E, LdCA = 0x4F,
    LdDB = 0x50, LdDC = 0x51, LdDD = 0x52, LdDE = 0x53, LdDH = 0x54, LdDL = 0x55, LdDHLPtr = 0x56, LdDA = 0x57,
    LdEB = 0x58, LdEC = 0x59, LdED = 0x5A, LdEE = 0x5B, LdEH = 0x5C, LdEL = 0x5D, LdEHLPtr = 0x5E, LdEA = 0x5F,
    LdHB = 0x60, LdHC = 0x61, LdHD = 0x62, LdHE = 0x63, LdHH = 0x64, LdHL = 0x65, LdHHLPtr = 0x66, LdHA = 0x67,
    LdLB = 0x68, LdLC = 0x69, LdLD = 0x6A, LdLE = 0x6B, LdLH = 0x6C, LdLL = 0x6D, LdLHLPtr = 0x6E, LdLA = 0x6F,
    LdHLPtrB = 0x70, LdHLPtrC = 0x71, LdHLPtrD = 0x72, LdHLPtrE = 0x73, LdHLPtrH = 0x74, LdHLPtrL = 0x75, Halt = 0x76, LdHLPtrA = 0x77,
    LdAB = 0x78, LdAC = 0x79, LdAD = 0x7A, LdAE = 0x7B, LdAH = 0x7C, LdAL = 0x7D, LdAHLPtr = 0x7E, LdAA = 0x7F,

    // 8-bit Load Immediate
    LdBN = 0x06, LdCN = 0x0E, LdDN = 0x16, LdEN = 0x1E,
    LdHN = 0x26, LdLN = 0x2E, LdHLPtrN = 0x36, LdAN = 0x3E,

    // 16-bit Load Instructions
    LdBCNN = 0x01, LdDENN = 0x11, LdHLNN = 0x21, LdSPNN = 0x31,

    // Memory Load Instructions
    LdBCPtrA = 0x02, LdDEPtrA = 0x12,
    LdABCPtr = 0x0A, LdADEPtr = 0x1A,
    LdNNPtrA = 0xEA, LdANNPtr = 0xFA,
    LdHLIncA = 0x22, LdHLDecA = 0x32,
    LdAHLInc = 0x2A, LdAHLDec = 0x3A,

    // 8-bit Arithmetic
    AddAB = 0x80, AddAC = 0x81, AddAD = 0x82, AddAE = 0x83, AddAH = 0x84, AddAL = 0x85, AddAHLPtr = 0x86, AddAA = 0x87,
    AdcAB = 0x88, AdcAC = 0x89, AdcAD = 0x8A, AdcAE = 0x8B, AdcAH = 0x8C, AdcAL = 0x8D, AdcAHLPtr = 0x8E, AdcAA = 0x8F,
    SubB = 0x90, SubC = 0x91, SubD = 0x92, SubE = 0x93, SubH = 0x94, SubL = 0x95, SubHLPtr = 0x96, SubA = 0x97,
    SbcAB = 0x98, SbcAC = 0x99, SbcAD = 0x9A, SbcAE = 0x9B, SbcAH = 0x9C, SbcAL = 0x9D, SbcAHLPtr = 0x9E, SbcAA = 0x9F,
    AndB = 0xA0, AndC = 0xA1, AndD = 0xA2, AndE = 0xA3, AndH = 0xA4, AndL = 0xA5, AndHLPtr = 0xA6, AndA = 0xA7,
    XorB = 0xA8, XorC = 0xA9, XorD = 0xAA, XorE = 0xAB, XorH = 0xAC, XorL = 0xAD, XorHLPtr = 0xAE, XorA = 0xAF,
    OrB = 0xB0, OrC = 0xB1, OrD = 0xB2, OrE = 0xB3, OrH = 0xB4, OrL = 0xB5, OrHLPtr = 0xB6, OrA = 0xB7,
    CpB = 0xB8, CpC = 0xB9, CpD = 0xBA, CpE = 0xBB, CpH = 0xBC, CpL = 0xBD, CpHLPtr = 0xBE, CpA = 0xBF,

    // 8-bit Arithmetic Immediate
    AddAN = 0xC6, AdcAN = 0xCE, SubN = 0xD6, SbcAN = 0xDE,
    AndN = 0xE6, XorN = 0xEE, OrN = 0xF6, CpN = 0xFE,

    // 16-bit Arithmetic
    AddHLBC = 0x09, AddHLDE = 0x19, AddHLHL = 0x29, AddHLSP = 0x39,
    AddSPN = 0xE8,

    // 8-bit Increment/Decrement
    IncB = 0x04, IncC = 0x0C, IncD = 0x14, IncE = 0x1C,
    IncH = 0x24, IncL = 0x2C, IncHLPtr = 0x34, IncA = 0x3C,
    DecB = 0x05, DecC = 0x0D, DecD = 0x15, DecE = 0x1D,
    DecH = 0x25, DecL = 0x2D, DecHLPtr = 0x35, DecA = 0x3D,

    // 16-bit Increment/Decrement
    IncBC = 0x03, IncDE = 0x13, IncHL = 0x23, IncSP = 0x33,
    DecBC = 0x0B, DecDE = 0x1B, DecHL = 0x2B, DecSP = 0x3B,

    // Rotate and Shift Instructions
    Rlca = 0x07, Rrca = 0x0F, Rla = 0x17, Rra = 0x1F,
    Daa = 0x27, Cpl = 0x2F, Scf = 0x37, Ccf = 0x3F,

    // Jump Instructions
    JpNN = 0xC3, JpHL = 0xE9,
    JpNZNN = 0xC2, JpZNN = 0xCA, JpNCNN = 0xD2, JpCNN = 0xDA,
    JrN = 0x18,
    JrNZN = 0x20, JrZN = 0x28, JrNCN = 0x30, JrCN = 0x38,

    // Call and Return Instructions
    CallNN = 0xCD,
    CallNZNN = 0xC4, CallZNN = 0xCC, CallNCNN = 0xD4, CallCNN = 0xDC,
    Ret = 0xC9, Reti = 0xD9,
    RetNZ = 0xC0, RetZ = 0xC8, RetNC = 0xD0, RetC = 0xD8,

    // Restart Instructions
    Rst00 = 0xC7, Rst08 = 0xCF, Rst10 = 0xD7, Rst18 = 0xDF,
    Rst20 = 0xE7, Rst28 = 0xEF, Rst30 = 0xF7, Rst38 = 0xFF,

    // Stack Operations
    PushBC = 0xC5, PushDE = 0xD5, PushHL = 0xE5, PushAF = 0xF5,
    PopBC = 0xC1, PopDE = 0xD1, PopHL = 0xE1, PopAF = 0xF1,

    // Miscellaneous
    Nop = 0x00, Stop = 0x10, Di = 0xF3, Ei = 0xFB,

    // High Memory Operations
    LdhNA = 0xE0, LdhAN = 0xF0,
    LdhCA = 0xE2, LdhAC = 0xF2,

    // Stack Pointer Operations
    LdHLSPN = 0xF8, LdSPHL = 0xF9, LdNNSP = 0x08,

    // CB Prefix - Extended Instructions (0xCB prefix)
    CbPrefix = 0xCB,
    
    // Invalid opcodes (these don't exist on the GameBoy)
    Invalid = 0x100, // Use a value outside the u8 range
}

/// Extended opcodes for CB-prefixed instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtendedOpcode {
    // Rotate Left Circular
    RlcB = 0x00, RlcC = 0x01, RlcD = 0x02, RlcE = 0x03, RlcH = 0x04, RlcL = 0x05, RlcHLPtr = 0x06, RlcA = 0x07,
    
    // Rotate Right Circular
    RrcB = 0x08, RrcC = 0x09, RrcD = 0x0A, RrcE = 0x0B, RrcH = 0x0C, RrcL = 0x0D, RrcHLPtr = 0x0E, RrcA = 0x0F,
    
    // Rotate Left
    RlB = 0x10, RlC = 0x11, RlD = 0x12, RlE = 0x13, RlH = 0x14, RlL = 0x15, RlHLPtr = 0x16, RlA = 0x17,
    
    // Rotate Right
    RrB = 0x18, RrC = 0x19, RrD = 0x1A, RrE = 0x1B, RrH = 0x1C, RrL = 0x1D, RrHLPtr = 0x1E, RrA = 0x1F,
    
    // Shift Left Arithmetic
    SlaB = 0x20, SlaC = 0x21, SlaD = 0x22, SlaE = 0x23, SlaH = 0x24, SlaL = 0x25, SlaHLPtr = 0x26, SlaA = 0x27,
    
    // Shift Right Arithmetic
    SraB = 0x28, SraC = 0x29, SraD = 0x2A, SraE = 0x2B, SraH = 0x2C, SraL = 0x2D, SraHLPtr = 0x2E, SraA = 0x2F,
    
    // Swap
    SwapB = 0x30, SwapC = 0x31, SwapD = 0x32, SwapE = 0x33, SwapH = 0x34, SwapL = 0x35, SwapHLPtr = 0x36, SwapA = 0x37,
    
    // Shift Right Logical
    SrlB = 0x38, SrlC = 0x39, SrlD = 0x3A, SrlE = 0x3B, SrlH = 0x3C, SrlL = 0x3D, SrlHLPtr = 0x3E, SrlA = 0x3F,
    
    // Bit Test Instructions (BIT b,r)
    Bit0B = 0x40, Bit0C = 0x41, Bit0D = 0x42, Bit0E = 0x43, Bit0H = 0x44, Bit0L = 0x45, Bit0HLPtr = 0x46, Bit0A = 0x47,
    Bit1B = 0x48, Bit1C = 0x49, Bit1D = 0x4A, Bit1E = 0x4B, Bit1H = 0x4C, Bit1L = 0x4D, Bit1HLPtr = 0x4E, Bit1A = 0x4F,
    Bit2B = 0x50, Bit2C = 0x51, Bit2D = 0x52, Bit2E = 0x53, Bit2H = 0x54, Bit2L = 0x55, Bit2HLPtr = 0x56, Bit2A = 0x57,
    Bit3B = 0x58, Bit3C = 0x59, Bit3D = 0x5A, Bit3E = 0x5B, Bit3H = 0x5C, Bit3L = 0x5D, Bit3HLPtr = 0x5E, Bit3A = 0x5F,
    Bit4B = 0x60, Bit4C = 0x61, Bit4D = 0x62, Bit4E = 0x63, Bit4H = 0x64, Bit4L = 0x65, Bit4HLPtr = 0x66, Bit4A = 0x67,
    Bit5B = 0x68, Bit5C = 0x69, Bit5D = 0x6A, Bit5E = 0x6B, Bit5H = 0x6C, Bit5L = 0x6D, Bit5HLPtr = 0x6E, Bit5A = 0x6F,
    Bit6B = 0x70, Bit6C = 0x71, Bit6D = 0x72, Bit6E = 0x73, Bit6H = 0x74, Bit6L = 0x75, Bit6HLPtr = 0x76, Bit6A = 0x77,
    Bit7B = 0x78, Bit7C = 0x79, Bit7D = 0x7A, Bit7E = 0x7B, Bit7H = 0x7C, Bit7L = 0x7D, Bit7HLPtr = 0x7E, Bit7A = 0x7F,
    
    // Reset Bit Instructions (RES b,r)
    Res0B = 0x80, Res0C = 0x81, Res0D = 0x82, Res0E = 0x83, Res0H = 0x84, Res0L = 0x85, Res0HLPtr = 0x86, Res0A = 0x87,
    Res1B = 0x88, Res1C = 0x89, Res1D = 0x8A, Res1E = 0x8B, Res1H = 0x8C, Res1L = 0x8D, Res1HLPtr = 0x8E, Res1A = 0x8F,
    Res2B = 0x90, Res2C = 0x91, Res2D = 0x92, Res2E = 0x93, Res2H = 0x94, Res2L = 0x95, Res2HLPtr = 0x96, Res2A = 0x97,
    Res3B = 0x98, Res3C = 0x99, Res3D = 0x9A, Res3E = 0x9B, Res3H = 0x9C, Res3L = 0x9D, Res3HLPtr = 0x9E, Res3A = 0x9F,
    Res4B = 0xA0, Res4C = 0xA1, Res4D = 0xA2, Res4E = 0xA3, Res4H = 0xA4, Res4L = 0xA5, Res4HLPtr = 0xA6, Res4A = 0xA7,
    Res5B = 0xA8, Res5C = 0xA9, Res5D = 0xAA, Res5E = 0xAB, Res5H = 0xAC, Res5L = 0xAD, Res5HLPtr = 0xAE, Res5A = 0xAF,
    Res6B = 0xB0, Res6C = 0xB1, Res6D = 0xB2, Res6E = 0xB3, Res6H = 0xB4, Res6L = 0xB5, Res6HLPtr = 0xB6, Res6A = 0xB7,
    Res7B = 0xB8, Res7C = 0xB9, Res7D = 0xBA, Res7E = 0xBB, Res7H = 0xBC, Res7L = 0xBD, Res7HLPtr = 0xBE, Res7A = 0xBF,
    
    // Set Bit Instructions (SET b,r)
    Set0B = 0xC0, Set0C = 0xC1, Set0D = 0xC2, Set0E = 0xC3, Set0H = 0xC4, Set0L = 0xC5, Set0HLPtr = 0xC6, Set0A = 0xC7,
    Set1B = 0xC8, Set1C = 0xC9, Set1D = 0xCA, Set1E = 0xCB, Set1H = 0xCC, Set1L = 0xCD, Set1HLPtr = 0xCE, Set1A = 0xCF,
    Set2B = 0xD0, Set2C = 0xD1, Set2D = 0xD2, Set2E = 0xD3, Set2H = 0xD4, Set2L = 0xD5, Set2HLPtr = 0xD6, Set2A = 0xD7,
    Set3B = 0xD8, Set3C = 0xD9, Set3D = 0xDA, Set3E = 0xDB, Set3H = 0xDC, Set3L = 0xDD, Set3HLPtr = 0xDE, Set3A = 0xDF,
    Set4B = 0xE0, Set4C = 0xE1, Set4D = 0xE2, Set4E = 0xE3, Set4H = 0xE4, Set4L = 0xE5, Set4HLPtr = 0xE6, Set4A = 0xE7,
    Set5B = 0xE8, Set5C = 0xE9, Set5D = 0xEA, Set5E = 0xEB, Set5H = 0xEC, Set5L = 0xED, Set5HLPtr = 0xEE, Set5A = 0xEF,
    Set6B = 0xF0, Set6C = 0xF1, Set6D = 0xF2, Set6E = 0xF3, Set6H = 0xF4, Set6L = 0xF5, Set6HLPtr = 0xF6, Set6A = 0xF7,
    Set7B = 0xF8, Set7C = 0xF9, Set7D = 0xFA, Set7E = 0xFB, Set7H = 0xFC, Set7L = 0xFD, Set7HLPtr = 0xFE, Set7A = 0xFF,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Nop,
            0x01 => Opcode::LdBCNN,
            0x02 => Opcode::LdBCPtrA,
            0x03 => Opcode::IncBC,
            0x04 => Opcode::IncB,
            0x05 => Opcode::DecB,
            0x06 => Opcode::LdBN,
            0x07 => Opcode::Rlca,
            0x08 => Opcode::LdNNSP,
            0x09 => Opcode::AddHLBC,
            0x0A => Opcode::LdABCPtr,
            0x0B => Opcode::DecBC,
            0x0C => Opcode::IncC,
            0x0D => Opcode::DecC,
            0x0E => Opcode::LdCN,
            0x0F => Opcode::Rrca,
            0x10 => Opcode::Stop,
            0x11 => Opcode::LdDENN,
            0x12 => Opcode::LdDEPtrA,
            0x13 => Opcode::IncDE,
            0x14 => Opcode::IncD,
            0x15 => Opcode::DecD,
            0x16 => Opcode::LdDN,
            0x17 => Opcode::Rla,
            0x18 => Opcode::JrN,
            0x19 => Opcode::AddHLDE,
            0x1A => Opcode::LdADEPtr,
            0x1B => Opcode::DecDE,
            0x1C => Opcode::IncE,
            0x1D => Opcode::DecE,
            0x1E => Opcode::LdEN,
            0x1F => Opcode::Rra,
            0x20 => Opcode::JrNZN,
            0x21 => Opcode::LdHLNN,
            0x22 => Opcode::LdHLIncA,
            0x23 => Opcode::IncHL,
            0x24 => Opcode::IncH,
            0x25 => Opcode::DecH,
            0x26 => Opcode::LdHN,
            0x27 => Opcode::Daa,
            0x28 => Opcode::JrZN,
            0x29 => Opcode::AddHLHL,
            0x2A => Opcode::LdAHLInc,
            0x2B => Opcode::DecHL,
            0x2C => Opcode::IncL,
            0x2D => Opcode::DecL,
            0x2E => Opcode::LdLN,
            0x2F => Opcode::Cpl,
            0x30 => Opcode::JrNCN,
            0x31 => Opcode::LdSPNN,
            0x32 => Opcode::LdHLDecA,
            0x33 => Opcode::IncSP,
            0x34 => Opcode::IncHLPtr,
            0x35 => Opcode::DecHLPtr,
            0x36 => Opcode::LdHLPtrN,
            0x37 => Opcode::Scf,
            0x38 => Opcode::JrCN,
            0x39 => Opcode::AddHLSP,
            0x3A => Opcode::LdAHLDec,
            0x3B => Opcode::DecSP,
            0x3C => Opcode::IncA,
            0x3D => Opcode::DecA,
            0x3E => Opcode::LdAN,
            0x3F => Opcode::Ccf,
            0x40 => Opcode::LdBB,
            0x41 => Opcode::LdBC,
            0x42 => Opcode::LdBD,
            0x43 => Opcode::LdBE,
            0x44 => Opcode::LdBH,
            0x45 => Opcode::LdBL,
            0x46 => Opcode::LdBHLPtr,
            0x47 => Opcode::LdBA,
            0x48 => Opcode::LdCB,
            0x49 => Opcode::LdCC,
            0x4A => Opcode::LdCD,
            0x4B => Opcode::LdCE,
            0x4C => Opcode::LdCH,
            0x4D => Opcode::LdCL,
            0x4E => Opcode::LdCHLPtr,
            0x4F => Opcode::LdCA,
            0x50 => Opcode::LdDB,
            0x51 => Opcode::LdDC,
            0x52 => Opcode::LdDD,
            0x53 => Opcode::LdDE,
            0x54 => Opcode::LdDH,
            0x55 => Opcode::LdDL,
            0x56 => Opcode::LdDHLPtr,
            0x57 => Opcode::LdDA,
            0x58 => Opcode::LdEB,
            0x59 => Opcode::LdEC,
            0x5A => Opcode::LdED,
            0x5B => Opcode::LdEE,
            0x5C => Opcode::LdEH,
            0x5D => Opcode::LdEL,
            0x5E => Opcode::LdEHLPtr,
            0x5F => Opcode::LdEA,
            0x60 => Opcode::LdHB,
            0x61 => Opcode::LdHC,
            0x62 => Opcode::LdHD,
            0x63 => Opcode::LdHE,
            0x64 => Opcode::LdHH,
            0x65 => Opcode::LdHL,
            0x66 => Opcode::LdHHLPtr,
            0x67 => Opcode::LdHA,
            0x68 => Opcode::LdLB,
            0x69 => Opcode::LdLC,
            0x6A => Opcode::LdLD,
            0x6B => Opcode::LdLE,
            0x6C => Opcode::LdLH,
            0x6D => Opcode::LdLL,
            0x6E => Opcode::LdLHLPtr,
            0x6F => Opcode::LdLA,
            0x70 => Opcode::LdHLPtrB,
            0x71 => Opcode::LdHLPtrC,
            0x72 => Opcode::LdHLPtrD,
            0x73 => Opcode::LdHLPtrE,
            0x74 => Opcode::LdHLPtrH,
            0x75 => Opcode::LdHLPtrL,
            0x76 => Opcode::Halt,
            0x77 => Opcode::LdHLPtrA,
            0x78 => Opcode::LdAB,
            0x79 => Opcode::LdAC,
            0x7A => Opcode::LdAD,
            0x7B => Opcode::LdAE,
            0x7C => Opcode::LdAH,
            0x7D => Opcode::LdAL,
            0x7E => Opcode::LdAHLPtr,
            0x7F => Opcode::LdAA,
            0x80 => Opcode::AddAB,
            0x81 => Opcode::AddAC,
            0x82 => Opcode::AddAD,
            0x83 => Opcode::AddAE,
            0x84 => Opcode::AddAH,
            0x85 => Opcode::AddAL,
            0x86 => Opcode::AddAHLPtr,
            0x87 => Opcode::AddAA,
            0x88 => Opcode::AdcAB,
            0x89 => Opcode::AdcAC,
            0x8A => Opcode::AdcAD,
            0x8B => Opcode::AdcAE,
            0x8C => Opcode::AdcAH,
            0x8D => Opcode::AdcAL,
            0x8E => Opcode::AdcAHLPtr,
            0x8F => Opcode::AdcAA,
            0x90 => Opcode::SubB,
            0x91 => Opcode::SubC,
            0x92 => Opcode::SubD,
            0x93 => Opcode::SubE,
            0x94 => Opcode::SubH,
            0x95 => Opcode::SubL,
            0x96 => Opcode::SubHLPtr,
            0x97 => Opcode::SubA,
            0x98 => Opcode::SbcAB,
            0x99 => Opcode::SbcAC,
            0x9A => Opcode::SbcAD,
            0x9B => Opcode::SbcAE,
            0x9C => Opcode::SbcAH,
            0x9D => Opcode::SbcAL,
            0x9E => Opcode::SbcAHLPtr,
            0x9F => Opcode::SbcAA,
            0xA0 => Opcode::AndB,
            0xA1 => Opcode::AndC,
            0xA2 => Opcode::AndD,
            0xA3 => Opcode::AndE,
            0xA4 => Opcode::AndH,
            0xA5 => Opcode::AndL,
            0xA6 => Opcode::AndHLPtr,
            0xA7 => Opcode::AndA,
            0xA8 => Opcode::XorB,
            0xA9 => Opcode::XorC,
            0xAA => Opcode::XorD,
            0xAB => Opcode::XorE,
            0xAC => Opcode::XorH,
            0xAD => Opcode::XorL,
            0xAE => Opcode::XorHLPtr,
            0xAF => Opcode::XorA,
            0xB0 => Opcode::OrB,
            0xB1 => Opcode::OrC,
            0xB2 => Opcode::OrD,
            0xB3 => Opcode::OrE,
            0xB4 => Opcode::OrH,
            0xB5 => Opcode::OrL,
            0xB6 => Opcode::OrHLPtr,
            0xB7 => Opcode::OrA,
            0xB8 => Opcode::CpB,
            0xB9 => Opcode::CpC,
            0xBA => Opcode::CpD,
            0xBB => Opcode::CpE,
            0xBC => Opcode::CpH,
            0xBD => Opcode::CpL,
            0xBE => Opcode::CpHLPtr,
            0xBF => Opcode::CpA,
            0xC0 => Opcode::RetNZ,
            0xC1 => Opcode::PopBC,
            0xC2 => Opcode::JpNZNN,
            0xC3 => Opcode::JpNN,
            0xC4 => Opcode::CallNZNN,
            0xC5 => Opcode::PushBC,
            0xC6 => Opcode::AddAN,
            0xC7 => Opcode::Rst00,
            0xC8 => Opcode::RetZ,
            0xC9 => Opcode::Ret,
            0xCA => Opcode::JpZNN,
            0xCB => Opcode::CbPrefix,
            0xCC => Opcode::CallZNN,
            0xCD => Opcode::CallNN,
            0xCE => Opcode::AdcAN,
            0xCF => Opcode::Rst08,
            0xD0 => Opcode::RetNC,
            0xD1 => Opcode::PopDE,
            0xD2 => Opcode::JpNCNN,
            0xD3 => Opcode::Invalid,
            0xD4 => Opcode::CallNCNN,
            0xD5 => Opcode::PushDE,
            0xD6 => Opcode::SubN,
            0xD7 => Opcode::Rst10,
            0xD8 => Opcode::RetC,
            0xD9 => Opcode::Reti,
            0xDA => Opcode::JpCNN,
            0xDB => Opcode::Invalid,
            0xDC => Opcode::CallCNN,
            0xDD => Opcode::Invalid,
            0xDE => Opcode::SbcAN,
            0xDF => Opcode::Rst18,
            0xE0 => Opcode::LdhNA,
            0xE1 => Opcode::PopHL,
            0xE2 => Opcode::LdhCA,
            0xE3 => Opcode::Invalid,
            0xE4 => Opcode::Invalid,
            0xE5 => Opcode::PushHL,
            0xE6 => Opcode::AndN,
            0xE7 => Opcode::Rst20,
            0xE8 => Opcode::AddSPN,
            0xE9 => Opcode::JpHL,
            0xEA => Opcode::LdNNPtrA,
            0xEB => Opcode::Invalid,
            0xEC => Opcode::Invalid,
            0xED => Opcode::Invalid,
            0xEE => Opcode::XorN,
            0xEF => Opcode::Rst28,
            0xF0 => Opcode::LdhAN,
            0xF1 => Opcode::PopAF,
            0xF2 => Opcode::LdhAC,
            0xF3 => Opcode::Di,
            0xF4 => Opcode::Invalid,
            0xF5 => Opcode::PushAF,
            0xF6 => Opcode::OrN,
            0xF7 => Opcode::Rst30,
            0xF8 => Opcode::LdHLSPN,
            0xF9 => Opcode::LdSPHL,
            0xFA => Opcode::LdANNPtr,
            0xFB => Opcode::Ei,
            0xFC => Opcode::Invalid,
            0xFD => Opcode::Invalid,
            0xFE => Opcode::CpN,
            0xFF => Opcode::Rst38,
            _ => Opcode::Invalid, // Catch any other invalid opcodes
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        opcode as u8
    }
}

impl From<u8> for ExtendedOpcode {
    fn from(byte: u8) -> Self {
        unsafe { std::mem::transmute(byte) }
    }
}

impl From<ExtendedOpcode> for u8 {
    fn from(opcode: ExtendedOpcode) -> u8 {
        opcode as u8
    }
}

impl Opcode {
    /// Get the instruction mnemonic for debugging
    pub fn mnemonic(&self) -> &'static str {
        match self {
            // Manually provide mnemonics for special cases
            Opcode::Nop => "NOP",
            Opcode::Halt => "HALT",
            Opcode::CbPrefix => "CB",
            Opcode::Invalid => "INVALID",
            Opcode::Ret => "RET",
            Opcode::Reti => "RETI",
            Opcode::Di => "DI",
            Opcode::Ei => "EI",
            Opcode::Stop => "STOP",
            
            // For load instructions, generate from the variant name
            Opcode::LdAE => "LD A, E",
            Opcode::LdAB => "LD A, B",
            Opcode::LdAC => "LD A, C",
            Opcode::LdAD => "LD A, D",
            Opcode::LdAH => "LD A, H",
            Opcode::LdAL => "LD A, L",
            Opcode::LdAA => "LD A, A",
            Opcode::LdAHLPtr => "LD A, (HL)",
            
            Opcode::LdBB => "LD B, B",
            Opcode::LdBC => "LD B, C",
            Opcode::LdBD => "LD B, D",
            Opcode::LdBE => "LD B, E",
            Opcode::LdBH => "LD B, H",
            Opcode::LdBL => "LD B, L",
            Opcode::LdBA => "LD B, A",
            Opcode::LdBHLPtr => "LD B, (HL)",
            
            // Add more as needed or use a default pattern
            _ => "UNKNOWN",
        }
    }

    /// Get expected instruction timing in cycles
    pub fn timing(&self) -> u8 {
        match self {
            Opcode::Nop => 4,
            Opcode::LdBCNN | Opcode::LdDENN | Opcode::LdHLNN | Opcode::LdSPNN => 12,
            Opcode::LdBCPtrA | Opcode::LdDEPtrA | Opcode::LdABCPtr | Opcode::LdADEPtr => 8,
            Opcode::IncBC | Opcode::IncDE | Opcode::IncHL | Opcode::IncSP => 8,
            Opcode::DecBC | Opcode::DecDE | Opcode::DecHL | Opcode::DecSP => 8,
            Opcode::IncB | Opcode::IncC | Opcode::IncD | Opcode::IncE |
            Opcode::IncH | Opcode::IncL | Opcode::IncA => 4,
            Opcode::DecB | Opcode::DecC | Opcode::DecD | Opcode::DecE |
            Opcode::DecH | Opcode::DecL | Opcode::DecA => 4,
            Opcode::LdBN | Opcode::LdCN | Opcode::LdDN | Opcode::LdEN |
            Opcode::LdHN | Opcode::LdLN | Opcode::LdAN => 8,
            Opcode::IncHLPtr | Opcode::DecHLPtr => 12,
            Opcode::LdHLPtrN => 12,
            Opcode::Rlca | Opcode::Rrca | Opcode::Rla | Opcode::Rra => 4,
            Opcode::Daa | Opcode::Cpl | Opcode::Scf | Opcode::Ccf => 4,
            Opcode::JrN | Opcode::JrNZN | Opcode::JrZN | Opcode::JrNCN | Opcode::JrCN => 12, // 8 if not taken
            Opcode::AddHLBC | Opcode::AddHLDE | Opcode::AddHLHL | Opcode::AddHLSP => 8,
            Opcode::LdHLIncA | Opcode::LdHLDecA | Opcode::LdAHLInc | Opcode::LdAHLDec => 8,
            Opcode::Halt => 4,
            // 8-bit loads (r,r)
            Opcode::LdBB | Opcode::LdBC | Opcode::LdBD | Opcode::LdBE | Opcode::LdBH | Opcode::LdBL | Opcode::LdBA |
            Opcode::LdCB | Opcode::LdCC | Opcode::LdCD | Opcode::LdCE | Opcode::LdCH | Opcode::LdCL | Opcode::LdCA |
            Opcode::LdDB | Opcode::LdDC | Opcode::LdDD | Opcode::LdDE | Opcode::LdDH | Opcode::LdDL | Opcode::LdDA |
            Opcode::LdEB | Opcode::LdEC | Opcode::LdED | Opcode::LdEE | Opcode::LdEH | Opcode::LdEL | Opcode::LdEA |
            Opcode::LdHB | Opcode::LdHC | Opcode::LdHD | Opcode::LdHE | Opcode::LdHH | Opcode::LdHL | Opcode::LdHA |
            Opcode::LdLB | Opcode::LdLC | Opcode::LdLD | Opcode::LdLE | Opcode::LdLH | Opcode::LdLL | Opcode::LdLA |
            Opcode::LdAB | Opcode::LdAC | Opcode::LdAD | Opcode::LdAE | Opcode::LdAH | Opcode::LdAL | Opcode::LdAA => 4,
            // Memory loads
            Opcode::LdBHLPtr | Opcode::LdCHLPtr | Opcode::LdDHLPtr | Opcode::LdEHLPtr |
            Opcode::LdHHLPtr | Opcode::LdLHLPtr | Opcode::LdAHLPtr => 8,
            Opcode::LdHLPtrB | Opcode::LdHLPtrC | Opcode::LdHLPtrD | Opcode::LdHLPtrE |
            Opcode::LdHLPtrH | Opcode::LdHLPtrL | Opcode::LdHLPtrA => 8,
            // 8-bit arithmetic
            Opcode::AddAB | Opcode::AddAC | Opcode::AddAD | Opcode::AddAE | Opcode::AddAH | Opcode::AddAL | Opcode::AddAA |
            Opcode::AdcAB | Opcode::AdcAC | Opcode::AdcAD | Opcode::AdcAE | Opcode::AdcAH | Opcode::AdcAL | Opcode::AdcAA |
            Opcode::SubB | Opcode::SubC | Opcode::SubD | Opcode::SubE | Opcode::SubH | Opcode::SubL | Opcode::SubA |
            Opcode::SbcAB | Opcode::SbcAC | Opcode::SbcAD | Opcode::SbcAE | Opcode::SbcAH | Opcode::SbcAL | Opcode::SbcAA |
            Opcode::AndB | Opcode::AndC | Opcode::AndD | Opcode::AndE | Opcode::AndH | Opcode::AndL | Opcode::AndA |
            Opcode::XorB | Opcode::XorC | Opcode::XorD | Opcode::XorE | Opcode::XorH | Opcode::XorL | Opcode::XorA |
            Opcode::OrB | Opcode::OrC | Opcode::OrD | Opcode::OrE | Opcode::OrH | Opcode::OrL | Opcode::OrA |
            Opcode::CpB | Opcode::CpC | Opcode::CpD | Opcode::CpE | Opcode::CpH | Opcode::CpL | Opcode::CpA => 4,
            // Default timing for others
            _ => 4,
        }
    }
}

impl ExtendedOpcode {
    /// Get the instruction mnemonic for debugging
    pub fn mnemonic(&self) -> &'static str {
        match self {
            ExtendedOpcode::RlcB => "RLC B",
            ExtendedOpcode::RlcC => "RLC C",
            ExtendedOpcode::RlcD => "RLC D",
            ExtendedOpcode::RlcE => "RLC E",
            ExtendedOpcode::RlcH => "RLC H",
            ExtendedOpcode::RlcL => "RLC L",
            ExtendedOpcode::RlcHLPtr => "RLC (HL)",
            ExtendedOpcode::RlcA => "RLC A",
            ExtendedOpcode::Bit0A => "BIT 0,A",
            ExtendedOpcode::Bit1A => "BIT 1,A",
            ExtendedOpcode::Bit2A => "BIT 2,A",
            ExtendedOpcode::Bit3A => "BIT 3,A",
            ExtendedOpcode::Bit4A => "BIT 4,A",
            ExtendedOpcode::Bit5A => "BIT 5,A",
            ExtendedOpcode::Bit6A => "BIT 6,A",
            ExtendedOpcode::Bit7A => "BIT 7,A",
            ExtendedOpcode::Res0A => "RES 0,A",
            ExtendedOpcode::Set0A => "SET 0,A",
            // Add more as needed...
            _ => "UNKNOWN",
        }
    }

    /// Get expected instruction timing in cycles
    pub fn timing(&self) -> u8 {
        match self {
            // Most CB instructions are 8 cycles, (HL) variants are 16
            ExtendedOpcode::RlcHLPtr | ExtendedOpcode::RrcHLPtr | ExtendedOpcode::RlHLPtr | ExtendedOpcode::RrHLPtr |
            ExtendedOpcode::SlaHLPtr | ExtendedOpcode::SraHLPtr | ExtendedOpcode::SwapHLPtr | ExtendedOpcode::SrlHLPtr => 16,
            ExtendedOpcode::Bit0HLPtr | ExtendedOpcode::Bit1HLPtr | ExtendedOpcode::Bit2HLPtr | ExtendedOpcode::Bit3HLPtr |
            ExtendedOpcode::Bit4HLPtr | ExtendedOpcode::Bit5HLPtr | ExtendedOpcode::Bit6HLPtr | ExtendedOpcode::Bit7HLPtr => 12,
            ExtendedOpcode::Res0HLPtr | ExtendedOpcode::Res1HLPtr | ExtendedOpcode::Res2HLPtr | ExtendedOpcode::Res3HLPtr |
            ExtendedOpcode::Res4HLPtr | ExtendedOpcode::Res5HLPtr | ExtendedOpcode::Res6HLPtr | ExtendedOpcode::Res7HLPtr |
            ExtendedOpcode::Set0HLPtr | ExtendedOpcode::Set1HLPtr | ExtendedOpcode::Set2HLPtr | ExtendedOpcode::Set3HLPtr |
            ExtendedOpcode::Set4HLPtr | ExtendedOpcode::Set5HLPtr | ExtendedOpcode::Set6HLPtr | ExtendedOpcode::Set7HLPtr => 16,
            _ => 8, // All other CB instructions
        }
    }
}
