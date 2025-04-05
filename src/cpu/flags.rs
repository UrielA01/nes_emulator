use crate::cpu::cpu::CPU;
use bitflags::bitflags;

bitflags! {
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

impl CPU {
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status
            .set(StatusFlags::NEGATIVE, result & 0b1000_0000 != 0);
    }

    pub fn update_carry_asl(&mut self, value: u8) {
        self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
    }

    pub fn update_carry_lsr(&mut self, value: u8) {
        self.status.set(StatusFlags::CARRY, (value & 0x01) != 0);
    }

    pub fn update_carry_adc(&mut self, result: u16) {
        self.status.set(StatusFlags::CARRY, result > 0xFF);
    }

    #[allow(dead_code)]
    fn update_carry_sbc(&mut self, result: u16) {
        self.status.set(StatusFlags::CARRY, result < 0x100);
    }

    #[allow(dead_code)]
    fn update_carry_cmp(&mut self, register: u8, operand: u8) {
        self.status.set(StatusFlags::CARRY, register >= operand);
    }

    // ---- CLEAR FLAGS ----
    pub fn clear_carry_flag(&mut self) {
        self.status.remove(StatusFlags::CARRY);
    }

    pub fn clear_interrupt_disable_flag(&mut self) {
        self.status.remove(StatusFlags::INTERRUPT);
    }

    pub fn clear_decimal_flag(&mut self) {
        self.status.remove(StatusFlags::DECIMAL);
    }

    pub fn clear_overflow_flag(&mut self) {
        self.status.remove(StatusFlags::OVERFLOW);
    }

    // ---- SET FLAGS ----
    pub fn set_carry_flag(&mut self) {
        self.status.insert(StatusFlags::CARRY);
    }
    pub fn set_interrupt_disable_flag(&mut self) {
        self.status.insert(StatusFlags::INTERRUPT);
    }

    pub fn set_decimal_flag(&mut self) {
        self.status.insert(StatusFlags::DECIMAL);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Flag control
    #[test]
    fn test_clear_and_set_carry_flag() {
        let mut cpu = CPU::new();

        cpu.set_carry_flag();
        assert!(cpu.status.contains(StatusFlags::CARRY));

        cpu.clear_carry_flag();
        assert!(!cpu.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_clear_and_set_decimal_flag() {
        let mut cpu = CPU::new();

        cpu.set_decimal_flag();
        assert!(cpu.status.contains(StatusFlags::DECIMAL));

        cpu.clear_decimal_flag();
        assert!(!cpu.status.contains(StatusFlags::DECIMAL));
    }

    #[test]
    fn test_clear_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.status.insert(StatusFlags::OVERFLOW);
        cpu.clear_overflow_flag();
        assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
    }
}
