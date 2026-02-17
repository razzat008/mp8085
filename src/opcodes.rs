// opcodes 8085
use crate::AddressingMode;
use std::{collections::HashMap, sync::LazyLock};
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

#[allow(dead_code)]
impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

#[allow(dead_code)]
pub static OPCODES_8085: LazyLock<Vec<OpCode>> = LazyLock::new(|| {
    vec![
        OpCode::new(0xCE, "ACI", 2, 7, AddressingMode::ImmediateMode),
        // add register to accumulator with carry
        OpCode::new(0x88, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x89, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8A, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8B, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8C, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8D, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8E, "ADC", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x8F, "ADC", 1, 4, AddressingMode::RegisterMode),
        // add register to accumulator
        OpCode::new(0x80, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x81, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x82, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x83, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x84, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x85, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x86, "ADD", 1, 7, AddressingMode::RegisterIndirectMode),
        OpCode::new(0x87, "ADD", 1, 4, AddressingMode::RegisterMode),
        // add immediate to accumulator
        OpCode::new(0xC6, "ADI", 2, 7, AddressingMode::ImmediateMode),
        // logical and with accumulator
        OpCode::new(0xC6, "ANA", 2, 7, AddressingMode::ImmediateMode),
        // complement accumulator
        OpCode::new(0x2F, "CMA", 1, 4, AddressingMode::ImplicitMode),
        // copy from source to destination
        OpCode::new(0x40, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x41, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x42, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x43, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x44, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x45, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x47, "MOV", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x48, "MOV", 1, 4, AddressingMode::RegisterMode),
    ]
});

#[allow(dead_code)]
pub static OPCODE_MODE: LazyLock<HashMap<u8, AddressingMode>> = LazyLock::new(move || {
    let mut opmap = HashMap::new();
    for cpuop in &*OPCODES_8085 {
        opmap.insert(cpuop.code, cpuop.mode);
    }
    opmap
});
