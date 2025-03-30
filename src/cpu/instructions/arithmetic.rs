use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};

impl CPU {
    fn asl_logic(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.update_carry_asl(value);
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

    pub fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = (self.register_a as u16)
            + (value as u16)
            + ((self.status & StatusFlags::CARRY).bits() as u16);
        self.update_carry_adc(result);

        let overflow = ((self.register_a ^ result as u8) & (value ^ result as u8) & 0x80) != 0;
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.update_zero_and_negative_flags(result as u8);

        self.register_a = result as u8;
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
