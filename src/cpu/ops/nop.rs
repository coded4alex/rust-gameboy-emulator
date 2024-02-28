use crate::app::{gameboy::Gameboy, utils::DataResult};
use crate::cpu::op::OperandStructure;
use crate::cpu::registers::RegisterNames;

fn nop_00(gameboy: &mut Gameboy, operands: OperandStructure) -> DataResult<u8> {
    let val = gameboy.registers.get_register(RegisterNames::PC)?;
    gameboy.registers.set_register(RegisterNames::PC, val + 1)?;
    Ok(1)
}

mod test {
    use super::nop_00;
    use crate::app::gameboy::Gameboy;
    use crate::cpu::op::OperandStructure;

    #[test]
    fn test_nop() {
        let mut test_gameboy = Gameboy::create();
        assert_eq!(nop_00(&mut test_gameboy, OperandStructure::create()).unwrap(), 1)
    }
}
