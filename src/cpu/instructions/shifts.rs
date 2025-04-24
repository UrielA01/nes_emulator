use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
    memory::Mem,
};

impl CPU {
    fn apply_shift<F>(&mut self, mode: &AddressingMode, op: F)
    where
        F: FnOnce(&mut Self, u8) -> u8,
    {
        match mode {
            AddressingMode::Accumulator => {
                self.register_a = op(self, self.register_a);
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);
                let result = op(self, value);
                self.mem_write(addr, result);
            }
        }
    }

    pub fn asl(&mut self, mode: &AddressingMode) {
        self.apply_shift(mode, |cpu, value| {
            let result = value << 1;
            cpu.update_carry_asl(value);
            cpu.update_zero_and_negative_flags(result);
            result
        });
    }

    pub fn lsr(&mut self, mode: &AddressingMode) {
        self.apply_shift(mode, |cpu, value| {
            let result = value >> 1;
            cpu.update_carry_lsr(value);
            cpu.update_zero_and_negative_flags(result);
            result
        });
    }

    pub fn rol(&mut self, mode: &AddressingMode) {
        self.apply_shift(mode, |cpu, value| {
            let mut result = value << 1;
            if cpu.status.contains(StatusFlags::CARRY) {
                result |= 0b0000_0001;
            }
            cpu.update_carry_asl(value);
            cpu.update_zero_and_negative_flags(result);
            result
        });
    }

    pub fn ror(&mut self, mode: &AddressingMode) {
        self.apply_shift(mode, |cpu, value| {
            let mut result = value >> 1;
            if cpu.status.contains(StatusFlags::CARRY) {
                result |= 0b1000_0000;
            }
            cpu.update_carry_lsr(value);
            cpu.update_zero_and_negative_flags(result);
            result
        });
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;
    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![0xA9, 0x41, 0x0A, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0010);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::test_new();

        cpu.mem_write(0x10, 0xC0);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x80);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_0x4a_lsr_accumulator() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![0xA9, 0b0000_0010, 0x4A, 0x00]);

        assert_eq!(cpu.register_a, 0b0000_0001);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_0x4a_lsr_accumulator_sets_carry() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![0xA9, 0b0000_0001, 0x4A, 0x00]);

        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), true);
    }

    #[test]
    fn test_0x46_lsr_memory() {
        let mut cpu = CPU::test_new();

        cpu.mem_write(0x10, 0b0000_0011);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0b0000_0001);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_rol_accumulator() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xA9,
            0b0100_0001, // LDA #$41
            0x38,        // set carry
            0x2A,        // ROL (Accumulator)
            0x00,        // BRK
        ]);
        assert_eq!(cpu.register_a, 0b1000_0011); // 0b1000_0010 + carry_in
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false); // old bit 7 was 0
    }

    #[test]
    fn test_ror_accumulator() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xA9,
            0b0000_0011, // LDA #$03
            0x18,        // clear carry
            0x6A,        // ROR (Accumulator)
            0x00,        // BRK
        ]);
        assert_eq!(cpu.register_a, 0b0000_0001); // shifted right, carry in was 0
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true); // bit 0 was 1 → carry out
    }
}
