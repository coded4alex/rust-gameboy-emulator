use crate::devices::io::Device;
use crate::app::utils::DataResult;

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
    fn read(&self, addr: u16) -> DataResult<u8> {
        match addr {
            0xff0f => Ok(self.interrupt_flag),
            0xffff => Ok(self.interrupt_enable),
            _ => Err(format!("Invalid read address for InterruptFlag: 0x{:04X}", addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> DataResult<()> {
        match addr {
            0xff0f => {
                self.interrupt_flag = value;
                Ok(())
            }
            0xffff => {
                self.interrupt_enable = value;
                Ok(())
            }
            _ => Err(format!("Invalid write address for InterruptFlag: 0x{:04X}", addr)),
        }
    }

    fn check_changed(&self) -> DataResult<bool> {
        Ok(false)
    }

    fn reset_changed(&mut self) -> DataResult<()> {
        Ok(())
    }

    fn get_range(&self) -> (u16, u16) {
        (0xff0f, 0xffff)
    }
}

