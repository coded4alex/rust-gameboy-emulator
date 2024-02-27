use crate::{
    cpu::{
        opcodes::{load_opcodes, Opcodes},
        registers::Registers,
    },
    memory::{
        access::Memory,
        router::{create_banks, init_banks, Bank},
    },
};

pub struct Gameboy {
    pub memory: Memory,
    pub banks: Vec<Bank>,
    pub opcodes: Opcodes,
    pub registers: Registers,
}

impl Gameboy {
    pub fn create() -> Gameboy {
        Gameboy {
            opcodes: load_opcodes("config/opcodes.json"),
            registers: Registers::create(),
            memory: Memory::create(),
            banks: create_banks(),
        }
    }

    pub fn init(&mut self) {
        init_banks(&mut self.banks);
    }
}
