// opcodes 8085
use mp8085::AddressingMode;
use std::sync::LazyLock;
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
        OpCode::new(0x86, "ADD", 1, 4, AddressingMode::RegisterMode),
        OpCode::new(0x87, "ADD", 1, 4, AddressingMode::RegisterMode),

        // add immediate to accumulator
        OpCode::new(0xC6, "ADI", 2, 7, AddressingMode::ImmediateMode),

        // logical and with accumulator
        OpCode::new(0xC6, "ANA", 2, 7, AddressingMode::ImmediateMode),


    ]
});
