use crate::devices::io::Device;

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
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => {
                self.buffer
            },
            _ => panic!("Invalid address for Joypad: 0x{:04X}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF00 => {
                self.buffer = value;
            },
            _ => panic!("Invalid address for Joypad: 0x{:04X}", addr),
        }
    }

    fn check_changed(&self) -> bool {
        self.changed
    }

    fn reset_changed(&mut self) {
        self.changed = false;
    }

    fn get_range(&self) -> (u16, u16) {
        (0xFF00, 0xFF00)
    }
}


mod test {
    use crate::devices::io::Device;

    #[test]
    fn test_joypad() {
        use super::Joypad;
        let mut joypad = Joypad::new();
        joypad.write(0xFF00, 0x01);
        assert_eq!(joypad.read(0xFF00), 0x01);
    }
}

