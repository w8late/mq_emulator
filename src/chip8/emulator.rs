pub struct Emulator {
    memory: [u8; 4096],     //4 kB of RAM
    pc: usize,                //program counter: current instruction in memory
    ir: u16,                //index register: locations in memory
    stack: Vec<u16>,        //stack for functions
    delay_timer: u8,        //timer decremented at 60 Hz until it reaches 0
    sound_timer: u8,        //same as delay_timer, gives of a beeping sound until 0
    registers: [u8; 16],    //variable registers
    display: [u32; 64 * 32], // 64 * 32 pixels to write to
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
            pc: 0,
            ir: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            display: [0; 64 * 32],
        }
    }

    pub fn fetch_decode(&self) {
        let mut instruction: u16;
        instruction = self.memory[self.pc] as u16;
        instruction <<= 8;
        instruction += self.memory[self.pc+1] as u16;
        match (instruction) {
            
            _  => return,
        }

        self.pc += 2;
    }
}