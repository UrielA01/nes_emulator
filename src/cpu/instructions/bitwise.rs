use crate::cpu::cpu::{AddressingMode, CPU};
enum LogicalGate {
    AND,
    OR,
    XOR,
}

impl CPU {
    fn logical_gate(&mut self, mode: &AddressingMode, gate: LogicalGate) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = match gate {
            LogicalGate::AND => self.register_a & value,
            LogicalGate::OR => self.register_a | value,
            LogicalGate::XOR => self.register_a ^ value,
        };

        println!("a - {}", self.register_a);
        println!("value - {}", value);

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
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;
    #[test]
    fn test_and_operation() {
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
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
