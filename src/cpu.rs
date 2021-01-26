use std::fs::File;
use std::io::prelude::*;

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize),
}

const OPCODE_SIZE: usize = 2;

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

    pub fn get_nnn(&self) -> usize {
        let nnn: usize = ((self.lr as usize) << 8) | ((self.rl as usize) << 4) | (self.rr as usize);
        nnn
    }
}

pub struct CPU {
    pc: usize,
    sp: usize,
    registers: [u16; 16],
    stack: [usize; 16],
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
            (0x0,   _,   _,   _) => self.execute_op_0nnn(opcode),
            (0x0, 0x0,   _, 0x0) => self.execute_op_00e0(opcode),
            (0x0, 0x0,   _,   _) => self.execute_op_00ee(opcode),
            (0x1,   _,   _,   _) => self.execute_op_1nnn(opcode),
            (0x2,   _,   _,   _) => self.execute_op_2nnn(opcode),
            (0x3,   _,   _,   _) => self.execute_op_3xnn(opcode),
            (0x4,   _,   _,   _) => self.execute_op_4xnn(opcode),
            (0x5,   _,   _, 0x0) => self.execute_op_5xy0(opcode),
            (0x6,   _,   _,   _) => self.execute_op_6xnn(opcode),
            (0x7,   _,   _,   _) => self.execute_op_7xnn(opcode),
            (0x8,   _,   _, 0x0) => self.execute_op_8xy0(opcode),
            (0x8,   _,   _, 0x1) => self.execute_op_8xy1(opcode),
            (0x8,   _,   _, 0x2) => self.execute_op_8xy2(opcode),
            (0x8,   _,   _, 0x3) => self.execute_op_8xy3(opcode),
            (0x8,   _,   _, 0x4) => self.execute_op_8xy4(opcode),
            (0x8,   _,   _, 0x5) => self.execute_op_8xy5(opcode),
            (0x8,   _,   _, 0x6) => self.execute_op_8xy6(opcode),
            (0x8,   _,   _, 0x7) => self.execute_op_8xy7(opcode),
            (0x8,   _,   _, 0xE) => self.execute_op_8xye(opcode),
            (0x9,   _,   _, 0x0) => self.execute_op_9xy0(opcode),
            (0xA,   _,   _,   _) => self.execute_op_annn(opcode),
            (0xB,   _,   _,   _) => self.execute_op_bnnn(opcode),
            (0xC,   _,   _,   _) => self.execute_op_cxnn(opcode),
            (0xD,   _,   _,   _) => self.execute_op_dxyn(opcode),
            (0xE,   _, 0x9, 0xE) => self.execute_op_ex9e(opcode),
            (0xE,   _, 0xA, 0x1) => self.execute_op_exa1(opcode),
            (0xF,   _, 0x0, 0x7) => self.execute_op_fx07(opcode),
            (0xF,   _, 0x0, 0xA) => self.execute_op_fx0a(opcode),
            (0xF,   _, 0x1, 0x5) => self.execute_op_fx15(opcode),
            (0xF,   _, 0x1, 0x8) => self.execute_op_fx18(opcode),
            (0xF,   _, 0x1, 0xE) => self.execute_op_fx1e(opcode),
            (0xF,   _, 0x2, 0x9) => self.execute_op_fx29(opcode),
            (0xF,   _, 0x3, 0x3) => self.execute_op_fx33(opcode),
            (0xF,   _, 0x5, 0x5) => self.execute_op_fx55(opcode),
            (0xF,   _, 0x6, 0x5) => self.execute_op_fx65(opcode),
            _ => ProgramCounter::Next,
        };

        self.pc += OPCODE_SIZE;
    }
    
    /// Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
    fn execute_op_0nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        self.stack[self.sp] = self.pc + OPCODE_SIZE;
        self.sp += 1;
        ProgramCounter::Jump(opcode.get_nnn())
    }

    /// Clears the screen.
    fn execute_op_00e0(&mut self, opcode: &OpCode) -> ProgramCounter {
        //self.vram.iter_mut().for_each(|x| *x = 0);
        self.vram_changed = true;
        ProgramCounter::Next
    }

    /// Returns from a subroutine.
    fn execute_op_00ee(&mut self, opcode: &OpCode) -> ProgramCounter {
        self.sp -= 1;
        ProgramCounter::Jump(self.stack[self.sp])
    }

    /// Jumps to address NNN.
    fn execute_op_1nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Jump(opcode.get_nnn())
    }

    /// Calls subroutine at NNN.
    fn execute_op_2nnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        self.stack[self.sp] = self.pc + OPCODE_SIZE;
        self.sp += 1;
        ProgramCounter::Jump(opcode.get_nnn())
    }

    /// Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_3xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        let nn: u16 = (opcode.rl << 4 | opcode.rr) as u16;
        if u16::from(opcode.lr) == nn {
            return ProgramCounter::Skip;
        }
        ProgramCounter::Next
    }

    /// Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_4xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        let nn: u16 = (opcode.rl << 4 | opcode.rr) as u16;
        if u16::from(opcode.lr) != nn {
            return ProgramCounter::Skip;
        }
        ProgramCounter::Next
    }

    /// Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_5xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        if opcode.rl == opcode.rr {
            return ProgramCounter::Skip;
        }
        ProgramCounter::Next
    }

    /// Sets VX to NN.
    fn execute_op_6xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        let nn: u16 = (opcode.rl << 4 | opcode.rr) as u16;
        self.registers[opcode.lr as usize] = nn;
        ProgramCounter::Next
    }

    /// Adds NN to VX. (Carry flag is not changed)
    fn execute_op_7xnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to the value of VY.
    fn execute_op_8xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to VX or VY. (Bitwise OR operation)
    fn execute_op_8xy1(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to VX and VY. (Bitwise AND operation)
    fn execute_op_8xy2(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to VX xor VY.
    fn execute_op_8xy3(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    fn execute_op_8xy4(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    fn execute_op_8xy5(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
    fn execute_op_8xy6(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    fn execute_op_8xy7(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
    fn execute_op_8xye(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_9xy0(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets I to the address NNN.
    fn execute_op_annn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Jumps to the address NNN plus V0.
    fn execute_op_bnnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
    fn execute_op_cxnn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N+1 pixels.
    fn execute_op_dxyn(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_ex9e(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }
    
    /// Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)
    fn execute_op_exa1(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets VX to the value of the delay timer.
    fn execute_op_fx07(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
    fn execute_op_fx0a(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets the delay timer to VX.
    fn execute_op_fx15(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets the sound timer to VX.
    fn execute_op_fx18(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Adds VX to I. VF is not affected.
    fn execute_op_fx1e(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    fn execute_op_fx29(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
    fn execute_op_fx33(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Stores V0 to VX (including VX) in memory starting at address I.
    fn execute_op_fx55(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }

    /// Fills V0 to VX (including VX) with values from memory starting at address I.
    fn execute_op_fx65(&mut self, opcode: &OpCode) -> ProgramCounter {
        ProgramCounter::Next
    }
}