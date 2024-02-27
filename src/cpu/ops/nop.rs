use crate::cpu::registers::RegisterNames;
use crate::{
    app::{gameboy::Gameboy, utils::DataResult},
    cpu::op::Op,
};

pub struct Nop {}

impl Op for Nop {
    fn execute(&self, gameboy: &mut Gameboy) -> DataResult<u8> {
        let val = gameboy.registers.get_register(RegisterNames::PC)?;
        gameboy.registers.set_register(RegisterNames::PC, val + 1)?;
        Ok(1)
    }
}

mod test {
    use super::Nop;
    use crate::app::gameboy::Gameboy;
    use crate::cpu::op::Op;

    #[test]
    fn test_nop() {
        let mut test_gameboy = Gameboy::create();
        let nop = Nop {};
        assert_eq!(nop.execute(&mut test_gameboy).unwrap(), 1)
    }
}
