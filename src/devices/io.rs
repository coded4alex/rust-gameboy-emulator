pub trait Device {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn check_changed(&self) -> bool;
    fn reset_changed(&mut self);
}