use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::cpu::opcodes::OpCode;

use super::cpu::AddressingMode;

static CPU_ILLEGAL_OPS_CODES: Lazy<Vec<OpCode>> = Lazy::new(|| {
    vec![
        /* Do nothing NOPs */
        OpCode::new(0x1A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x3A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x5A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x7A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0xDA, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0xFA, "*NOP", 1, 2, AddressingMode::Implied),
        /* Double NOP - DOP */
        OpCode::new(0x04, "*NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x14, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x34, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x44, "*NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x54, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x64, "*NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x74, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x80, "*NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x82, "*NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x89, "*NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc2, "*NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xd4, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xe2, "*NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xf4, "*NOP", 2, 4, AddressingMode::ZeroPage_X),
        /* Triple NOP - TOP */
        OpCode::new(0x0c, "*NOP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1c, "*NOP", 3, 4 /*or 5*/, AddressingMode::Absolute_X),
        OpCode::new(0x3c, "*NOP", 3, 4 /*or 5*/, AddressingMode::Absolute_X),
        OpCode::new(0x5c, "*NOP", 3, 4 /*or 5*/, AddressingMode::Absolute_X),
        OpCode::new(0x7c, "*NOP", 3, 4 /*or 5*/, AddressingMode::Absolute_X),
        OpCode::new(
            0xdc,
            "*NOP",
            3,
            4, /* or 5*/
            AddressingMode::Absolute_X,
        ),
        OpCode::new(
            0xfc,
            "*NOP",
            3,
            4, /* or 5*/
            AddressingMode::Absolute_X,
        ),
        /* Combined operations - https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes */
        OpCode::new(0x0b, "*ANC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x2b, "*ANC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x87, "*SAX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x97, "*SAX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8f, "*SAX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x83, "*SAX", 2, 6, AddressingMode::Indirect_X),
    ]
});

pub static ILLEGAL_CODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    CPU_ILLEGAL_OPS_CODES
        .iter()
        .map(|op_code| (op_code.code, op_code))
        .collect::<HashMap<_, _>>()
});
