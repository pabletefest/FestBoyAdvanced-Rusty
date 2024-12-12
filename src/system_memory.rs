pub trait MemoryOperation {
    fn read8(&self, address: u32) -> u8;

    fn read16(&self, address: u32) -> u16 {
        let lo : u16 = self.read8(address) as u16;
        let hi : u16 = self.read8(address.wrapping_add(1)) as u16;

        (hi << 8) | lo
    }
    
    fn read32(&self, address: u32) -> u32 {
        let lo : u32 = self.read16(address) as u32;
        let hi : u32 = self.read16(address.wrapping_add(2)) as u32;

        (hi << 16) | lo
    }

    fn write8(&mut self, address: u32, value: u8);
    
    fn write16(&mut self, address: u32, value: u16) {
        self.write8(address, value as u8 & 0xFFu8);
        self.write8(address.wrapping_add(1), (value >> 8) as u8 & 0xFFu8);
    }
    
    fn write32(&mut self, address: u32, value: u32) {
        self.write16(address, value as u16 & 0xFFFFu16);
        self.write16(address.wrapping_add(2), (value >> 16) as u16 & 0xFFFFu16);
    }
}
