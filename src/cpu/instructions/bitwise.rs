use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};
enum LogicalGate {
    AND,
    OR,
    XOR,
}

impl CPU {
    fn logical_gate(&mut self, mode: &AddressingMode, gate: LogicalGate) {
        let value = self.get_mode_return_value(mode);

        let result = match gate {
            LogicalGate::AND => self.register_a & value,
            LogicalGate::OR => self.register_a | value,
            LogicalGate::XOR => self.register_a ^ value,
        };

        self.register_a = result;
        self.update_zero_and_negative_flags(result)
    }
    pub fn and(&mut self, mode: &AddressingMode) {
        self.logical_gate(mode, LogicalGate::AND);
    }

    pub fn ora(&mut self, mode: &AddressingMode) {
        self.logical_gate(mode, LogicalGate::OR);
    }

    pub fn eor(&mut self, mode: &AddressingMode) {
        self.logical_gate(mode, LogicalGate::XOR);
    }

    pub fn bit(&mut self, mode: &AddressingMode) {
        let value = self.get_mode_return_value(mode);

        self.status
            .set(StatusFlags::ZERO, self.register_a & value == 0);
        self.status
            .set(StatusFlags::NEGATIVE, value & 0b10000000 > 0);
        self.status
            .set(StatusFlags::OVERFLOW, value & 0b01000000 > 0);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{flags::StatusFlags, memory::Mem};

    use super::*;
    #[test]
    fn test_and_operation() {
        let mut cpu = CPU::test_new();

        cpu.register_a = 0b11001100;
        cpu.mem_write(0x0200, 0x25); // AND ZeroPage opcode
        cpu.mem_write(0x0201, 0x10); // Address $10
        cpu.mem_write(0x10, 0b10101010); // Value in memory
        cpu.program_counter = 0x0200;

        cpu.run();

        assert_eq!(cpu.register_a, 0b10001000);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
    }

    #[test]
    fn test_or_operation() {
        let mut cpu = CPU::test_new();

        cpu.register_a = 0b11001100;
        cpu.mem_write(0x0200, 0x05); // ORA ZeroPage opcode
        cpu.mem_write(0x0201, 0x10); // Address $10
        cpu.mem_write(0x10, 0b10101010);
        cpu.program_counter = 0x0200;

        cpu.run();

        assert_eq!(cpu.register_a, 0b11101110);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
    }

    #[test]
    fn test_xor_operation() {
        let mut cpu = CPU::test_new();

        cpu.register_a = 0b11001100;
        cpu.mem_write(0x0200, 0x45); // EOR ZeroPage opcode
        cpu.mem_write(0x0201, 0x10); // Address $10
        cpu.mem_write(0x10, 0b10101010);
        cpu.program_counter = 0x0200;

        cpu.run();

        assert_eq!(cpu.register_a, 0b01100110);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), false);
    }

    #[test]
    fn test_and_sets_zero_flag() {
        let mut cpu = CPU::test_new();

        cpu.register_a = 0b00001111;
        cpu.mem_write(0x0200, 0x25); // AND ZeroPage opcode
        cpu.mem_write(0x0201, 0x10);
        cpu.mem_write(0x10, 0b11110000);
        cpu.program_counter = 0x0200;

        cpu.run();

        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_eor_sets_negative_flag() {
        let mut cpu = CPU::test_new();

        cpu.register_a = 0b01010101;
        cpu.mem_write(0x0200, 0x45); // EOR ZeroPage opcode
        cpu.mem_write(0x0201, 0x10);
        cpu.mem_write(0x10, 0b10101010);
        cpu.program_counter = 0x0200;

        cpu.run();

        assert_eq!(cpu.register_a, 0b11111111);
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }
}
