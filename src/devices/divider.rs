use crate::devices::io::Device;
use crate::app::utils::DeviceResult;

pub struct Divider {
    counter: u8,
}

impl Divider {
    pub fn new() -> Self {
        Divider { counter: 0 }
    }

    pub fn tick(&mut self, cycles: u32) {
        let old_counter = self.counter;
        self.counter = self.counter.wrapping_add(cycles as u8);
        if self.counter < old_counter {
            // Overflow occurred, reset to 0
            self.counter = 0;
        }
    }
}

impl Device for Divider {
    fn read(&self, addr: u16) -> DeviceResult<u8> {
        match addr {
            0xff04 => Ok(self.counter),
            _ => Err(format!("Invalid divider address: 0x{:04X}", addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> DeviceResult<()> {
        match addr {
            0xff04 => {
                self.counter = 0;
                Ok(())
            }
            _ => Err(format!("Invalid divider address: 0x{:04X}", addr)),
        }
    }

    fn check_changed(&self) -> DeviceResult<bool> {
        Ok(false)
    }

    fn reset_changed(&mut self) -> DeviceResult<()> {
        Ok(())
    }

    fn get_range(&self) -> (u16, u16) {
        (0xff04, 0xff04)
    }
}
