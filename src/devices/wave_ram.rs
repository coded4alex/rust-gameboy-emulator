use crate::devices::io::Device;

const WAVE_RAM_SIZE: usize = 16;

pub struct WaveRam {
    data: [u8; WAVE_RAM_SIZE],
}

impl WaveRam {
    pub fn new() -> Self {
        Self { data: [0; WAVE_RAM_SIZE] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0xFF30]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0xFF30] = value;
    }
}

impl Device for WaveRam {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }

    fn check_changed(&self) -> bool {
        false
    }

    fn reset_changed(&mut self) {}

    fn get_range(&self) -> (u16, u16) {
        (0xFF30, 0xFF3F)
    }
}


mod test{
    use super::*;

    #[test]
    fn test_read_write() {
        let mut wave_ram = WaveRam::new();

        wave_ram.write(0xFF30, 0x01);
        assert_eq!(wave_ram.read(0xFF30), 0x01);

        wave_ram.write(0xFF3F, 0x02);
        assert_eq!(wave_ram.read(0xFF3F), 0x02);
    }
}