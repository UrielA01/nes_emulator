use crate::opcodes;
use bitflags::bitflags;

/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
///

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
    pub struct StatusFlags: u8 {
        const CARRY     = 0b0000_0001;
        const ZERO      = 0b0000_0010;
        const INTERRUPT = 0b0000_0100;
        const DECIMAL   = 0b0000_1000;
        const BREAK     = 0b0001_0000;
        const UNUSED    = 0b0010_0000;
        const OVERFLOW  = 0b0100_0000;
        const NEGATIVE  = 0b1000_0000;
    }
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&mut self, position: u16) -> u16 {
        let low_bit = self.mem_read(position) as u16;
        let high_bit = self.mem_read(position + 1) as u16;
        (high_bit << 8) | (low_bit as u16)
    }
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high_bit = (data >> 8) as u8;
        let low_bit = (data & 0xff) as u8;
        self.mem_write(pos, low_bit);
        self.mem_write(pos + 1, high_bit);
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
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

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x)
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x)
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = (self.register_a as u16)
            + (value as u16)
            + ((self.status & StatusFlags::CARRY).bits() as u16);
        self.status.set(StatusFlags::CARRY, result > 0xff);

        let overflow = ((self.register_a ^ result as u8) & (value ^ result as u8) & 0x80) != 0;
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.update_zero_and_negative_flags(result as u8);

        self.register_a = result as u8;
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let mem_address = self.get_operand_address(mode);
        self.program_counter = mem_address;
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.register_a = result;
        self.update_zero_and_negative_flags(result)
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status
            .set(StatusFlags::NEGATIVE, result & 0b1000_0000 != 0);
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

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

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = StatusFlags::UNUSED | StatusFlags::BREAK;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let start_index = 0x8000;
        let end_index = 0x8000 + program.len();
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
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let original_program_counter = self.program_counter;

            let opcode = opcodes::CODES_MAP
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                0xA9 | 0xA5 | 0xAD | 0xb5 | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode.mode),
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),

                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode),

                0xAA => self.tax(),

                0xe8 => self.inx(),

                0x4c | 0x6c => self.jmp(&opcode.mode),

                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),

                0x00 => return,

                _ => todo!(),
            }

            if original_program_counter == self.program_counter {
                self.program_counter += (opcode.bytes - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == 0);
        assert!(cpu.status.bits() & StatusFlags::NEGATIVE.bits() == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == StatusFlags::ZERO.bits());
    }

    #[test]
    fn test_0x69_adc_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x69, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!((cpu.status & StatusFlags::CARRY).is_empty());
        assert!((cpu.status & StatusFlags::OVERFLOW).is_empty());
    }

    #[test]
    fn test_0x69_adc_immediate_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFA, 0x69, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 4);
        assert!((cpu.status & StatusFlags::CARRY).eq(&StatusFlags::CARRY));
    }

    #[test]
    fn test_0x69_adc_immediate_with_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x64, 0x69, 0x32, 0x00]);
        assert_eq!(cpu.register_a, 150);
        assert!((cpu.status & StatusFlags::OVERFLOW).eq(&StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x4C, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x2000, 0x1234);

        cpu.load_and_run(vec![0x6C, 0x00, 0x20, 0x00]);

        println!("pos: {}", cpu.program_counter);

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }
}
