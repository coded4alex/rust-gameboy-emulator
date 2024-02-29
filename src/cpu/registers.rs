use crate::app::utils::DataResult;

pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

#[derive(Clone)]
pub enum RegisterNames {
    A,
    F,
    AF,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,
    SP,
    PC,
}

pub enum RegisterFlags {
    Z,
    N,
    H,
    C,
}

const UPPER_MASK: u16 = 0b0000000011111111;
const LOWER_MASK: u16 = 0b1111111100000000;

fn get_flag_mask(flag: RegisterFlags) -> u16 {
    match flag {
        RegisterFlags::Z => 7,
        RegisterFlags::N => 6,
        RegisterFlags::H => 5,
        RegisterFlags::C => 4,
    }
}

impl Registers {
    pub fn create() -> Registers {
        Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_flag(&self, flag: RegisterFlags) -> u16 {
        let mask = get_flag_mask(flag);
        (self.af & (1 << mask)) >> mask
    }

    pub fn set_flag(&mut self, flag: RegisterFlags, value: u16) -> DataResult<u8> {
        if value != 0 && value != 1 {
            return Err("Invalid value for a flag".to_string());
        }
        let mask = get_flag_mask(flag);
        let masked = self.af & !(1 << mask);
        self.af = masked | (value << mask);
        Ok(0)
    }

    pub fn get_register(&self, register: RegisterNames) -> DataResult<u16> {
        match register {
            RegisterNames::A => get_msb_8(self.af),
            RegisterNames::B => get_msb_8(self.bc),
            RegisterNames::C => get_lsb_8(self.bc),
            RegisterNames::D => get_msb_8(self.de),
            RegisterNames::E => get_lsb_8(self.de),
            RegisterNames::H => get_msb_8(self.hl),
            RegisterNames::L => get_lsb_8(self.hl),
            RegisterNames::BC => Ok(self.bc),
            RegisterNames::DE => Ok(self.de),
            RegisterNames::HL => Ok(self.hl),
            RegisterNames::PC => Ok(self.pc),
            RegisterNames::SP => Ok(self.sp),
            RegisterNames::F => Err("Accessing F register is impossible".to_string()),
            RegisterNames::AF => Err("Accessing AF register is impossible".to_string()),
        }
    }

    pub fn set_register(&mut self, register: RegisterNames, value: u16) -> DataResult<u16> {
        match register {
            RegisterNames::A => set_msb_8(&mut self.af, value),
            RegisterNames::B => set_msb_8(&mut self.bc, value),
            RegisterNames::C => set_lsb_8(&mut self.bc, value),
            RegisterNames::D => set_msb_8(&mut self.de, value),
            RegisterNames::E => set_lsb_8(&mut self.de, value),
            RegisterNames::H => set_msb_8(&mut self.hl, value),
            RegisterNames::L => set_lsb_8(&mut self.hl, value),
            RegisterNames::BC => Ok({
                self.bc = value;
                self.bc
            }),
            RegisterNames::DE => Ok({
                self.de = value;
                self.de
            }),
            RegisterNames::HL => Ok({
                self.hl = value;
                self.hl
            }),
            RegisterNames::PC => Ok({
                self.pc = value;
                self.pc
            }),
            RegisterNames::SP => Ok({
                self.sp = value;
                self.sp
            }),
            _ => Err("Accessing this register is impossible".to_string()),
        }
    }
}

fn get_msb_8(value: u16) -> DataResult<u16> {
    Ok((value & LOWER_MASK) >> 8)
}

fn get_lsb_8(value: u16) -> DataResult<u16> {
    Ok(value & UPPER_MASK)
}

fn set_msb_8(register_value: &mut u16, value: u16) -> DataResult<u16> {
    *register_value = (*register_value & UPPER_MASK) + (value << 8);
    Ok(*register_value)
}

fn set_lsb_8(register_value: &mut u16, value: u16) -> DataResult<u16> {
    *register_value = (*register_value & LOWER_MASK) + value;
    Ok(*register_value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registers() {
        let mut registers = Registers::create();
        registers.set_register(RegisterNames::B, 1).unwrap();
        registers.set_register(RegisterNames::C, 4).unwrap();
        assert_eq!(registers.get_register(RegisterNames::B).unwrap(), 1);
        assert_eq!(registers.get_register(RegisterNames::C).unwrap(), 4);
        assert_eq!(registers.get_register(RegisterNames::BC).unwrap(), 260);
        assert!(registers.get_register(RegisterNames::AF).is_err())
    }

    #[test]
    fn test_flags() {
        let mut registers = Registers::create();
        registers.set_flag(RegisterFlags::C, 1).unwrap();
        registers.set_flag(RegisterFlags::H, 1).unwrap();
        registers.set_flag(RegisterFlags::N, 1).unwrap();
        registers.set_flag(RegisterFlags::Z, 1).unwrap();

        assert_eq!(registers.get_flag(RegisterFlags::C), 1);
        assert_eq!(registers.get_flag(RegisterFlags::H), 1);
        assert_eq!(registers.get_flag(RegisterFlags::N), 1);
        assert_eq!(registers.get_flag(RegisterFlags::Z), 1);
    }
}
