mod core;
mod opcodes;

use bitflags::bitflags;

// flags available in the 8085 cpu
bitflags! {
    #[derive(Default,Debug,Clone,Copy)]
    pub struct CPUFlags: u8 {
        const SIGN_F   = 0b10000000;
        const ZERO_F   = 0b01000000;
        const AUXC_F   = 0b00010000;
        const PARITY_F = 0b00000100;
        const CARRY_F  = 0b00000001;
    }
}

const STACK_START: u16 = 0xFFFF;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CPU {
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_h: u8,
    pub reg_l: u8,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub status: CPUFlags,
    pub mem: [u8; 0x10000],
    pub io: [u8; 256],
    pub int_enable: bool,
    pub halted: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

// implementations for cpu
#[allow(unused)]
impl CPU {
    // lets fuckin destroy everything
    pub fn new() -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            program_counter: 0,
            stack_pointer: STACK_START,
            status: CPUFlags::default(),
            mem: [0; 0x10000],
            io: [0; 256],
            int_enable: false,
            halted: false,
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Load a program into memory at address 0, reset state, then run until HLT.
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.reset();
        let n = program.len().min(self.mem.len());
        self.mem[..n].copy_from_slice(&program[..n]);
        self.run();
    }

    /// Fetch → decode → execute loop. Stops on HLT.
    pub fn run(&mut self) {
        while !self.halted {
            let op = self.fetch();
            self.execute(op);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    ImmediateMode,        //operand included in the instruction
    RegisterMode,         // operates on CPU registers
    RegisterIndirectMode, //addresses held in register pair(HL)
    ImplicitMode,         // no operand needed
    DirectMode,           // 16-bit memory address specified
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
