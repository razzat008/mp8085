use bitflags::bitflags;

// flags available in the 8085 cpu
bitflags! {
    #[derive(Default,Debug)]
    pub struct CPUFlags: u8 {
        const SIGN_F   = 0b10000000;
        const ZERO_F   = 0b01000000;
        const AUXC_F   = 0b00010000;
        const PARITY_F = 0b00000100;
        const CARRY_F  = 0b00000001;
    }
}

// needs change, i don't know where it starts
const STACK_START: u16 = 0x00;

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
    pub mem: [u8; 0xFFFF],
}

// implementations for cpu
impl CPU {
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
            mem: [0; 0xFFFF],
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[derive(Debug)]
pub enum AddressingMode {
    ImmediateMode,
    RegisterMode,
    IndirectMode,
    DirectMode,
    // todo
}

#[cfg(test)]
mod test {
    use crate::CPU;

    #[test]
    fn init_cpu() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0;
        let cpu_reg = cpu.reg_a;
        assert_eq!(0, cpu_reg);
    }
}
