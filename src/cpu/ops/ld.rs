use crate::{
    app::{gameboy::Gameboy, utils::DataResult},
    cpu::{
        op::{OperandStructure, OperandType},
        registers::RegisterNames,
    },
};

fn ld_01(gameboy: &mut Gameboy, operands: OperandStructure) -> DataResult<u8> {
    match operands.operand1_key {
        OperandType::N16 => {
            gameboy.registers.set_register(RegisterNames::BC, operands.operand1_value)?;
            Ok(0)
        }
        _ => Err("Invalid Operand {} for LD".to_string()),
    }
}

fn ld_02(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let value = gameboy.registers.get_register(RegisterNames::A)? as u8;
    let address = gameboy.registers.get_register(RegisterNames::BC)?;
    gameboy.memory.write(address, value);
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{
        app::gameboy::Gameboy,
        cpu::{
            op::{OperandStructure, OperandType},
            registers::RegisterNames,
        },
    };

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
    fn test_Ld02() {
        let mut gameboy = Gameboy::create();
        let operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::BC, 1024).unwrap();
        gameboy.registers.set_register(RegisterNames::A, 10).unwrap();
        ld_02(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.memory.read(1024), 10);
    }
}
