use crate::devices::io::Device;

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
    fn read(&self, address: u16) -> u8 {
        match address {
            0xff04 => self.counter,
            _ => panic!("Invalid read address for Divider: 0x{:04x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => self.counter = 0,
            _ => panic!("Invalid write address for Divider: 0x{:04x}", address),
        }
    }

    fn check_changed(&self) -> bool {
        false
    }

    fn reset_changed(&mut self) {}

    fn get_range(&self) -> (u16, u16) {
        (0xff04, 0xff04)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid read address for Divider")]
    fn test_divider_read_invalid_address() {
        let divider = Divider::new();
        divider.read(0x1234);
    }

    #[test]
    #[should_panic(expected = "Invalid write address for Divider")]
    fn test_divider_write_invalid_address() {
        let mut divider = Divider::new();
        divider.write(0x1234, 0);
    }

    #[test]
    fn test_divider_write_reset() {
        let mut divider = Divider::new();
        divider.counter = 0x42;
        divider.write(0xff04, 0);
        assert_eq!(divider.counter, 0);
    }
}
