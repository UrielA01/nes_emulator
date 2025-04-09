use crate::cpu::{cpu::CPU, flags::StatusFlags};

impl CPU {
    fn push(&mut self, data: u8) {
        let addr = 0x0100 | self.sp as u16;

        self.mem_write(addr, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100 | self.sp as u16;

        self.mem_read(addr)
    }

    pub fn pha(&mut self) {
        self.push(self.register_a);
    }

    pub fn php(&mut self) {
        self.push(self.status.bits());
    }

    pub fn pla(&mut self) {
        self.register_a = self.pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn plp(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.pop());
    }

    pub fn txs(&mut self) {
        self.sp = self.register_x;
    }

    pub fn tsx(&mut self) {
        self.register_x = self.sp;
        self.update_zero_and_negative_flags(self.register_x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x42, // LDA #$42
            0x48, // PHA
            0x00, // BRK
        ]);
        let sp_after = cpu.sp;
        let val_on_stack = cpu.mem_read(0x0100 | (sp_after + 1) as u16);
        assert_eq!(val_on_stack, 0x42);
    }

    #[test]
    fn test_php() {
        let mut cpu = CPU::new();
        cpu.status.insert(StatusFlags::CARRY | StatusFlags::ZERO);
        cpu.load_and_run(vec![
            0x08, // PHP
            0x00, // BRK
        ]);
        let sp_after = cpu.sp;
        let val_on_stack = cpu.mem_read(0x0100 | (sp_after + 1) as u16);
        assert_eq!(val_on_stack, cpu.status.bits());
    }

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x37, // LDA #$37
            0x48, // PHA
            0xA9, 0x00, // LDA #$00 (overwrite A)
            0x68, // PLA
            0x00, // BRK
        ]);
        assert_eq!(cpu.register_a, 0x37);
    }

    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();
        let init_status = StatusFlags::UNUSED | StatusFlags::BREAK; // Clear all
        cpu.load_and_run(vec![
            0x08, // PHP (push current status)
            0xA9, 0x00, // LDA #$00 to modify flags
            0x28, // PLP
            0x00, // BRK
        ]);
        // Should init status
        assert_eq!(cpu.status, init_status);
    }

    #[test]
    fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xba, // TSX
            0x00, // BRK
        ]);
        assert_eq!(cpu.register_x, 0xff);
    }

    #[test]
    fn test_txs() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0x9a, // TSX
            0x00, // BRK
        ]);
        assert_eq!(cpu.sp, 0x00);
    }
}
