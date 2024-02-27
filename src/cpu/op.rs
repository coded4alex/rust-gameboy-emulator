use crate::app::{gameboy::Gameboy, utils::DataResult};

pub trait Op {
    fn execute(&self, gameboy: &mut Gameboy) -> DataResult<u8>;
}
