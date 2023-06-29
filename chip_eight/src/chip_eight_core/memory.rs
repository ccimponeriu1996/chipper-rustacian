use super::api::Tick;

// Memory
const MEMORY_SIZE: usize = 4096;

const MEMORY_START: usize = 0x000;
const MEMORY_END: usize = 0x1FF;
const FONT_START: usize = 0x050;
const FONT_END: usize = 0x0A0;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;

pub struct Memory {
    memory: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE]
        }
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        println!("le file: ");
        for (i, byte) in program.iter().enumerate() {
            self.memory[PROGRAM_START + i] = *byte;
            print!("{}:{}, ", PROGRAM_START + i, byte);
        }
    }
    pub fn read(&mut self, location: usize) -> u8 {
        return self.memory[location];
    }
    pub fn write(&mut self, location: usize, byte: u8) -> () {
        self.memory[location] = byte
    }
}

impl Tick for Memory {
    fn tick(&mut self) -> () {
        return
    }
}
