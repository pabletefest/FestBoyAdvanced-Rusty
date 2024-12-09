trait MemoryOperation {
    fn read(&self, address: u32) -> u8;
    fn read(&self, address: u32) -> u16;
    fn read(&self, address: u32) -> u32;

    fn write(&mut self, address: u32, value: u8);
    fn write(&mut self, address: u32, value: u16);
    fn write(&mut self, address: u32, value: u32);
}
