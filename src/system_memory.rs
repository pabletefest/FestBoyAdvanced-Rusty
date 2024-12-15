const IWRAM_SIZE: usize = 32 * 1024;
const EWRAM_SIZE: usize = 256 * 1024;
const VRAM_SIZE: usize = 96 * 1024;
const OAM_SIZE: usize = 1 * 1024;
const PAL_RAM_SIZE: usize = 1 * 1024;
pub trait MemoryOperation {
    fn read8(&self, address: usize) -> u8;

    fn read16(&self, address: usize) -> u16 {
        let lo : u16 = self.read8(address) as u16;
        let hi : u16 = self.read8(address.wrapping_add(1)) as u16;

        (hi << 8) | lo
    }
    
    fn read32(&self, address: usize) -> u32 {
        let lo : u32 = self.read16(address) as u32;
        let hi : u32 = self.read16(address.wrapping_add(2)) as u32;

        (hi << 16) | lo
    }

    fn write8(&mut self, address: usize, value: u8);
    
    fn write16(&mut self, address: usize, value: u16) {
        self.write8(address, value as u8 & 0xFFu8);
        self.write8(address.wrapping_add(1), (value >> 8) as u8 & 0xFFu8);
    }
    
    fn write32(&mut self, address: usize, value: u32) {
        self.write16(address, value as u16 & 0xFFFFu16);
        self.write16(address.wrapping_add(2), (value >> 16) as u16 & 0xFFFFu16);
    }
}

pub struct SysMem {
    iwram: [u8; IWRAM_SIZE],
    ewram: [u8; EWRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    pal_ram: [u8; PAL_RAM_SIZE]
}

impl MemoryOperation for SysMem {
    fn read8(&self, address: usize) -> u8 {
        if address <= 0x0000_3FFF {
            0
        } else if address >= 0x0200_0000 && address <= 0x0203_FFFF{
            self.ewram[address & 0x3FFFF]
        } else if address >= 0x0300_0000 && address <= 0x0300_7FFF {
            self.iwram[address & 0x7FFF]
        } else if address >= 0x0400_0000 && address <= 0x0400_03FF {
            0
        } else if address >= 0x0500_0000 && address <= 0x0500_03FF {
            self.pal_ram[address & 0x3FF]
        } else if address >= 0x0600_0000 && address <= 0x0601_7FFF {
            self.vram[address & 0x17FFF]
        } else if address >= 0x0700_0000 && address <= 0x0700_03FF {
            self.oam[address & 0x3FF]
        }
        else {
            panic!("Reading at memory address {address:#X}!")
        }
    }

    fn write8(&mut self, address: usize, value: u8) {
        if address <= 0x0000_3FFF {
            
        } else if address >= 0x0200_0000 && address <= 0x0203_FFFF{
            self.ewram[address & 0x3FFFF] = value;
        } else if address >= 0x0300_0000 && address <= 0x0300_7FFF {
            self.iwram[address & 0x7FFF] = value;
        } else if address >= 0x0400_0000 && address <= 0x0400_03FF {
            
        } else if address >= 0x0500_0000 && address <= 0x0500_03FF {
            self.pal_ram[address & 0x3FF] = value;
        } else if address >= 0x0600_0000 && address <= 0x0601_7FFF {
            self.vram[address & 0x17FFF] = value;
        } else if address >= 0x0700_0000 && address <= 0x0700_03FF {
            self.oam[address & 0x3FF] = value;
        }
        else {
            panic!("Writting at memory address {address:#X}!")
        }
    }
}