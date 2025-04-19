use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn jmp(&mut self, mode: &AddressingMode) {
        let mem_address = self.get_operand_address(mode);
        self.program_counter = mem_address;
    }

    pub fn jsr(&mut self, mode: &AddressingMode) {
        let mem_address = self.get_operand_address(mode);
        let pc = self.program_counter + 1;
        let high = (pc >> 8) as u8;
        let low = (pc & 0xff) as u8;

        self.push(low);
        self.push(high);
        self.program_counter = mem_address;
    }

    pub fn rts(&mut self) {
        let high = self.pop() as u16;
        let low = self.pop() as u16;
        self.program_counter = ((high << 8) | (low)).wrapping_add(1);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::memory::Mem;

    use super::*;

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

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }

    #[test]
    fn test_jsr_rts() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0x20, 0x09, 0x80, 0x20, 0x0c, 0x80, 0x20, 0x12, 0x80, 0xa2, 0x00, 0x60, 0xe8, 0xe0,
            0x05, 0xd0, 0xfb, 0x60, 0x00,
        ]);

        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(cpu.program_counter, 0x8013);
        assert_eq!(cpu.register_x, 0x05);
    }

    #[test]
    fn test_jsr_rts_2() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0x20, 0x06, 0x80, // JSR $8006
            0xa9, 0x42, // LDA #$42
            0x00, // BRK (should never reach here)
            0x60, // RTS
        ]);

        assert_eq!(cpu.register_a, 0x42);
    }
}
