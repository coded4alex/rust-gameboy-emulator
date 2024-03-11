use crate::{
    app::{gameboy::Gameboy, utils::DataResult},
    cpu::{
        op::{OperandStructure, OperandType},
        registers::RegisterNames,
    },
};

pub fn ld_01(gameboy: &mut Gameboy, operands: OperandStructure) -> DataResult<u8> {
    match operands.operand1_key {
        OperandType::N16 => {
            gameboy.registers.set_register(RegisterNames::BC, operands.operand1_value)?;
            Ok(0)
        }
        _ => Err("Invalid Operand for LD".to_string()),
    }
}

pub fn ld_02(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let value = gameboy.registers.get_register(RegisterNames::A)? as u8;
    let address = gameboy.registers.get_register(RegisterNames::BC)?;
    gameboy.memory.write(address, value);
    Ok(0)
}

pub fn ld_06(gameboy: &mut Gameboy, operands: OperandStructure) -> DataResult<u8> {
    match operands.operand1_key {
        OperandType::N8 => {
            let value = operands.operand1_value;
            gameboy.registers.set_register(RegisterNames::B, value)?;
            Ok(0)
        }
        _ => Err("Invalid Operand for LD".to_string()),
    }
}

pub fn ld_08(gameboy: &mut Gameboy, operands: OperandStructure) -> DataResult<u8> {
    let value = gameboy.memory.read(operands.operand1_value);
    gameboy.registers.set_register(RegisterNames::SP, value as u16)?;
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ld_01() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        operands.operand1_key = OperandType::N16;
        operands.operand1_value = 1234;
        ld_01(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::BC).unwrap(), 1234);

        operands = OperandStructure::create();
        operands.operand1_key = OperandType::A16;
        assert_eq!(ld_01(&mut gameboy, operands).is_err(), true);
    }

    #[test]
    fn test_ld02() {
        let mut gameboy = Gameboy::create();
        let operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::BC, 1024).unwrap();
        gameboy.registers.set_register(RegisterNames::A, 10).unwrap();

        ld_02(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.memory.read(1024), 10);
    }

    #[test]
    fn test_ld06() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        operands.operand1_key = OperandType::N8;
        operands.operand1_value = 10;

        ld_06(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 10);
    }

    #[test]
    fn test_ld08() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        gameboy.memory.write(10, 10);
        operands.operand1_value = 10;

        ld_08(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::SP).unwrap(), 10);
    }
}
