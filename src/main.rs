pub mod utils;

use mp8085::CPU;

fn main() {
    let mut cpu = CPU::new();
    // MVI A, 5 ; MVI B, 3 ; ADD B ; HLT
    cpu.interpret(vec![0x3E, 0x80, 0x06, 0x80, 0x80, 0x27, 0x76]);
    println!(
        "A = {},B = {}, Mem= {}, (flags: {:?})",
        cpu.reg_a,
        cpu.reg_b,
        cpu.hl(),
        cpu.status
    );
}
