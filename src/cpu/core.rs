use crate::cpu::cpu::CPU;
use crate::cpu::opcodes;

use super::{cpu::AddressingMode, flags::StatusFlags};

impl CPU {
    pub fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::Implied => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect => {
                let addr = self.mem_read_u16(self.program_counter);

                let lo = self.mem_read(addr);
                let hi = self.mem_read(if addr & 0x00FF == 0x00FF {
                    addr & 0xFF00
                } else {
                    addr + 1
                });

                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressingMode::NoneAddressing | _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn get_mode_return_value(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        return value;
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = StatusFlags::UNUSED | StatusFlags::BREAK;
        self.sp = 0xff;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let start_index = 0x0600;
        let end_index = 0x0600 + program.len();
        let program_counter_pos = 0xFFFC;
        self.memory[start_index..(end_index)].copy_from_slice(&program);
        self.mem_write_u16(program_counter_pos, start_index as u16);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let original_program_counter = self.program_counter;

            let opcode = opcodes::CODES_MAP
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                0xA9 | 0xA5 | 0xAD | 0xb5 | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode.mode),

                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(&opcode.mode),

                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(&opcode.mode),

                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),

                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => self.sbc(&opcode.mode),

                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.cmp(&opcode.mode),
                0xe0 | 0xe4 | 0xec => self.cpx(&opcode.mode),
                0xc0 | 0xc4 | 0xcc => self.cpy(&opcode.mode),

                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),

                0x86 | 0x96 | 0x8e => self.stx(&opcode.mode),

                0x84 | 0x94 | 0x8c => self.sty(&opcode.mode),

                0xAA => self.tax(),
                0xa8 => self.tay(),
                0x8a => self.txa(),
                0x98 => self.tya(),

                0xe8 => self.inx(),

                0xc8 => self.iny(),

                0x88 => self.dey(),

                0xca => self.dex(),

                0xe6 | 0xf6 | 0xee | 0xfe => self.inc(&opcode.mode),

                0xc6 | 0xd6 | 0xce | 0xde => self.dec(&opcode.mode),

                0x4c | 0x6c => self.jmp(&opcode.mode),
                0x20 => self.jsr(&opcode.mode),
                0x60 => self.rts(),

                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),

                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode),

                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => self.ora(&opcode.mode),

                0x024 | 0x2c => self.bit(&opcode.mode),

                0x06 | 0x16 | 0x0e | 0x1e | 0x0a => self.asl(&opcode.mode),

                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => self.lsr(&opcode.mode),

                0x26 | 0x36 | 0x2e | 0x3e | 0x2a => self.rol(&opcode.mode),

                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => self.ror(&opcode.mode),

                0x18 => self.clear_carry_flag(),
                0xD8 => self.clear_decimal_flag(),
                0x58 => self.clear_interrupt_disable_flag(),
                0xB8 => self.clear_overflow_flag(),
                0x38 => self.set_carry_flag(),
                0xF8 => self.set_decimal_flag(),
                0x78 => self.set_interrupt_disable_flag(),

                // Branches
                0x90 => self.bcc(),
                0xd0 => self.bne(),
                0x70 => self.bvs(),
                0x50 => self.bvc(),
                0x10 => self.bpl(),
                0x30 => self.bmi(),
                0xf0 => self.beq(),
                0xb0 => self.bcs(),

                // Stack
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),
                0x9a => self.txs(),
                0xba => self.tsx(),

                0x00 => return,

                0xea => {}

                _ => todo!(),
            }

            if original_program_counter == self.program_counter {
                self.program_counter += (opcode.bytes - 1) as u16;
            }
        }
    }
}
