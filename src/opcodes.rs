#![allow(dead_code)]
// 8085 opcode metadata table
use crate::AddressingMode;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: String,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(
        code: u8,
        mnemonic: impl Into<String>,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            mnemonic: mnemonic.into(),
            len,
            cycles,
            mode,
        }
    }
}

const REGS: [&str; 8] = ["B", "C", "D", "E", "H", "L", "M", "A"];

fn op(code: u8, mnemonic: impl Into<String>, len: u8, cycles: u8, mode: AddressingMode) -> OpCode {
    OpCode::new(code, mnemonic, len, cycles, mode)
}

#[allow(dead_code)]
pub static OPCODES_8085: LazyLock<Vec<OpCode>> = LazyLock::new(|| {
    let mut v = Vec::with_capacity(256);

    // MOV r, r
    for dst in 0..8usize {
        for src in 0..8usize {
            if dst == 6 && src == 6 {
                continue; // 0x76 is HLT
            }
            let code = 0x40 | ((dst as u8) << 3) | src as u8;
            let cycles = if dst == 6 || src == 6 { 7 } else { 4 };
            let mode = if dst == 6 || src == 6 {
                AddressingMode::RegisterIndirectMode
            } else {
                AddressingMode::RegisterMode
            };
            v.push(op(
                code,
                format!("MOV {},{}", REGS[dst], REGS[src]),
                1,
                cycles,
                mode,
            ));
        }
    }

    // MVI r, imm
    for (r, name) in REGS.iter().enumerate() {
        let code = 0x06 | ((r as u8) << 3);
        let (cycles, mode) = if r == 6 {
            (10, AddressingMode::RegisterIndirectMode)
        } else {
            (7, AddressingMode::ImmediateMode)
        };
        v.push(op(code, format!("MVI {}", name), 2, cycles, mode));
    }

    // INR / DCR r
    for (r, name) in REGS.iter().enumerate() {
        let cycles = if r == 6 { 10 } else { 4 };
        let mode = if r == 6 {
            AddressingMode::RegisterIndirectMode
        } else {
            AddressingMode::RegisterMode
        };
        v.push(op(
            0x04 | ((r as u8) << 3),
            format!("INR {}", name),
            1,
            cycles,
            mode,
        ));
        v.push(op(
            0x05 | ((r as u8) << 3),
            format!("DCR {}", name),
            1,
            cycles,
            mode,
        ));
    }

    // register ALU ops on accumulator
    for (base, name, imm_code, imm_name) in [
        (0x80, "ADD", Some(0xC6), "ADI"),
        (0x88, "ADC", Some(0xCE), "ACI"),
        (0x90, "SUB", Some(0xD6), "SUI"),
        (0x98, "SBB", Some(0xDE), "SBI"),
        (0xA0, "ANA", Some(0xE6), "ANI"),
        (0xA8, "XRA", Some(0xEE), "XRI"),
        (0xB0, "ORA", Some(0xF6), "ORI"),
        (0xB8, "CMP", Some(0xFE), "CPI"),
    ] {
        for (r, reg) in REGS.iter().enumerate() {
            let cycles = if r == 6 { 7 } else { 4 };
            let mode = if r == 6 {
                AddressingMode::RegisterIndirectMode
            } else {
                AddressingMode::RegisterMode
            };
            v.push(op(
                base | r as u8,
                format!("{} {}", name, reg),
                1,
                cycles,
                mode,
            ));
        }
        if let Some(code) = imm_code {
            v.push(op(code, imm_name, 2, 7, AddressingMode::ImmediateMode));
        }
    }

    // register pair ops
    for (name, lxi, inx, dcx, dad) in [
        ("B", 0x01u8, 0x03u8, 0x0Bu8, 0x09u8),
        ("D", 0x11, 0x13, 0x1B, 0x19),
        ("H", 0x21, 0x23, 0x2B, 0x29),
        ("SP", 0x31, 0x33, 0x3B, 0x39),
    ] {
        v.push(op(
            lxi,
            format!("LXI {}", name),
            3,
            10,
            AddressingMode::ImmediateMode,
        ));
        v.push(op(
            inx,
            format!("INX {}", name),
            1,
            6,
            AddressingMode::ImplicitMode,
        ));
        v.push(op(
            dcx,
            format!("DCX {}", name),
            1,
            6,
            AddressingMode::ImplicitMode,
        ));
        v.push(op(
            dad,
            format!("DAD {}", name),
            1,
            10,
            AddressingMode::RegisterMode,
        ));
    }

    // stack push/pop
    for (name, p, q) in [
        ("B", 0xC5, 0xC1),
        ("D", 0xD5, 0xD1),
        ("H", 0xE5, 0xE1),
        ("PSW", 0xF5, 0xF1),
    ] {
        v.push(op(
            p,
            format!("PUSH {}", name),
            1,
            12,
            AddressingMode::RegisterMode,
        ));
        v.push(op(
            q,
            format!("POP {}", name),
            1,
            10,
            AddressingMode::RegisterMode,
        ));
    }

    // LDAX / STAX
    v.push(op(
        0x0A,
        "LDAX B",
        1,
        7,
        AddressingMode::RegisterIndirectMode,
    ));
    v.push(op(
        0x1A,
        "LDAX D",
        1,
        7,
        AddressingMode::RegisterIndirectMode,
    ));
    v.push(op(
        0x02,
        "STAX B",
        1,
        7,
        AddressingMode::RegisterIndirectMode,
    ));
    v.push(op(
        0x12,
        "STAX D",
        1,
        7,
        AddressingMode::RegisterIndirectMode,
    ));

    // direct memory ops
    v.push(op(0x3A, "LDA", 3, 13, AddressingMode::DirectMode));
    v.push(op(0x32, "STA", 3, 13, AddressingMode::DirectMode));
    v.push(op(0x2A, "LHLD", 3, 16, AddressingMode::DirectMode));
    v.push(op(0x22, "SHLD", 3, 16, AddressingMode::DirectMode));
    v.push(op(0xEB, "XCHG", 1, 4, AddressingMode::ImplicitMode));

    // single-byte accumulator/flag ops
    v.push(op(0x00, "NOP", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x76, "HLT", 1, 5, AddressingMode::ImplicitMode));
    v.push(op(0x2F, "CMA", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x3F, "CMC", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x37, "STC", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x27, "DAA", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x07, "RLC", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x0F, "RRC", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x17, "RAL", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x1F, "RAR", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0xE9, "PCHL", 1, 6, AddressingMode::RegisterIndirectMode));
    v.push(op(
        0xE3,
        "XTHL",
        1,
        16,
        AddressingMode::RegisterIndirectMode,
    ));
    v.push(op(0xF9, "SPHL", 1, 6, AddressingMode::ImplicitMode));
    v.push(op(0xFB, "EI", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0xF3, "DI", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x20, "RIM", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0x30, "SIM", 1, 4, AddressingMode::ImplicitMode));
    v.push(op(0xDB, "IN", 2, 10, AddressingMode::ImmediateMode));
    v.push(op(0xD3, "OUT", 2, 10, AddressingMode::ImmediateMode));

    // jumps
    v.push(op(0xC3, "JMP", 3, 10, AddressingMode::DirectMode));
    for (code, name) in [
        (0xC2, "JNZ"),
        (0xCA, "JZ"),
        (0xD2, "JNC"),
        (0xDA, "JC"),
        (0xE2, "JPO"),
        (0xEA, "JPE"),
        (0xF2, "JP"),
        (0xFA, "JM"),
    ] {
        v.push(op(code, name, 3, 7, AddressingMode::DirectMode));
    }

    // calls
    v.push(op(0xCD, "CALL", 3, 18, AddressingMode::DirectMode));
    for (code, name) in [
        (0xC4, "CNZ"),
        (0xCC, "CZ"),
        (0xD4, "CNC"),
        (0xDC, "CC"),
        (0xE4, "CPO"),
        (0xEC, "CPE"),
        (0xF4, "CP"),
        (0xFC, "CM"),
    ] {
        v.push(op(code, name, 3, 9, AddressingMode::DirectMode));
    }

    // returns
    v.push(op(0xC9, "RET", 1, 10, AddressingMode::ImplicitMode));
    for (code, name) in [
        (0xC0, "RNZ"),
        (0xC8, "RZ"),
        (0xD0, "RNC"),
        (0xD8, "RC"),
        (0xE0, "RPO"),
        (0xE8, "RPE"),
        (0xF0, "RP"),
        (0xF8, "RM"),
    ] {
        v.push(op(code, name, 1, 6, AddressingMode::ImplicitMode));
    }

    // RST
    for n in 0..8u8 {
        v.push(op(
            0xC7 | (n << 3),
            format!("RST {}", n),
            1,
            12,
            AddressingMode::ImplicitMode,
        ));
    }

    v
});

pub static OPCODE_MODE: LazyLock<[AddressingMode; 256]> = LazyLock::new(|| {
    let mut modes = [AddressingMode::ImplicitMode; 256];
    for op in OPCODES_8085.iter() {
        modes[op.code as usize] = op.mode;
    }
    modes
});
