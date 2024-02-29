use crate::{
    app::{gameboy::Gameboy, utils::DataResult},
    cpu::{
        op::OperandStructure,
        registers::{RegisterFlags, RegisterNames},
    },
};

pub fn dec_05(gameboy: &mut Gameboy, _operands: OperandStructure) -> DataResult<u8> {
    let mut value = gameboy.registers.get_register(RegisterNames::B)?;

    if value == 1 {
        gameboy.registers.set_flag(RegisterFlags::Z, 1)?;
    } else if value == 0 {
        value = 256;
    } else if (value) & 0b1111 == 0 {
        gameboy.registers.set_flag(RegisterFlags::H, 1)?;
    }

    gameboy.registers.set_flag(RegisterFlags::N, 1)?;
    gameboy.registers.set_register(RegisterNames::B, value - 1)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{
        app::gameboy::Gameboy,
        cpu::{
            op::OperandStructure,
            registers::{RegisterFlags, RegisterNames},
        },
    };

    use super::dec_05;

    #[test]
    fn test_dec_05() {
        let mut gameboy = Gameboy::create();
        let mut operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::B, 1).unwrap();
        dec_05(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 0);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::Z), 1);

        operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::B, 0).unwrap();
        dec_05(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 255);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::N), 1);

        operands = OperandStructure::create();
        gameboy.registers.set_register(RegisterNames::B, 32).unwrap();
        dec_05(&mut gameboy, operands).unwrap();
        assert_eq!(gameboy.registers.get_register(RegisterNames::B).unwrap(), 31);
        assert_eq!(gameboy.registers.get_flag(RegisterFlags::H), 1);
    }
}
