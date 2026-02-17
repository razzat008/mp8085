use crate::{AddressingMode, CPU, Mem};

#[allow(unused)]
impl CPU {
    fn get_operand(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::ImmediateMode => self.program_counter,

            AddressingMode::RegisterMode => {
                unimplemented!()
            }
            _ => todo!(),
        }
    }

    fn add(&self, mode: &AddressingMode) {
        let addr = self.get_operand(&mode);
        let value = self.mem_read(addr);

    }
}
