use crate::cpu::{cpu::CPU, flags::StatusFlags};

impl CPU {
    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);

            self.program_counter = jump_addr;
        }
    }

    pub fn bne(&mut self) {
        self.branch(!self.status.contains(StatusFlags::ZERO));
    }

    pub fn beq(&mut self) {
        self.branch(self.status.contains(StatusFlags::ZERO));
    }

    pub fn bcc(&mut self) {
        self.branch(!self.status.contains(StatusFlags::CARRY));
    }

    pub fn bcs(&mut self) {
        self.branch(self.status.contains(StatusFlags::CARRY));
    }

    pub fn bmi(&mut self) {
        self.branch(self.status.contains(StatusFlags::NEGATIVE));
    }

    pub fn bpl(&mut self) {
        self.branch(!self.status.contains(StatusFlags::NEGATIVE));
    }

    pub fn bvc(&mut self) {
        self.branch(!self.status.contains(StatusFlags::OVERFLOW));
    }

    pub fn bvs(&mut self) {
        self.branch(self.status.contains(StatusFlags::OVERFLOW));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa2, 0x08, 0xca, 0x8e, 0x00, 0x02, 0xe0, 0x03, 0xd0, 0xf8, 0x8e, 0x01, 0x02, 0x00,
        ]);

        assert_eq!(cpu.register_x, 0x03);
        assert_eq!(cpu.mem_read(0x0200), 0x03);
        assert_eq!(cpu.mem_read(0x0201), 0x03);
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x00, // LDA #$00 → sets Zero flag
            0xf0, 0x02, // BEQ +2 → should jump over next instruction
            0xa9, 0xff, // LDA #$FF → should be skipped
            0x8d, 0x00, 0x02, // STA $0200 → should store 0x00
            0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x00);
    }

    #[test]
    fn test_bmi() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0xff, // LDA #$FF → Negative flag set
            0x30, 0x02, // BMI +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0x8d, 0x00, 0x02, // STA $0200 → should store 0xFF
            0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0xff);
    }

    #[test]
    fn test_bpl() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x01, // LDA #$01 → clears Negative flag
            0x10, 0x02, // BPL +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0x8d, 0x00, 0x02, // STA $0200 → should store 0x01
            0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x01);
    }

    #[test]
    fn test_bcs() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0x38, // SEC → set carry
            0xb0, 0x02, // BCS +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0xa9, 0x01, // LDA #$01 → executed
            0x8d, 0x00, 0x02, // STA $0200
            0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x01);
    }

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0x18, // CLC → clear carry
            0x90, 0x02, // BCC +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0xa9, 0x02, // LDA #$02 → executed
            0x8d, 0x00, 0x02, // STA $0200
            0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x02);
    }

    #[test]
    fn test_bvs() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x50, // LDA #$50
            0x69, 0x50, // ADC #$50 → causes overflow
            0x70, 0x02, // BVS +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0xa9, 0x03, // LDA #$03 → executed
            0x8d, 0x00, 0x02, 0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x03);
    }

    #[test]
    fn test_bvc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x01, // LDA #$01
            0x69, 0x01, // ADC #$01 → no overflow
            0x50, 0x02, // BVC +2 → should jump
            0xa9, 0x00, // LDA #$00 → skipped
            0xa9, 0x04, // LDA #$04 → executed
            0x8d, 0x00, 0x02, 0x00,
        ]);
        assert_eq!(cpu.mem_read(0x0200), 0x04);
    }
}
