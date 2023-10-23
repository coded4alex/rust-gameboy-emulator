
pub struct Memory {
    pub buffer: [i32; 0x10000],
}


impl Memory {
    fn read(&self, address: u32) -> i32 {
        self.buffer[address as usize]
    }

    fn write(&mut self, address: u32, value: i32) {
        self.buffer[address as usize] = value;
    }
}

pub fn create_memory() -> Memory {
    Memory {
        buffer: [0; 0x10000],
    }
}


mod test {
    #[test]
    fn test_memory() {
        use super::Memory;
        let mut mem = Memory {
            buffer: [0; 0x10000],
        };
        mem.write(0x0000, 0x01);
        assert_eq!(mem.read(0x0000), 0x01);
    }
}
