use crate::{app::{gameboy::Gameboy, utils::DataResult}, cpu::{op::OperandStructure, registers::RegisterNames}};

pub fn dec_05(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let value = gameboy.registers.get_register(RegisterNames::B)?;
    gameboy.registers.set_register(RegisterNames::B, value-1)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{app::gameboy::Gameboy, cpu::{op::OperandStructure, registers::RegisterNames}};

    use super::dec_05;

    #[test]
    fn test_dec_05() {
        let mut gameboy = Gameboy::create();
        let operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::B, 1).unwrap();
        dec_05(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 0)
    }
}