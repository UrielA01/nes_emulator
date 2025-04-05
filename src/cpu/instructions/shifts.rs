use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    fn asl_logic(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.update_carry_asl(value);
        self.update_zero_and_negative_flags(result);
        result
    }

    fn lsr_logic(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.update_carry_lsr(value);
        self.update_zero_and_negative_flags(result);
        result
    }

    pub fn asl(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                self.register_a = self.asl_logic(self.register_a);
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);

                let result = self.asl_logic(value);
                self.mem_write(addr, result);
            }
        }
    }

    pub fn lsr(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                self.register_a = self.lsr_logic(self.register_a);
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);

                let result = self.lsr_logic(value);
                self.mem_write(addr, result);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;
    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x41, 0x0A, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0010);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xC0);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x80);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_0x4a_lsr_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0b0000_0010, 0x4A, 0x00]);

        assert_eq!(cpu.register_a, 0b0000_0001);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_0x4a_lsr_accumulator_sets_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0b0000_0001, 0x4A, 0x00]);

        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), true);
    }

    #[test]
    fn test_0x46_lsr_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_0011);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0b0000_0001);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }
}
