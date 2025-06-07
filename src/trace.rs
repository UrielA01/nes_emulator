use crate::cpu::{
    cpu::{AddressingMode, CPU},
    memory::Mem,
    opcodes::OpCode,
};

impl CPU {
    fn instruction_bytes(&self, bytes: u8) -> Vec<u8> {
        let mut operands = vec![];
        let pc = self.program_counter - 1;
        for n in 0..bytes {
            let byte = self.mem_read(pc + (n as u16));
            operands.push(byte);
        }
        operands
    }

    fn format_operand(&self, mnemonic: &str, mode: &AddressingMode) -> String {
        let pc = self.program_counter;
        let final_addr = self.get_operand_address(mode);
        let final_addr_u8 = self.get_operand_address(mode) as u8;
        let value: u8 = self.get_mode_return_value(mode);
        match mode {
            AddressingMode::Immediate => {
                format!("{} #${:02X}", mnemonic, value)
            }
            AddressingMode::Implied => format!("{}", mnemonic),
            AddressingMode::Accumulator => format!("{} A", mnemonic),
            AddressingMode::ZeroPage => {
                format!("{} ${:02X} = {:02X}", mnemonic, final_addr_u8, value)
            }
            AddressingMode::ZeroPage_X => {
                let base_addr = self.mem_read(pc);
                format!(
                    "{} ${:02X},X @ {:02X} = {:02X}",
                    mnemonic, base_addr, final_addr_u8, value
                )
            }
            AddressingMode::ZeroPage_Y => {
                let base_addr = self.mem_read(pc);
                format!(
                    "{} ${:02X},Y @ {:02X} = {:02X}",
                    mnemonic, base_addr, final_addr_u8, value
                )
            }
            AddressingMode::Absolute => {
                if mnemonic == "JMP" || mnemonic == "JSR" {
                    format!("{} ${:04X}", mnemonic, final_addr)
                } else {
                    format!("{} ${:04X} = {:02X}", mnemonic, final_addr, value)
                }
            }
            AddressingMode::Absolute_X => {
                let base_addr = self.mem_read_u16(pc);
                format!(
                    "{} ${:04X},X @ {:04X} = {:02X}",
                    mnemonic, base_addr, final_addr, value
                )
            }
            AddressingMode::Absolute_Y => {
                let base_addr = self.mem_read_u16(pc);
                format!(
                    "{} ${:04X},Y @ {:04X} = {:02X}",
                    mnemonic, base_addr, final_addr, value
                )
            }
            AddressingMode::Indirect => {
                let base_addr = self.mem_read_u16(pc);
                format!("{} (${:04X}) = {:04X}", mnemonic, base_addr, final_addr)
            }

            AddressingMode::Indirect_X => {
                let base_addr = self.mem_read(pc);
                let base_addr_with_x = base_addr.wrapping_add(self.register_x);
                format!(
                    "{} (${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    mnemonic, base_addr, base_addr_with_x, final_addr, value
                )
            }
            AddressingMode::Indirect_Y => {
                let base_addr = self.mem_read(pc);
                let base_addr_with_y = final_addr.wrapping_sub(self.register_y as u16);

                format!(
                    "{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    mnemonic, base_addr, base_addr_with_y, final_addr, value
                )
            }
            AddressingMode::Relative => {
                let jump: i8 = self.mem_read(self.program_counter) as i8;
                let jump_addr = self
                    .program_counter
                    .wrapping_add(1)
                    .wrapping_add(jump as u16);

                format!("{} ${:04X}", mnemonic, jump_addr)
            }
            AddressingMode::NoneAddressing => {
                format!("{}", mnemonic)
            }
        }
    }

    pub fn trace(&mut self, opcode: &OpCode) -> String {
        let pc = self.program_counter;

        let instruction_bytes_str = self
            .instruction_bytes(opcode.bytes)
            .iter()
            .map(|z| format!("{:02X}", z))
            .collect::<Vec<String>>()
            .join(" ");

        let operand_display = self.format_operand(&opcode.mnemonic, &opcode.mode);

        format!(
            "{:04X}  {:8}  {:<32}A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            pc - 1,
            instruction_bytes_str,
            operand_display,
            self.register_a,
            self.register_x,
            self.register_y,
            self.status,
            self.sp,
        )
    }
}
