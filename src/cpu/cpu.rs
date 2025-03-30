use crate::cpu::{execution, flags, instructions, memory};
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::UNUSED | StatusFlags::BREAK,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }
}
