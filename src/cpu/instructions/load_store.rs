use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn lda(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn ldx(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn ldy(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    pub fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    pub fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

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
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xa2_ldx_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 5);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == 0);
        assert!(cpu.status.bits() & StatusFlags::NEGATIVE.bits() == 0);
    }

    #[test]
    fn test_0xa0_ldy_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 5);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == 0);
        assert!(cpu.status.bits() & StatusFlags::NEGATIVE.bits() == 0);
    }

    #[test]
    fn test_0x85_sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x08, 0x00]);

        assert_eq!(cpu.register_a, 5);
        assert_eq!(cpu.mem_read(0x08), 5)
    }

    #[test]
    fn test_0x86_stx_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x86, 0x08, 0x00]);

        assert_eq!(cpu.register_x, 5);
        assert_eq!(cpu.mem_read(0x08), 5)
    }

    #[test]
    fn test_0x84_sty_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x84, 0x08, 0x00]);

        assert_eq!(cpu.register_y, 5);
        assert_eq!(cpu.mem_read(0x08), 5)
    }
}
