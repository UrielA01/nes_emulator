use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::cpu::AddressingMode;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            bytes: bytes,
            cycles: cycles,
            mode: mode,
        }
    }
}

static CPU_OPS_CODES: Lazy<Vec<OpCode>> = Lazy::new(|| {
    vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::Implied),
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xc8, "INY", 1, 2, AddressingMode::Implied),
        OpCode::new(0xca, "DEX", 1, 2, AddressingMode::Implied),
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::Implied),
        // ADC
        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),
        // AND
        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0x3d,
            "AND",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
        OpCode::new(
            0x39,
            "AND",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(
            0x31,
            "AND",
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
        // EOR
        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0x5d,
            "EOR",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
        OpCode::new(
            0x59,
            "EOR",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(
            0x51,
            "EOR",
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
        // ORA
        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0x1d,
            "ORA",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
        OpCode::new(
            0x19,
            "ORA",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(
            0x11,
            "ORA",
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
        // ASL
        OpCode::new(0x0a, "ASL", 1, 2, AddressingMode::Accumulator),
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::Absolute_X),
        // LDA
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5, AddressingMode::Indirect_Y),
        // LDX
        OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0xbe,
            "LDX",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
        // LDY
        OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0xbc,
            "LDY",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
        // STA
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),
        // JMP
        OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::Absolute),
        OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::Indirect),
        // Clear flags
        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::Implied),
        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::Implied),
        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::Implied),
        OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::Implied),
        // Set flags
        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::Implied),
        OpCode::new(0xF8, "SED", 1, 2, AddressingMode::Implied),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::Implied),
        // INC
        OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),
        // DEC
        OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X),
    ]
});

pub static CODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    CPU_OPS_CODES
        .iter()
        .map(|op_code| (op_code.code, op_code))
        .collect::<HashMap<_, _>>()
});
