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
    cpu.registers[1] = 0x44;
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
    cpu.registers[1] = 0x40;
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
    cpu.registers[1] = 0x40;
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
    cpu.registers[1] = 0x44;
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
    cpu.registers[1] = 0x44;
    cpu.registers[2] = 0x44;
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
    cpu.registers[1] = 0x44;
    cpu.registers[2] = 0x40;
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
    assert_eq!(cpu.registers[1], 0x44);
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
    cpu.registers[1] = 0x05;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_7xnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0x17);
}

#[test]
fn test_execute_op_8xy0() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x0,
    };
    cpu.registers[1] = 0x0;
    cpu.registers[2] = 0x44;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy0(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0x44);
    assert_eq!(cpu.registers[2], 0x44);
}

#[test]
fn test_execute_op_8xy1() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x1,
    };
    cpu.registers[1] = 0b11010100;
    cpu.registers[2] = 0b01100110;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy1(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0b11110110);
    assert_eq!(cpu.registers[2], 0b01100110);
}

#[test]
fn test_execute_op_8xy2() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x2,
    };
    cpu.registers[1] = 0b11010100;
    cpu.registers[2] = 0b01100110;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy2(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0b01000100);
    assert_eq!(cpu.registers[2], 0b01100110);
}

#[test]
fn test_execute_op_8xy3() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x3,
    };
    cpu.registers[1] = 0b11010100;
    cpu.registers[2] = 0b01100110;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy3(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0b10110010);
    assert_eq!(cpu.registers[2], 0b01100110);
}

#[test]
fn test_execute_op_8xy4_no_carry() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x4,
    };
    cpu.registers[1] = 120;
    cpu.registers[2] = 103;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy4(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 223);
    assert_eq!(cpu.registers[2], 103);
    assert_eq!(cpu.registers[15], 0);
}

#[test]
fn test_execute_op_8xy4_carry() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x4,
    };
    cpu.registers[1] = 200;
    cpu.registers[2] = 200;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy4(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 144);
    assert_eq!(cpu.registers[2], 200);
    assert_eq!(cpu.registers[15], 1);
}

#[test]
fn test_execute_op_8xy5_no_borrow() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x5,
    };
    cpu.registers[1] = 50;
    cpu.registers[2] = 20;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy5(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 30);
    assert_eq!(cpu.registers[2], 20);
    assert_eq!(cpu.registers[15], 1);
}

#[test]
fn test_execute_op_8xy5_borrow() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x5,
    };
    cpu.registers[1] = 100;
    cpu.registers[2] = 200;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy5(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 156);
    assert_eq!(cpu.registers[2], 200);
    assert_eq!(cpu.registers[15], 0);
}

#[test]
fn test_execute_op_8xy6() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x6,
    };
    cpu.registers[1] = 0b01011101;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy6(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0b00101110);
    assert_eq!(cpu.registers[15], 1);
}

#[test]
fn test_execute_op_8xy7_no_borrow() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x7,
    };
    cpu.registers[1] = 101;
    cpu.registers[2] = 155;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy7(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 54);
    assert_eq!(cpu.registers[2], 155);
    assert_eq!(cpu.registers[15], 1);
}

#[test]
fn test_execute_op_8xy7_borrow() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0x7,
    };
    cpu.registers[1] = 155;
    cpu.registers[2] = 101;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xy7(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 202);
    assert_eq!(cpu.registers[2], 101);
    assert_eq!(cpu.registers[15], 0);
}

#[test]
fn test_execute_op_8xye() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x8,
        lr: 0x1,
        rl: 0x2,
        rr: 0xe,
    };
    cpu.registers[1] = 0b11011101;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_8xye(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.registers[1], 0b10111010);
    assert_eq!(cpu.registers[15], 1);
}

#[test]
fn test_execute_op_9xy0_next() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x9,
        lr: 0x1,
        rl: 0x2,
        rr: 0x0,
    };
    cpu.registers[1] = 23;
    cpu.registers[2] = 23;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_9xy0(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_9xy0_skip() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0x9,
        lr: 0x1,
        rl: 0x2,
        rr: 0x0,
    };
    cpu.registers[1] = 25;
    cpu.registers[2] = 23;
    let expected_program_counter = ProgramCounter::Skip;

    let program_counter = cpu.execute_op_9xy0(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_annn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0xa,
        lr: 0x1,
        rl: 0x2,
        rr: 0x3,
    };
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_annn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.index, 0x123);
}

#[test]
fn test_execute_op_bnnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0xb,
        lr: 0x1,
        rl: 0x2,
        rr: 0x3,
    };
    cpu.registers[0] = 0x12;
    let expected_program_counter = ProgramCounter::Jump(0x135);

    let program_counter = cpu.execute_op_bnnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_cxnn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0xc,
        lr: 0x1,
        rl: 0x2,
        rr: 0x3,
    };
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_cxnn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
}

#[test]
fn test_execute_op_dxyn() {
    let mut cpu = CPU::new();
    let opcode = OpCode {
        ll: 0xd,
        lr: 0x1,
        rl: 0x2,
        rr: 0x1,
    };
    cpu.registers[1] = 1;
    cpu.registers[2] = 1;
    cpu.index = 0x200;
    cpu.ram[0x200] = 0xFF;
    let expected_program_counter = ProgramCounter::Next;

    let program_counter = cpu.execute_op_dxyn(&opcode);
    
    assert_eq!(program_counter, expected_program_counter);
    assert_eq!(cpu.vram[0][0], 0);
    assert_eq!(cpu.vram[1][1], 1);
    assert_eq!(cpu.vram[1][8], 1);
    assert_eq!(cpu.vram[1][9], 0);
}