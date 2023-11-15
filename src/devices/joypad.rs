use crate::devices::io::Device;
use crate::app::utils::DeviceResult;

pub struct Joypad {
    pub buffer: u8,
    changed: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            buffer: 0,
            changed: false,
        }
    }
}

impl Device for Joypad {
    fn read(&self, addr: u16) -> DeviceResult<u8> {
        match addr {
            0xff00 => Ok(self.buffer),
            _ => Err(format!("Invalid read address for Joypad: 0x{:04X}", addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> DeviceResult<()> {
        match addr {
            0xff00 => {
                self.buffer = value;
                Ok(())
            }
            _ => Err(format!("Invalid write address for Joypad: 0x{:04X}", addr)),
        }
    }

    fn check_changed(&self) -> DeviceResult<bool> {
        Ok(self.changed)
    }

    fn get_range(&self) -> (u16, u16) {
        (0xff00, 0xff00)
    }

    fn reset_changed(&mut self) -> DeviceResult<()> {
        self.changed = false;
        Ok(())
    }
}
