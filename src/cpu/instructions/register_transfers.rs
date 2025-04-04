use crate::cpu::cpu::CPU;

impl CPU {
    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x)
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y)
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a)
    }

    pub fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_0xa8_tay() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xa8, 0x00]);

        assert_eq!(cpu.register_y, 10)
    }

    #[test]
    fn test_0x8a_txa() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x0A, 0x8a, 0x00]);

        assert_eq!(cpu.register_a, 10)
    }

    #[test]
    fn test_0x98_tya() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x0A, 0x98, 0x00]);

        assert_eq!(cpu.register_a, 10)
    }
}
