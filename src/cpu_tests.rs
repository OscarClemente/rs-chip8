use super::*;

#[test]
fn test_load_rom() {
    let mut cpu = CPU::new();
    let mut data = [0u8; 3];
    data[0] = 11;
    data[1] = 22;
    data[2] = 33;
    cpu.load(&data);
    assert_eq!(cpu.ram[0x200], 11);
    assert_eq!(cpu.ram[0x201], 22);
    assert_eq!(cpu.ram[0x202], 33);
}

#[test]
fn test_get_opcode() {
    let mut cpu = CPU::new();
    cpu.ram[0x200] = 0xCA;
    cpu.ram[0x201] = 0xFE;

    let opcode = cpu.get_opcode();
    let expected_opcode = OpCode {
        ll: 0xC,
        lr: 0xA,
        rl: 0xF,
        rr: 0xE,
    };
    assert_eq!(opcode.ll, expected_opcode.ll);
    assert_eq!(opcode.lr, expected_opcode.lr);
    assert_eq!(opcode.rl, expected_opcode.rl);
    assert_eq!(opcode.rr, expected_opcode.rr);
}

#[test]
fn test_execute_op_0nnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x0,
        lr: 0x3,
        rl: 0x2,
        rr: 0x1,
    };
    let expected_program_counter = ProgramCounter::Jump(0x0321);

    let program_counter = cpu.execute_op_0nnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}