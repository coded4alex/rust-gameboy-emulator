use crate::app::utils::DataResult;

pub trait Device {
    fn read(&self, addr: u16) -> DataResult<u8>;
    fn write(&mut self, addr: u16, value: u8) -> DataResult<()>;

    fn check_changed(&self) -> DataResult<bool>;
    fn reset_changed(&mut self) -> DataResult<()>;

    fn get_range(&self) -> (u16, u16);
}
