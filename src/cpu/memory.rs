use crate::cpu::cpu::CPU;

impl CPU {
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&mut self, position: u16) -> u16 {
        let low_bit = self.mem_read(position) as u16;
        let high_bit = self.mem_read(position + 1) as u16;
        (high_bit << 8) | (low_bit as u16)
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high_bit = (data >> 8) as u8;
        let low_bit = (data & 0xff) as u8;
        self.mem_write(pos, low_bit);
        self.mem_write(pos + 1, high_bit);
    }
}
