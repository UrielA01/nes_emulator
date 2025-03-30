use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};

impl CPU {
    fn asl_logic(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.update_carry_asl(value);
        self.update_zero_and_negative_flags(result);
        result
    }

    fn asl(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                self.register_a = self.asl_logic(self.register_a);
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);

                let result = self.asl_logic(value);
                self.mem_write(addr, result);
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = (self.register_a as u16)
            + (value as u16)
            + ((self.status & StatusFlags::CARRY).bits() as u16);
        self.update_carry_adc(result);

        let overflow = ((self.register_a ^ result as u8) & (value ^ result as u8) & 0x80) != 0;
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.update_zero_and_negative_flags(result as u8);

        self.register_a = result as u8;
    }
}
