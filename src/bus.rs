use crate::{cpu::memory::Mem, rom::Rom};

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

// APU and I/O
pub const APU_IO_REGISTERS: u16 = 0x4000;
pub const APU_IO_REGISTERS_END: u16 = 0x4017;
// APU test mode and usually disabled I/O
pub const APU_IO_DISABLED_START: u16 = 0x4018;
pub const APU_IO_DISABLED_END: u16 = 0x401F;

// Program ROM
pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END: u16 = 0xFFFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut mapped_addr = (addr - 0x8000) as usize;
        if self.rom.prg_rom.len() == 0x4000 {
            // Mirror is needed
            mapped_addr %= 0x4000;
        }
        self.rom.prg_rom[mapped_addr]
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirron_down_addr = addr & 0b00100000_00000111;
                // todo!("Support PPU");
                0xff
            }
            PRG_ROM_START..=PRG_ROM_END => self.read_prg_rom(addr),

            APU_IO_REGISTERS..=APU_IO_REGISTERS_END => {
                // For now, stub it out safely
                // Eventually you can emulate APU or controller
                // todo!("Support APU");
                0xff
            }

            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirron_down_addr = addr & 0b00100000_00000111;
                todo!("Support PPU")
            }
            PRG_ROM_START..=PRG_ROM_END => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            APU_IO_REGISTERS..=APU_IO_REGISTERS_END => {
                // println!("Write to APU or IO: {:04X} <- {:02X}", addr, data);
                // Eventually implement APU and controller behavior here
                // todo!("Support APU");
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}

#[cfg(test)]
impl Bus {
    pub fn test_new() -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom: Rom::from_test_code(vec![]),
        }
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.rom = rom;
    }
}
