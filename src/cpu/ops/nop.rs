use crate::app::{gameboy::Gameboy, utils::DataResult};
use crate::cpu::op::OperandStructure;

fn nop_00(_gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    Ok(1)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::gameboy::Gameboy;
    use crate::cpu::op::OperandStructure;

    #[test]
    fn test_nop() {
        let mut test_gameboy = Gameboy::create();
        assert_eq!(nop_00(&mut test_gameboy, OperandStructure::create()).unwrap(), 1)
    }
}
