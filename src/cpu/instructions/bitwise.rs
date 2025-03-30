use crate::cpu::cpu::{AddressingMode, CPU};

impl CPU {
    pub fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.register_a = result;
        self.update_zero_and_negative_flags(result)
    }
}
