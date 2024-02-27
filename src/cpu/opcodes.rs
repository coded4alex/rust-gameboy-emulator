use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct Opcodes {
    unprefixed: HashMap<String, Opcode>,
    cbprefixed: HashMap<String, Opcode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Opcode {
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Option<Vec<Operand>>,
    immediate: bool,
    flags: Flags,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operand {
    name: String,
    #[serde(default)]
    bytes: Option<u8>,
    immediate: bool,
}

#[allow(warnings)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    Z: String,
    N: String,
    H: String,
    C: String,
}

impl Opcodes {}

pub fn load_opcodes(path: &str) -> Opcodes {
    let opcode_file = fs::read_to_string(path).expect("Error reading opcodes file");
    serde_json::from_str(opcode_file.as_str()).unwrap()
}
