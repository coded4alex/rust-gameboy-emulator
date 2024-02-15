use crate::devices::io::Device;
use crate::app::utils::DataResult;

pub struct SoundControl {
    nr50: u8,
    nr51: u8,
    nr52: u8,
}

impl SoundControl {
    pub fn new() -> Self {
        SoundControl {
            nr50: 0,
            nr51: 0,
            nr52: 0,
        }
    }
}

impl Device for SoundControl {
    fn read(&self, address: u16) -> DataResult<u8> {
        match address {
            0xFF24 => Ok(self.nr50),
            0xFF25 => Ok(self.nr51),
            0xFF26 => Ok(self.nr52),
            _ => Err(format!("Invalid sound control address: 0x{:04X}", address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) -> DataResult<()> {
        match address {
            0xFF24 => {
                self.nr50 = value;
                Ok(())
            }
            0xFF25 => {
                self.nr51 = value;
                Ok(())
            }
            0xFF26 => {
                self.nr52 = value;
                Ok(())
            }
            _ => Err(format!("Invalid sound control address: 0x{:04X}", address)),
        }
    }

    fn check_changed(&self) -> DataResult<bool> {
        Ok(false)
    }

    fn reset_changed(&mut self) -> DataResult<()> {
        Ok(())
    }

    fn get_range(&self) -> (u16, u16) {
        (0xFF24, 0xFF26)
    }
}
