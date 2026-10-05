use mp8085::{CPU, CPUFlags};

#[test]
fn test_mvi_and_halt() {
    let mut cpu = CPU::new();
    // MVI B, 0x50 ; HLT
    cpu.interpret(vec![0x06, 0x50, 0x76]);
    assert_eq!(cpu.reg_b, 0x50);
}

#[test]
fn test_mov() {
    let mut cpu = CPU::new();
    // MVI B, 5 ; MOV C, B ; HLT
    cpu.interpret(vec![0x06, 0x05, 0x48, 0x76]);
    assert_eq!(cpu.reg_b, 5);
    assert_eq!(cpu.reg_c, 5);
}

#[test]
fn test_add() {
    let mut cpu = CPU::new();
    // MVI A, 5 ; MVI B, 3 ; ADD B ; HLT
    cpu.interpret(vec![0x3E, 0x05, 0x06, 0x03, 0x80, 0x76]);
    assert_eq!(cpu.reg_a, 8);
    assert!(!cpu.status.contains(CPUFlags::CARRY_F));
    assert!(!cpu.status.contains(CPUFlags::ZERO_F));
}

#[test]
fn test_adi_carry_and_zero() {
    let mut cpu = CPU::new();
    // MVI A, 0xFF ; ADI 0x01 ; HLT
    cpu.interpret(vec![0x3E, 0xFF, 0xC6, 0x01, 0x76]);
    assert_eq!(cpu.reg_a, 0);
    assert!(cpu.status.contains(CPUFlags::ZERO_F));
    assert!(cpu.status.contains(CPUFlags::CARRY_F));
    assert!(cpu.status.contains(CPUFlags::PARITY_F)); // 0b0 has even parity
}

#[test]
fn test_sub() {
    let mut cpu = CPU::new();
    // MVI A, 10 ; SUI 4 ; HLT
    cpu.interpret(vec![0x3E, 0x0A, 0xD6, 0x04, 0x76]);
    assert_eq!(cpu.reg_a, 6);
    assert!(!cpu.status.contains(CPUFlags::CARRY_F));
}

#[test]
fn test_jmp() {
    let mut cpu = CPU::new();
    // MVI A, 1 ; JMP 0x0005 ; (0x03,0x04 padding NOP) ; MVI A, 2 ; HLT
    cpu.interpret(vec![0x3E, 0x01, 0xC3, 0x05, 0x00, 0x3E, 0x02, 0x76]);
    assert_eq!(cpu.reg_a, 2);
}

#[test]
fn test_call_ret() {
    let mut cpu = CPU::new();
    // CALL 0x0006 ; HLT ; NOP ; NOP ; MVI A, 9 ; RET
    cpu.interpret(vec![0xCD, 0x06, 0x00, 0x76, 0x00, 0x00, 0x3E, 0x09, 0xC9]);
    assert_eq!(cpu.reg_a, 9);
}

#[test]
fn test_push_pop() {
    let mut cpu = CPU::new();
    // MVI B, 0x12 ; MVI C, 0x34 ; PUSH B ; POP D ; HLT
    cpu.interpret(vec![0x06, 0x12, 0x0E, 0x34, 0xC5, 0xD1, 0x76]);
    assert_eq!(cpu.reg_d, 0x12);
    assert_eq!(cpu.reg_e, 0x34);
}

#[test]
fn test_lxi_ldax_stax() {
    let mut cpu = CPU::new();
    // LXI B, 0x1234 ; LXI H, 0x2000 ; MOV M, B? use STAX path:
    // LXI H, 0x2000 ; MVI A, 0x77 ; STAX? no. Use direct:
    // LXI B, 0x0010 ; LDA test via STA then LDAX DE
    cpu.interpret(vec![
        0x11, 0x10, 0x00, // LXI D, 0x0010
        0x3E, 0xAB,       // MVI A, 0xAB
        0x12,             // STAX D
        0x3E, 0x00,       // MVI A, 0
        0x1A,             // LDAX D
        0x76,
    ]);
    assert_eq!(cpu.reg_a, 0xAB);
}
