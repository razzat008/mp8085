// opcodes 8085
use mp8085::AddressingMode;
#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            code: code,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}

// lazy_static! {
//     pub static ref OPCODES_8085: Vec<OpCode> = vec![OpCode::new(
//         0xCE,
//         "ACI",
//         2,
//         7,
//         AddressingMode::ImmediateMode,
//     )];
// }
