use crate::app::utils::DeviceResult;

pub trait Device {
    fn read(&self, addr: u16) -> DeviceResult<u8>;
    fn write(&mut self, addr: u16, value: u8) -> DeviceResult<()>;

    fn check_changed(&self) -> DeviceResult<bool>;
    fn reset_changed(&mut self) -> DeviceResult<()>;

    fn get_range(&self) -> (u16, u16);
}
