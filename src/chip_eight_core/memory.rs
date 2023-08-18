// Memory
const MEMORY_SIZE: usize = 0x1000;

const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;
const MEMORY_START: usize= 0x000;
const MEMORY_END: usize = 0x1FF;

// I know there is another popular font start and end but I don't know why so I refuse.
const FONT_START: u16 = 0x000;
const FONT_END: u16 = 0x04B;

const ONE_BYTE: usize = 8;

const SPRITE_LOCATIONS: [u8; 16] = [
    0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75];

const SPRITE_DATA: [u8; 80] = [
    // Sprite data to be loaded into protected memory
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
];

pub struct Memory {
    memory: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        // TODO: Find a way to initialize rom data to memory immediately
        Memory {
            memory: [0; MEMORY_SIZE]
        }
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        assert!(SPRITE_DATA.len() < MEMORY_END); // Make sure the rom never goes past protected mem.
        for (i, byte) in SPRITE_DATA.iter().enumerate() {
            self.memory[i] = *byte;
        }
        assert!(program.len() < PROGRAM_END - PROGRAM_START); // Make sure program is within mem.
        for (i, byte) in program.iter().enumerate() {
            self.memory[PROGRAM_START + i] = *byte;
        }
        println!("loaded program");
    }
    pub fn get_opcode(&mut self, address: usize) -> u16 {
        let big_end: u16 = (self.memory[address] as u16) << ONE_BYTE; // Left shift one whole byte
        let little_end: u16 = self.memory[address + 1] as u16; // Store the next byte as u16
        return big_end + little_end; // Add together and return
    }
    pub fn get_bytes(&mut self, starting_index: u16, number_of_bytes: u8) -> Vec<u8> {
        let ending_index: u16 = starting_index + number_of_bytes as u16;
        return self.memory[starting_index as usize..ending_index as usize].to_vec();
    }
    pub fn set_bytes(&mut self, starting_index: usize, bytes: Vec<u8>) {
        let start: usize = starting_index - 1;
        for (i, byte) in bytes.iter().enumerate() {
            self.memory[start + i] = *byte;
        }
    }
    pub fn get_hex_sprite_index(&mut self, hex_digit: u8) -> u8 {
        return SPRITE_LOCATIONS[hex_digit as usize];
    }
}
