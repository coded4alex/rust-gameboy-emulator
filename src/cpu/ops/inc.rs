use crate::{
    app::{gameboy::Gameboy, utils::DataResult},
    cpu::{
        op::OperandStructure,
        registers::{RegisterFlags, RegisterNames},
    },
};

pub fn inc_03(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let mut value = gameboy.registers.get_register(RegisterNames::BC)? + 1;

    if value == 256 {
        value = 0;
        gameboy.registers.set_flag(RegisterFlags::C, 1)?;
        gameboy.registers.set_flag(RegisterFlags::Z, 1)?;
    }

    gameboy.registers.set_register(RegisterNames::BC, value)?;
    Ok(0)
}

pub fn inc_04(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let mut value = gameboy.registers.get_register(RegisterNames::B)? + 1;

    if value == 256 {
        value = 0;
        gameboy.registers.set_flag(RegisterFlags::C, 1)?;
        gameboy.registers.set_flag(RegisterFlags::Z, 1)?;
    }

    gameboy.registers.set_register(RegisterNames::B, value)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{
        app::gameboy::Gameboy,
        cpu::{op::OperandStructure, registers::RegisterNames},
    };

    use super::*;

    #[test]
    fn test_inc_03() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        inc_03(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::BC).unwrap(), 1);

        operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::BC, 255).unwrap();
        inc_03(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::BC).unwrap(), 0);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::C), 1);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::Z), 1);
    }

    #[test]
    fn test_inc_04() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        inc_04(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 1);

        operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::B, 255).unwrap();
        inc_04(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 0);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::C), 1);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::Z), 1);
    }
}
