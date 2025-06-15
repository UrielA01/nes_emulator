use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
    memory::Mem,
};

impl CPU {
    pub fn anc(&mut self, mode: &AddressingMode) {
        self.and(mode);
        let is_neg_set = self.status.contains(StatusFlags::NEGATIVE);
        self.status.set(StatusFlags::CARRY, is_neg_set);
    }

    pub fn sax(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(&mode);
        let result = self.register_a & self.register_x;
        self.mem_write(address, result);
        self.update_zero_and_negative_flags(result);
    }

    pub fn axs(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(&mode);
        let x_and_a = self.register_x & self.register_a;
        let result = x_and_a.wrapping_sub(value);
        self.update_carry_axs(result);
        self.update_zero_and_negative_flags(result);
        self.register_x = result;
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;
    #[test]
    fn test_anc_sets_carry_and_result() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xa9,
            0b1100_1100, // LDA #$CC
            0x0b,
            0b1010_1010, // ANC #$AA (CC & AA = 0b1000_1000)
        ]);

        assert_eq!(cpu.register_a, 0b1000_1000);
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_sax_zp() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xa9,
            0b1100_1100, // LDA #$CC
            0xa2,
            0b1000_1010, // LDX #$8A
            0x87,
            0xfa, // SAX #$FA
        ]);

        assert_eq!(cpu.register_a, 0b1100_1100);
        assert_eq!(cpu.register_x, 0b1000_1010);
        assert_eq!(cpu.mem_read(0xfa), 0b1000_1000);
    }

    #[test]
    fn test_axs() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xa9,
            0b1100_1100, // LDA #$CC
            0xa2,
            0b1010_1010, // LDX #$AA
            0xcb,
            0b1000_1000, // AXS #$88 → (A & X) = 0x88, then 0x88 - 0x88 = 0
        ]);

        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(cpu.status.contains(StatusFlags::CARRY)); // 0x88 >= 0x88
    }
}
