use crate::app::{gameboy::Gameboy, utils::DataResult};

pub type OperandExecuter = fn(&mut Gameboy, OperandStructure) -> DataResult<u8>;

pub enum OperandType {
    N8,
    N16,
    A8,
    A16,
    E8,
}

pub struct OperandStructure {
    pub operand1_key: OperandType,
    pub operand1_value: u16,
    pub operand2_key: OperandType,
    pub operand2_value: u16,
}

impl OperandStructure {
    pub fn create() -> OperandStructure {
        OperandStructure {
            operand1_key: OperandType::N8,
            operand1_value: 0,
            operand2_key: OperandType::N8,
            operand2_value: 0,
        }
    }
}
