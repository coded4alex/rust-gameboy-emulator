use crate::{app::{gameboy::Gameboy, utils::DataResult}, cpu::{op::OperandStructure, registers::{RegisterFlags, RegisterNames}}};

pub fn rlca_07(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let mut value = gameboy.registers.get_register(RegisterNames::A)?;
    let flag_bit = 1 << 7;
    
    if value & flag_bit != 0 {
        gameboy.registers.set_flag(RegisterFlags::C, 1)?;
        value = value & 127;
    }
    
    gameboy.registers.set_register(RegisterNames::A, value<<1)?;
    Ok(0)
}

pub fn add_08(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let a = gameboy.registers.get_register(RegisterNames::HL)?;
    let b = gameboy.registers.get_register(RegisterNames::BC)?;

    let sum = add(gameboy, a, b)?;
    gameboy.registers.set_register(RegisterNames::HL, sum)?;

    Ok(0)
}

pub fn add(gameboy: &mut Gameboy, a: u16, b:u16) -> DataResult<u16> {
    let half_value = a & 15;

    if half_value + b > 255 {
        gameboy.registers.set_flag(RegisterFlags::H, 1)?;
    }

    let mut sum: u32 = a as u32 + b as u32;
    if sum > 65535 {
        gameboy.registers.set_flag(RegisterFlags::C, 1)?;
        gameboy.registers.set_flag(RegisterFlags::H, 1)?;
        sum = sum & 65535;
    }

    Ok(sum as u16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rlca_07() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::A, 1).unwrap();
        rlca_07(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::A).unwrap(), 2);

        operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::A, 128).unwrap();
        rlca_07(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::A).unwrap(), 0);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::C), 1);
    }

    #[test]
    fn test_add() {
        let mut gameboy = Gameboy::create();
        assert_eq!(add(&mut gameboy, 1, 2).unwrap(), 3);
        assert_eq!(add(&mut gameboy, 65535, 1).unwrap(), 0);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::C), 1);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::H), 1);

        gameboy.registers.set_flag(RegisterFlags::C, 0).unwrap();
        add(&mut gameboy, 255, 1).unwrap();
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::H), 1);
    }

    #[test]
    fn test_add_08() {
        let mut gameboy = Gameboy::create();
        let operands = OperandStructure::create();

        gameboy.registers.set_register(RegisterNames::HL, 1).unwrap();
        gameboy.registers.set_register(RegisterNames::BC, 2).unwrap();
        add_08(&mut gameboy, operands).unwrap();

        assert_eq!(gameboy.registers.get_register(RegisterNames::HL).unwrap(), 3);
    }
}