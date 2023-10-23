use super::access::Memory;

pub trait Switchable{
    fn get_valid_banks(&self) -> Vec<u8>;
    fn get_current_bank(&self) -> u8;
    fn set_current_bank(&mut self, bank: u8, mem: &mut Memory);
}