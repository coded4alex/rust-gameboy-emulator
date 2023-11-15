use crate::devices::io::Device;

pub struct SerialTransfer {
    data: u8,
    control: u8,
}

impl SerialTransfer {
    pub fn new() -> Self {
        Self {
            data: 0,
            control: 0,
        }
    }

    pub fn send_data(&mut self, data: u8) {
        self.data = data;
    }

    pub fn receive_data(&self) -> u8 {
        self.data
    }

    pub fn send_control(&mut self, control: u8) {
        self.control = control;
    }

    pub fn receive_control(&self) -> u8 {
        self.control
    }
}

impl Device for SerialTransfer {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => panic!("Invalid address for SerialTransfer: 0x{:04X}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF01 => self.data = value,
            0xFF02 => self.control = value,
            _ => panic!("Invalid address for SerialTransfer: 0x{:04X}", addr),
        }
    }

    fn check_changed(&self) -> bool {
        false
    }

    fn reset_changed(&mut self) {}

    fn get_range(&self) -> (u16, u16) {
        (0xFF01, 0xFF02)
    }
}


mod test {
    use crate::devices::io::Device;

    #[test]
    fn test_serial_transfer() {
        use super::SerialTransfer;
        let mut serial_transfer = SerialTransfer::new();
        serial_transfer.write(0xFF01, 0x01);
        assert_eq!(serial_transfer.read(0xFF01), 0x01);
    }
}