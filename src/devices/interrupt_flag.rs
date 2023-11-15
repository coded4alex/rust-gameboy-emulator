use crate::devices::io::Device;

pub struct InterruptFlag {
    interrupt_enable: u8,
    interrupt_flag: u8,
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {
            interrupt_enable: 0,
            interrupt_flag: 0,
        }
    }

    pub fn set_interrupt(&mut self, interrupt: u8) {
        self.interrupt_flag |= interrupt;
    }

    pub fn clear_interrupt(&mut self, interrupt: u8) {
        self.interrupt_flag &= !interrupt;
    }

    pub fn is_interrupt_enabled(&self, interrupt: u8) -> bool {
        self.interrupt_enable & interrupt != 0
    }

    pub fn is_interrupt_requested(&self, interrupt: u8) -> bool {
        self.interrupt_flag & interrupt != 0
    }
}

impl Device for InterruptFlag {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xff0f => self.interrupt_flag,
            0xffff => self.interrupt_enable,
            _ => panic!("Invalid read address for InterruptFlag: 0x{:04x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xff0f => self.interrupt_flag = value,
            0xffff => self.interrupt_enable = value,
            _ => panic!("Invalid write address for InterruptFlag: 0x{:04x}", address),
        }
    }

    fn check_changed(&self) -> bool {
        false
    }

    fn reset_changed(&mut self) {}

    fn get_range(&self) -> (u16, u16) {
        (0xff0f, 0xff0f)
    }
}

mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid read address for InterruptFlag")]
    fn test_interrupt_flag_read_invalid_address() {
        let interrupt_flag = InterruptFlag::new();
        interrupt_flag.read(0x1234);
    }

    #[test]
    #[should_panic(expected = "Invalid write address for InterruptFlag")]
    fn test_interrupt_flag_write_invalid_address() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.write(0x1234, 0);
    }

    #[test]
    fn test_interrupt_flag_write() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.write(0xff0f, 0x42);
        assert_eq!(interrupt_flag.interrupt_flag, 0x42);
        interrupt_flag.write(0xffff, 0x42);
        assert_eq!(interrupt_flag.interrupt_enable, 0x42);
    }

    #[test]
    fn test_interrupt_flag_read() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.interrupt_flag = 0x42;
        interrupt_flag.interrupt_enable = 0x42;
        assert_eq!(interrupt_flag.read(0xff0f), 0x42);
        assert_eq!(interrupt_flag.read(0xffff), 0x42);
    }

    #[test]
    fn test_interrupt_flag_set_interrupt() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.set_interrupt(0x42);
        assert_eq!(interrupt_flag.interrupt_flag, 0x42);
    }

    #[test]
    fn test_interrupt_flag_clear_interrupt() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.interrupt_flag = 0x42;
        interrupt_flag.clear_interrupt(0x42);
        assert_eq!(interrupt_flag.interrupt_flag, 0);
    }

    #[test]
    fn test_interrupt_flag_is_interrupt_enabled() {
        let mut interrupt_flag = InterruptFlag::new();
        interrupt_flag.interrupt_enable = 0x42;
        assert!(interrupt_flag.is_interrupt_enabled(0x42));
        assert!(!interrupt_flag.is_interrupt_enabled(0x21));
    }
}