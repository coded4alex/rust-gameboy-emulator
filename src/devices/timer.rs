use crate::devices::io::Device;
use crate::app::utils::DeviceResult;

pub struct Timer {
    // TODO: Implement timer fields
}

impl Timer {
    pub fn new() -> Self {
        // TODO: Initialize timer fields
        Self {}
    }

    pub fn tick(&mut self, cycles: u32) {
        // TODO: Implement timer tick logic
    }
}

impl Device for Timer {
    fn read(&self, addr: u16) -> DeviceResult<u8> {
        Err(String::from("Unimplemented"))
    }

    fn write(&mut self, addr: u16, value: u8) -> DeviceResult<()> {
        Err(String::from("Unimplemented"))
    }

    fn check_changed(&self) -> DeviceResult<bool> {
        Err(String::from("Unimplemented"))
    }

    fn reset_changed(&mut self) -> DeviceResult<()> {
        Err(String::from("Unimplemented"))
    }

    fn get_range(&self) -> (u16, u16) {
        (0xff04, 0xff07)
    }
}