use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x)
    }

    pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y)
    }

    pub fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_add(1);

        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value)
    }

    pub fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_sub(1);

        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value)
    }

    pub fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y)
    }

    pub fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;

    #[test]
    fn test_0xee_inc_absolute() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x8010, 0x05);
        cpu.load_and_run(vec![0xee, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.mem_read_u16(0x8010), 6);
    }

    #[test]
    fn test_0xce_dec_absolute() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x8010, 0x05);
        cpu.load_and_run(vec![0xce, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.mem_read_u16(0x8010), 4);
    }

    #[test]
    fn test_0xce_dec_zero_flag() {
        let mut cpu = CPU::new();
        cpu.status.remove(StatusFlags::ZERO);

        cpu.mem_write_u16(0x8010, 0x01);
        cpu.load_and_run(vec![0xce, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.mem_read_u16(0x8010), 0);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xce_dec_negative_flag() {
        let mut cpu = CPU::new();
        cpu.status.remove(StatusFlags::NEGATIVE);

        cpu.mem_write_u16(0x8010, 0x00);
        cpu.load_and_run(vec![0xce, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.mem_read_u16(0x8010), 0xff);
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
