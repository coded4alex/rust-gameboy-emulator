use crate::devices::io::Device;

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
    fn read(&self, address: u16) -> u8 {
        0
    }

    fn write(&mut self, address: u16, value: u8) {
    }

    fn check_changed(&self) -> bool {
        false
    }

    fn reset_changed(&mut self) {
    }

    fn get_range(&self) -> (u16, u16) {
        (0xFF05, 0xFF07)
    }
}