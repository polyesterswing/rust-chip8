struct Chip8 {
    ram: [u8; 4096],
    pc: u16,
    regs: [u8; 16],
    I: u16,
    stack: [u8; 16],
}

#[derive(Debug)]
enum Instruction {
    JUMP {nnn: u16},
    CALL {nnn: u16},
    SKE {s: u8, nn: u8},
    SKNE {s: u8, nn: u8},
    SKRE {s: u8, t: u8},
    LOAD {s: u8, nn: u8},
    ADD {s: u8, nn: u8},
}

impl Chip8 {
    fn decode(opcode: u16) -> Option<Instruction> {
        match (opcode & 0xF000) >> 12 {
            0x1 => Some(Instruction::JUMP {nnn: opcode & 0x0FFF}),
            0x2 => Some(Instruction::CALL {nnn: opcode & 0x0FFF}),
            0x3 => Some(Instruction::SKE  {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x3 => Some(Instruction::SKNE {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x4 => Some(Instruction::SKRE {s: ((opcode & 0x0F00) >> 8) as u8, t:  ((opcode & 0x00F0) >> 4) as u8}),
            0x6 => Some(Instruction::LOAD {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x7 => Some(Instruction::ADD  {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            _ => None,
        }
    }

    fn execute(&mut self, instruction: Instruction)
    {
        match instruction {
            Instruction::JUMP {nnn} => self.pc = nnn,
            Instruction::SKE {s, nn} => {
                if self.regs[s as usize] == nn {
                    self.pc += 0x2;
                }
            },
            Instruction::SKNE {s, nn} => {
                if self.regs[s as usize] != nn {
                    self.pc += 0x2;
                }
            },
            Instruction::SKRE {s, t} => {
                if self.regs[s as usize] == self.regs[t as usize] {
                    self.pc += 0x2;
                }
            },
            Instruction::ADD {s, nn} => {
                self.regs[s as usize] += nn
            }
            Instruction::LOAD {s, nn} => self.regs[s as usize] = nn,
            _ => unimplemented!("This instruction has not been implemented"),
        };
    }
}

fn main() {
    let mut chip8 = Chip8 {
        ram: [0; 4096],
        pc: 0x200,
        regs: [0; 16],
        I: 0x000,
        stack: [0; 16],
    };

    match Chip8::decode(0x60FF) {
        Some(i) => chip8.execute(i),
        None => println!("Unknown Opcode"),
    };

    println!("{:X}", chip8.regs[0x0]);
}
