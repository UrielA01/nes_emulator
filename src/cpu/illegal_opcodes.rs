use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::cpu::opcodes::OpCode;

use super::cpu::AddressingMode;

static CPU_ILLEGAL_OPS_CODES: Lazy<Vec<OpCode>> = Lazy::new(|| {
    vec![
        OpCode::new(0x1A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x1A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x3A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x5A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0x7A, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0xDA, "*NOP", 1, 2, AddressingMode::Implied),
        OpCode::new(0xFA, "*NOP", 1, 2, AddressingMode::Implied),
    ]
});

pub static ILLEGAL_CODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    CPU_ILLEGAL_OPS_CODES
        .iter()
        .map(|op_code| (op_code.code, op_code))
        .collect::<HashMap<_, _>>()
});
