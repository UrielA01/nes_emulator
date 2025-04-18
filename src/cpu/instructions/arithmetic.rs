use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};

impl CPU {
    fn add_to_register_a(&mut self, value: u8) {
        let result = (self.register_a as u16)
            + (value as u16)
            + ((self.status & StatusFlags::CARRY).bits() as u16);
        self.update_carry_adc(result);

        let overflow = ((self.register_a ^ result as u8) & (value ^ result as u8) & 0x80) != 0;
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.update_zero_and_negative_flags(result as u8);

        self.register_a = result as u8;
    }

    pub fn adc(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);
        self.add_to_register_a(value);
    }

    pub fn sbc(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);
        self.add_to_register_a(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn compare_register(&mut self, mode: &AddressingMode, register: u8) {
        let value = self.get_mode_return_value(mode);

        let result = register.wrapping_sub(value);
        self.update_zero_and_negative_flags(result as u8);
        self.status.set(StatusFlags::CARRY, register >= value);
    }

    pub fn cmp(&mut self, mode: &AddressingMode) {
        self.compare_register(mode, self.register_a);
    }
    pub fn cpx(&mut self, mode: &AddressingMode) {
        self.compare_register(mode, self.register_x);
    }
    pub fn cpy(&mut self, mode: &AddressingMode) {
        self.compare_register(mode, self.register_y);
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
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::OVERFLOW), false);
    }

    #[test]
    fn test_0x69_adc_immediate_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFA, 0x69, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 4);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
    }

    #[test]
    fn test_0x69_adc_immediate_with_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x64, 0x69, 0x32, 0x00]);
        assert_eq!(cpu.register_a, 150);
        assert_eq!(cpu.status.contains(StatusFlags::OVERFLOW), true);
    }

    #[test]
    fn test_0xc9_cmp_equal_sets_zero_and_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x42, // LDA #$42
            0xC9, 0x42, // CMP #$42
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x42);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), true);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
    }

    #[test]
    fn test_0xc9_cmp_a_greater_than_m_sets_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x80, // LDA #$80
            0xC9, 0x40, // CMP #$40
            0x00,
        ]);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.register_a, 0x80);
    }

    #[test]
    fn test_0xc9_cmp_a_less_than_m_sets_negative_and_clears_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x30, // LDA #$30
            0xC9, 0x50, // CMP #$50
            0x00,
        ]);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
    }

    #[test]
    fn test_0xc9_cmp_signed_negative_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x10, // LDA #$10 (16)
            0xC9, 0x90, // CMP #$90 (144)
            0x00,
        ]);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
    }

    #[test]
    fn test_0xc9_cmp_signed_positive_result_with_negative_bit_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x90, // LDA #$90 (144)
            0xC9, 0x10, // CMP #$10 (16)
            0x00,
        ]);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }
}
