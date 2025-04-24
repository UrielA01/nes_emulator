use crate::bus::Bus;

use super::{flags::StatusFlags, memory::Mem};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    Implied,
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
    pub sp: u8,
    pub bus: Bus,
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::UNUSED | StatusFlags::BREAK,
            program_counter: 0x8000,
            sp: 0xff,
            bus: bus,
        }
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

#[cfg(test)]
impl CPU {
    pub fn test_new() -> Self {
        CPU::new(Bus::test_new())
    }

    pub fn load(&mut self, program: Vec<u8>) {
        use crate::rom::Rom;

        let rom = Rom::from_test_code(program);
        self.bus.load_rom(rom);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::test_new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }
}
