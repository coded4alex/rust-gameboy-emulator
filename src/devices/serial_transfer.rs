use crate::app::utils::DataResult;
use crate::devices::io::Device;

pub struct SerialTransfer {
    data: u8,
    control: u8,
}

impl SerialTransfer {
    pub fn new() -> Self {
        Self { data: 0, control: 0 }
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
    fn read(&self, addr: u16) -> DataResult<u8> {
        match addr {
            0xff01 => Ok(self.data),
            0xff02 => Ok(self.control),
            _ => Err(format!("Invalid read address for SerialTransfer: 0x{:04X}", addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> DataResult<()> {
        match addr {
            0xff01 => {
                self.data = value;
                Ok(())
            }
            0xff02 => {
                self.control = value;
                Ok(())
            }
            _ => Err(format!("Invalid write address for SerialTransfer: 0x{:04X}", addr)),
        }
    }

    fn check_changed(&self) -> DataResult<bool> {
        Ok(false)
    }

    fn get_range(&self) -> (u16, u16) {
        (0xff01, 0xff02)
    }

    fn reset_changed(&mut self) -> DataResult<()> {
        Ok(())
    }
}
