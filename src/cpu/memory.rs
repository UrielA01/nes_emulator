pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&mut self, position: u16) -> u16;
    fn mem_write_u16(&mut self, pos: u16, data: u16);
}
