// Memory
const MEMORY_SIZE: u16 = 0x1000;

const MEMORY_START: u16 = 0x000;
const MEMORY_END: u16 = 0x1FF;
const FONT_START: u16 = 0x050;
const FONT_END: u16 = 0x0A0;
const PROGRAM_START: u16 = 0x200;
const PROGRAM_END: u16 = 0xFFF;

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
    pub fn get_bytes(&mut self, starting_index: u16, number_of_bytes: u8) -> [u8] {
        let ending_index = starting_index + number_of_bytes as u16;
        return self.memory[starting_index..ending_index]
    }
    pub fn write(&mut self, location: u16, byte: u8) -> () {
        self.memory[location] = byte
    }
}
