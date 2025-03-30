use super::flags::StatusFlags;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    pub memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::UNUSED | StatusFlags::BREAK,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == 0);
        assert!(cpu.status.bits() & StatusFlags::NEGATIVE.bits() == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.bits() & StatusFlags::ZERO.bits() == StatusFlags::ZERO.bits());
    }

    #[test]
    fn test_0x69_adc_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x69, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!((cpu.status & StatusFlags::CARRY).is_empty());
        assert!((cpu.status & StatusFlags::OVERFLOW).is_empty());
    }

    #[test]
    fn test_0x69_adc_immediate_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFA, 0x69, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 4);
        assert!((cpu.status & StatusFlags::CARRY).eq(&StatusFlags::CARRY));
    }

    #[test]
    fn test_0x69_adc_immediate_with_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x64, 0x69, 0x32, 0x00]);
        assert_eq!(cpu.register_a, 150);
        assert!((cpu.status & StatusFlags::OVERFLOW).eq(&StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x41, 0x0A, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0010);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), false);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xC0);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x80);
        assert_eq!(cpu.status.contains(StatusFlags::CARRY), true);
        assert_eq!(cpu.status.contains(StatusFlags::NEGATIVE), true);
        assert_eq!(cpu.status.contains(StatusFlags::ZERO), false);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x4C, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x2000, 0x1234);

        cpu.load_and_run(vec![0x6C, 0x00, 0x20, 0x00]);

        println!("pos: {}", cpu.program_counter);

        assert_eq!(cpu.program_counter - 1, 0x1234);
    }
}
