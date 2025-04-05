use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};

impl CPU {
    pub fn adc(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);

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
}
