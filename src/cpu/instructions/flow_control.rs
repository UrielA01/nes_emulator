use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn jmp(&mut self, mode: &AddressingMode) {
        let mem_address = self.get_operand_address(mode);
        self.program_counter = mem_address;
    }
}

#[cfg(test)]
mod test {
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

        println!("pos: {}", cpu.program_counter);

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }
}
