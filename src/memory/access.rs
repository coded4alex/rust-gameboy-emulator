pub struct Memory {
    pub buffer: [u8; 0x10000],
}

impl Memory {
    pub fn create() -> Memory {
        Memory { buffer: [0; 0x10000] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.buffer[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.buffer[address as usize] = value;
    }
}

mod test {
    #[test]
    fn test_memory() {
        use super::Memory;
        let mut mem = Memory::create();
        mem.write(0x0000, 0x01);
        assert_eq!(mem.read(0x0000), 0x01);
    }
}
