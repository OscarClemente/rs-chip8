use std::fs::File;
use std::io::prelude::*;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

#[derive(Debug)]
struct OpCode {
    pub ll: u8,
    pub lr: u8,
    pub rl: u8,
    pub rr: u8,
}

impl OpCode {
    pub fn new(lhs: u8, rhs: u8) -> Self {
        OpCode {
            ll: (lhs & 0xF0) >> 4,
            lr: lhs & 0x0F,
            rl: (rhs & 0xF0) >> 4,
            rr: rhs & 0x0F,
        }
    }
}

pub struct CPU {
    pc: usize,
    sp: usize,
    registers: [u8; 16],
    stack: [u16; 16],
    ram: [u8; 4096],
    vram: [[u8; 64]; 32],
    vram_changed: bool,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut ram = [0u8; 4096];
        // TODO: Fonts

        CPU {
            pc: 0x200,
            sp: 0,
            registers: [0; 16],
            stack: [0; 16],
            ram,
            vram: [[0; 64]; 32],
            vram_changed: false,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load(&mut self, filename: &str) {
        let mut f = File::open(filename).expect("File not found");
        f.read(&mut self.ram[0x200..]).unwrap();
    }

    pub fn cycle(&mut self) {
        self.run_opcode(&self.get_opcode());
    }

    fn get_opcode(&self) -> OpCode {
        OpCode::new(self.ram[self.pc], self.ram[self.pc + 1])
    }

    fn run_opcode(&mut self, opcode: &OpCode) {
        let pc_change = match (opcode.ll, opcode.lr, opcode.rl, opcode.rr) {
            (0x0, _,   _,   _  ) => self.execute_op_0nnn(opcode),
            (0x0, 0x0, _,   0x0) => self.execute_op_00e0(opcode),
            (0x0, 0x0, _,   _  ) => self.execute_op_00ee(opcode),
            (0x1, _,   _,   _  ) => self.execute_op_1nnn(opcode),
            (0x2, _,   _,   _  ) => self.execute_op_2nnn(opcode),
            (0x3, _,   _,   _  ) => self.execute_op_3xnn(opcode),
            (0x4, _,   _,   _  ) => self.execute_op_4xnn(opcode),
            (0x5, _,   _,   0x0) => self.execute_op_5xy0(opcode),
            (0x6, _,   _,   _  ) => self.execute_op_6xnn(opcode),
            (0x7, _,   _,   _  ) => self.execute_op_7xnn(opcode),
            (0x8, _,   _,   0x0) => self.execute_op_8xy0(opcode),
            (0x8, _,   _,   0x1) => self.execute_op_8xy1(opcode),
            (0x8, _,   _,   0x2) => self.execute_op_8xy2(opcode),
            (0x8, _,   _,   0x3) => self.execute_op_8xy3(opcode),
            (0x8, _,   _,   0x4) => self.execute_op_8xy4(opcode),
            (0x8, _,   _,   0x5) => self.execute_op_8xy5(opcode),
            (0x8, _,   _,   0x6) => self.execute_op_8xy6(opcode),
            (0x8, _,   _,   0x7) => self.execute_op_8xy7(opcode),
            (0x8, _,   _,   0xE) => self.execute_op_8xye(opcode),
            (0x9, _,   _,   0x0) => self.execute_op_9xy0(opcode),
            (0xA, _,   _,   _  ) => self.execute_op_annn(opcode),
            (0xB, _,   _,   _  ) => self.execute_op_bnnn(opcode),
            (0xC, _,   _,   _  ) => self.execute_op_cxnn(opcode),
            (0xD, _,   _,   _  ) => self.execute_op_dxyn(opcode),
            (0xE, _,   0x9, 0xE) => self.execute_op_ex9e(opcode),
            (0xE, _,   0xA, 0x1) => self.execute_op_exa1(opcode),
            (0xF, _,   0x0, 0x7) => self.execute_op_fx07(opcode),
            (0xF, _,   0x0, 0xA) => self.execute_op_fx0a(opcode),
            (0xF, _,   0x1, 0x5) => self.execute_op_fx15(opcode),
            (0xF, _,   0x1, 0x8) => self.execute_op_fx18(opcode),
            (0xF, _,   0x1, 0xE) => self.execute_op_fx1e(opcode),
            (0xF, _,   0x2, 0x9) => self.execute_op_fx29(opcode),
            (0xF, _,   0x3, 0x3) => self.execute_op_fx33(opcode),
            (0xF, _,   0x5, 0x5) => self.execute_op_fx55(opcode),
            (0xF, _,   0x6, 0x5) => self.execute_op_fx65(opcode),
            _ => ProgramCounter::Next,
        };

        self.pc += 2;
    }

    fn execute_op_0nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_00e0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_00ee(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_1nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_2nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_3xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_4xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_5xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_6xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_7xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy1(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy2(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy3(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy4(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy5(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy6(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xy7(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_8xye(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_9xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_annn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_bnnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_cxnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_dxyn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_ex9e(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }
    
    fn execute_op_exa1(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx07(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx0a(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx15(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx18(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx1e(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx29(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx33(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx55(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    fn execute_op_fx65(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }
}