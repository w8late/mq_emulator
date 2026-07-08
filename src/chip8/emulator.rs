use macroquad::{color, input, shapes};
use crate::chip8::rom;
use crate::chip8::timer;
use crate::chip8::timer::CountdownTimer;

#[allow(dead_code)]
pub struct Emulator {
    memory: [u8; 4096],         //4 kB of RAM
    pc: usize,                  //program counter: current instruction in memory
    ir: u16,                    //index register: locations in memory
    fn_stack: Vec<u16>,         //stack for functions
    delay_timer: timer::CountdownTimer<60>,            //timer decremented at 60 Hz until it reaches 0
    sound_timer: timer::CountdownTimer<60>,            //same as delay_timer, gives of a beeping sound until 0
    registers: [u8; 16],        //variable registers
    display: [u32; 64 * 32],    // 64 * 32 pixels to write to
}

impl Emulator {
    const FLAG_REG: usize = 15; //the last variable register is also a flag register
    pub fn new() -> Self {
        let mut memory = [0; 4096];
        
        //load font into memory
        memory[0x50..=0x9f].copy_from_slice(&[
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ]);

        Self {
            memory,
            pc: 0x200,
            ir: 0,
            fn_stack: Vec::new(),
            delay_timer: CountdownTimer::with_thread(),
            sound_timer: CountdownTimer::with_thread(),
            registers: [0; 16],
            display: [0; 64 * 32],
        }
    }

    pub fn load_rom(&mut self, r: rom::ROMLoader) {
        // load rom into memory after the first 512 bytes
        self.memory[0x200..0x200+r.len].copy_from_slice(&r.bytes[0..r.len]);
    }

    // fetch - decode - execute
    pub fn fde(&mut self) {
        let mut instruction: u16;
        instruction = self.memory[self.pc] as u16;
        instruction <<= 8;
        instruction |= self.memory[self.pc+1] as u16;

        let x = instruction >> 8 & 15;
        let y = instruction >> 4 & 15;
        let n = instruction >> 0 & 15;
        let nn = (instruction & 255) as u8;
        let nnn = instruction & 4095;

        match instruction >> 12 {
            0x1 => self.pc = (nnn - 2) as usize, // 1nnn: jump
            0x2 => {
                self.fn_stack.push(self.pc as u16);
                self.pc = (nnn - 2) as usize;
            }, // 2nnn: subroutine (call)
            0x6 => self.registers[x as usize] = nn, // 6xnn: set
            0x7 => self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn), // 7xnn: add
            0xa => self.ir = nnn, // annn: set index
            0x3 => self.pc += (self.registers[x as usize] == nn) as usize * 2, // 3xnn: skip cond. (if equal)
            0x4 => self.pc += (self.registers[x as usize] != nn) as usize * 2, // 4xnn: skip cond. (if not equal)
            0x5 => self.pc += (self.registers[x as usize] == self.registers[y as usize]) as usize * 2, // 5xy0: skip cond. (if equal)
            0x9 => self.pc += (self.registers[x as usize] != self.registers[y as usize]) as usize * 2, // 9xy0: skip cond. (if not equal)
            0xc => self.registers[x as usize] = rand::random::<u8>() & nn, // cxnn: random
            0xd => { /* dxyn: display */ 
                
                self.registers[Self::FLAG_REG] = 0;  
                let (mut cx, mut cy): (usize, usize);
                cy = (self.registers[y as usize] % 32) as usize;

                for i in 0..n {
                    cx = (self.registers[x as usize] % 64) as usize;
                    let byte = self.memory[(self.ir + i) as usize];

                    for j in (0..8).rev() {
                        let bit = (byte >> j) & 1;

                        if bit == 1 && self.active_pixel(cx, cy) {
                            self.set_pixel(cx, cy, 0);
                            self.registers[Self::FLAG_REG] = 1;
                        } else if bit == 1 && !self.active_pixel(cx, cy) {
                            self.set_pixel(cx, cy, 0xffffffff);
                        }
                        cx += 1;
                        if cx == 63 { break; }
                    }
                    cy +=1;
                    if cy == 31 { break; }
                }
            },
          
            0xf => match instruction & 255 {
                0x1e => self.ir += self.registers[x as usize] as u16, //fx1e: add to index
                _ => (),
            },
            0x0 => match instruction & 255 { 
                0xe0 => self.display = [0; 64 * 32],  // 00e0: clear screen
                0xee => self.pc = self.fn_stack.pop().expect("popping empty stack") as usize, // 00ee: subroutine (pop)
                _ => (),
            },
            0xe => match instruction & 255 { 
                0x9e => if let Some(code) = get_key_code(x) { if input::is_key_down(code) { self.pc += 2 } } // ex9e: skip if key is down
                0xa1 => if let Some(code) = get_key_code(x) { if !input::is_key_down(code) { self.pc += 2 } } // exa1: skip if key is not down

                _ => (),
            },
            0x8 => match instruction & 16 {
                0x0 => self.registers[x as usize] = self.registers[y as usize], // 8xy0: set
                0x1 => self.registers[x as usize] |= self.registers[y as usize], // 8xy1: binary or
                0x2 => self.registers[x as usize] &= self.registers[y as usize], // 8xy2: binary and
                0x3 => self.registers[x as usize] ^= self.registers[y as usize], // 8xy3: binary xor
                0x4 => self.registers[x as usize] += self.registers[y as usize], // 8xy4: add
                0x5 =>  {  // 8xy5: subtract
                    self.registers[Self::FLAG_REG] = if self.registers[x as usize] >= self.registers[y as usize] { 1 } else { 0 };
                    self.registers[x as usize] -= self.registers[y as usize]
                },
                0x7 =>  {  // 8xy7: subtract
                    self.registers[Self::FLAG_REG] = if self.registers[y as usize] >= self.registers[x as usize] { 1 } else { 0 };
                    self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize]
                },
                _ => (),
            }
            _  => panic!("Instruction not implemented: {:x}", instruction),
        }

        self.pc += 2;
    } 

    fn active_pixel(&self, x: usize, y: usize) -> bool {
        self.display[y * 64 + x] & 255 != 0
    }

    fn set_pixel(&mut self, x: usize, y: usize, v: u32) {
        self.display[y * 64 + x] = v;
    }

    pub fn display(&self) {
        for x in 0..64 {
            for y in 0..32 {
                let p = self.display[y * 64 + x];
                shapes::draw_rectangle(x as f32 * 10.0, y as f32 * 10.0, 10.0, 10.0, color::Color {
                    r: (p >> 24 & 255) as f32 / 255.0,
                    g: (p >> 16 & 255) as f32 / 255.0,
                    b: (p >>  8 & 255) as f32 / 255.0,
                    a: (p >>  0 & 255) as f32 / 255.0,
                });
            }
        }
    }
}

const fn get_key_code(k: u16) -> Option<input::KeyCode> {
    use input::KeyCode::*;
    match k {
        0x1 => Some(Kp1),
        0x2 => Some(Kp2),
        0x3 => Some(Kp3),
        0xc => Some(Kp4),
        0x4 => Some(Q),
        0x5 => Some(W),
        0x6 => Some(E),
        0xd => Some(R),
        0x7 => Some(A),
        0x8 => Some(S),
        0x9 => Some(D),
        0xe => Some(F),
        0xa => Some(Z),
        0x0 => Some(X),
        0xb => Some(C),
        0xf => Some(V),
        _ => None
    }
}