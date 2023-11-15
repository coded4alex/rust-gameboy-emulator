use crate::devices::io::Device;
use crate::app::utils::DeviceResult;

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
    fn read(&self, addr: u16) -> DeviceResult<u8> {
        Ok(self.read(addr))
    }

    fn write(&mut self, addr: u16, value: u8) -> DeviceResult<()> {
        self.write(addr, value);
        Ok(())
    }

    fn check_changed(&self) -> DeviceResult<bool> {
        Ok(false)
    }

    fn reset_changed(&mut self) -> DeviceResult<()> {
        Ok(())
    }

    fn get_range(&self) -> (u16, u16) {
        (0xFF30, 0xFF3F)
    }
}
