use std::io::prelude::*;
use std::io;
use std::fs::File;

pub struct Chip8 {
    pub ram: [u8; 4096],
    pub vmem: [bool; 64 * 32],
    pub pc: usize,
    pub regs: [u8; 16],
    pub I: u16,
    pub stack: Vec<u16>,
    pub fonts: [u8; 80],
}

#[derive(Debug)]
pub enum Instruction {
    CLR,
    RTS,
    JUMP  {nnn: u16},
    CALL  {nnn: u16},
    SKE   {s: u8, nn: u8},
    SKNE  {s: u8, nn: u8},
    SKRE  {s: u8, t: u8},
    LOAD  {s: u8, nn: u8},
    ADD   {s: u8, nn: u8},
    LOADI {nnn: u16},
    DRAW  {x: u8, y: u8, n: u8},
    LDSPR {x: u8},
    AND {s: u8, t: u8},
    BCD {s: u8},
    READ {s: u8},
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            ram: [0; 4096],
            vmem: [false; 64 * 32],
            pc: 0x200,
            regs: [0; 16],
            I: 0x000,
            stack: Vec::new(),
            fonts: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],

        };

        chip8.ram[..80].copy_from_slice(&chip8.fonts);
        chip8
    }

    pub fn decode(opcode: u16) -> Option<Instruction> {
        match (opcode & 0xF000) >> 12 {
            0x0 => match opcode & 0x0FFF {
                0x00EE => Some(Instruction::RTS),
                0x00E0 => Some(Instruction::CLR),
                _ => None,
            }
            0x1 => Some(Instruction::JUMP  {nnn: opcode & 0x0FFF}),
            0x2 => Some(Instruction::CALL  {nnn: opcode & 0x0FFF}),
            0x3 => Some(Instruction::SKE   {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x4 => Some(Instruction::SKNE  {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x5 => Some(Instruction::SKRE  {s: ((opcode & 0x0F00) >> 8) as u8, t:  ((opcode & 0x00F0) >> 4) as u8}),
            0x6 => Some(Instruction::LOAD  {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x7 => Some(Instruction::ADD   {s: ((opcode & 0x0F00) >> 8) as u8, nn: (opcode & 0x00FF) as u8}),
            0x8 => match opcode & 0x000F {
                0x2 => Some(Instruction::AND {s: ((opcode & 0x0F00) >> 8) as u8, t: ((opcode & 0x00F0) >> 4) as u8}),
                _ => None,
            }
            0xA => Some(Instruction::LOADI {nnn: (opcode & 0x0FFF)}),
            0xD => Some(Instruction::DRAW  {x: ((opcode & 0x0F00) >> 8) as u8, y: ((opcode & 0x00F0) >> 4) as u8, n: (opcode & 0x000F) as u8}),
            0xF => match opcode & 0x00FF {
                0x29 => Some(Instruction::LDSPR {x: ((opcode & 0x0F00) >> 8) as u8}),
                0x33 => Some(Instruction::BCD {s: ((opcode & 0x0F00) >> 8) as u8}),
                0x65 => Some(Instruction::READ {s: ((opcode & 0x0F00) >> 8) as u8}),
                _ => None,
            }
            _ => None,
        }
    }

    pub fn execute(&mut self, instruction: Instruction)
    {
        match instruction {
            Instruction::CLR => self.vmem = [false; 64 * 32],
            Instruction::RTS => self.pc = self.stack.pop().unwrap() as usize,
            Instruction::JUMP {nnn} => self.pc = nnn as usize,
            Instruction::CALL {nnn} => {
                self.stack.push((self.pc) as u16);
                self.pc = nnn as usize;
            },
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
            },
            Instruction::AND {s, t} => {
                self.regs[s as usize] = self.regs[s as usize] & self.regs[t as usize];
            },
            Instruction::LOAD {s, nn} => self.regs[s as usize] = nn,
            Instruction::LOADI {nnn} => self.I = nnn,
            Instruction::DRAW {x, y, n} => {
                let x: usize = (self.regs[x as usize] - 1).into(); 
                let y: usize = (self.regs[y as usize] + 1).into();

                for j in 0..n {
                    for i in (0..8).rev() {
                        // Compare with 0 instead of casting to bool, sus lang
                        self.vmem[((x + (7 - i)) + 64 * (y + j as usize))] ^= ((self.ram[(self.I + j as u16) as usize] >> i) & 0x01) != 0;
                    }
;
                }

                for y in 0..32 {
                    for x in 0..64 {
                        print!("{}", self.vmem[x + 64 * y] as u8);
                    }
                    print!("\n");
                }

            },
            Instruction::LDSPR {x} => {
                let chr = self.regs[x as usize];
                self.I = (chr * 0x4) as u16;
            },
            Instruction::BCD {s} => {
                let x = self.regs[s as usize];

                for i in 0..3 {
                    self.ram[((self.I + 3) - i) as usize] = (x / (10_u8.pow((i).into()))) % 10;
                }
            },
            Instruction::READ {s} => {
                println!("FINISH THE READ INSTRUCTION");
            },
            _ => println!("This instruction has not been implemented"),
        };
    }

    pub fn fetch(&mut self) -> u16
    {
        let instruction: u16 = ((self.ram[self.pc] as u16) << 8) | self.ram[self.pc + 1] as u16;
        self.pc += 2;

        return instruction;
    }

    pub fn load_program(&mut self)
    {
        let mut file = File::open("GAMES/PONG").unwrap();
        file.read(&mut self.ram[0x200..]).unwrap();
    }

    pub fn cycle(&mut self)
    {
        let instruction = self.fetch();
        println!("{:X}", instruction);
        let decoded = Chip8::decode(instruction);

        match decoded {
            Some(i) => self.execute(i),
            None => println!("Ee instruction inikki ariyilla"),
        }

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
    }

}

