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

#[test]
fn test_execute_op_00e0() {
    let mut cpu = CPU::new();
    cpu.vram[0][0] = 0x1u8;
    cpu.vram[1][1] = 0x13u8;
    cpu.vram[10][10] = 0xFFu8;
    let opcode = OpCode {
        ll: 0x0,
        lr: 0x0,
        rl: 0xE,
        rr: 0x0,
    };
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_00e0(&opcode);

    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.vram[0][0], 0x0u8);
    assert_eq!(cpu.vram[1][1], 0x0u8);
    assert_eq!(cpu.vram[10][10], 0x0u8);
}

#[test]
fn test_execute_op_00ee() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x0,
        lr: 0x0,
        rl: 0xe,
        rr: 0xe,
    };
    cpu.sp = 1;
    cpu.stack[0] = 0x0321;
    let expected_program_counter = ProgramCounter::Jump(0x0321);

    let program_counter = cpu.execute_op_00ee(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_1nnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x1,
        lr: 0x4,
        rl: 0x5,
        rr: 0x6,
    };
    let expected_program_counter = ProgramCounter::Jump(0x0456);

    let program_counter = cpu.execute_op_1nnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_2nnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x2,
        lr: 0x3,
        rl: 0x2,
        rr: 0x1,
    };
    let expected_program_counter = ProgramCounter::Jump(0x0321);

    let program_counter = cpu.execute_op_2nnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_3xnn_skip() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x3,
        lr: 0x1,
        rl: 0x4,
        rr: 0x4,
    };
    cpu.registers[1] = 0x44u16;
    let expected_program_counter = ProgramCounter::Skip;

    let program_counter = cpu.execute_op_3xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_3xnn_next() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x3,
        lr: 0x1,
        rl: 0x4,
        rr: 0x4,
    };
    cpu.registers[1] = 0x40u16;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_3xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_4xnn_skip() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x4,
        lr: 0x1,
        rl: 0x4,
        rr: 0x4,
    };
    cpu.registers[1] = 0x40u16;
    let expected_program_counter = ProgramCounter::Skip;

    let program_counter = cpu.execute_op_4xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_4xnn_next() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x4,
        lr: 0x1,
        rl: 0x4,
        rr: 0x4,
    };
    cpu.registers[1] = 0x44u16;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_4xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_5xy0_skip() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x5,
        lr: 0x1,
        rl: 0x2,
        rr: 0x0,
    };
    cpu.registers[1] = 0x44u16;
    cpu.registers[2] = 0x44u16;
    let expected_program_counter = ProgramCounter::Skip;

    let program_counter = cpu.execute_op_5xy0(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_5xy0_next() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x5,
        lr: 0x1,
        rl: 0x2,
        rr: 0x0,
    };
    cpu.registers[1] = 0x44u16;
    cpu.registers[2] = 0x40u16;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_5xy0(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_6xnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x6,
        lr: 0x1,
        rl: 0x4,
        rr: 0x4,
    };
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_6xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0x44u16);
}

#[test]
fn test_execute_op_7xnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x7,
        lr: 0x1,
        rl: 0x1,
        rr: 0x2,
    };
    cpu.registers[1] = 0x05u16;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_7xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0x17u16);
}