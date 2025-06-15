use crate::cpu::{
    cpu::{AddressingMode, CPU},
    flags::StatusFlags,
};

impl CPU {
    pub fn anc(&mut self, mode: &AddressingMode) {
        self.and(mode);
        let is_neg_set = self.status.contains(StatusFlags::NEGATIVE);
        self.status.set(StatusFlags::CARRY, is_neg_set);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::flags::StatusFlags;

    use super::*;
    #[test]
    fn test_anc_sets_carry_and_result() {
        let mut cpu = CPU::test_new();

        cpu.load_and_run(vec![
            0xa9,
            0b1100_1100, // LDA #$CC
            0x0b,
            0b1010_1010, // ANC #$AA (CC & AA = 0b1000_1000)
        ]);

        assert_eq!(cpu.register_a, 0b1000_1000);
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }
}
