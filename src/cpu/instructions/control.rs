use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn jmp(&mut self, mode: &AddressingMode) {
        let mem_address = self.get_operand_address(mode);
        self.program_counter = mem_address;
    }
}
