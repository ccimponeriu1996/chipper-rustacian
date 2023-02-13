// Graphics
const PIXELS: usize = 64 * 32;

// Memory
const MEMORY_START: usize = 0x000;
const MEMORY_END: usize = 0x1FF;
const FONT_START: usize = 0x050;
const FONT_END: usize = 0x0A0;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;

struct ChipEight {
    opcode: u16, // TODO: do something with this

    memory: [u8; 4096],
    registers: [u8; 16],

    // Timer Registers
    sound_timer: u8,
    delay_timer: u8,

    index_register: u8,
    program_counter: u8,

    // Graphics
    graphics: [u8; PIXELS],

    // Stack
    stack: [u8; 16], // 16 levels of stack
    stack_pointer: u8,

    // Keypad
    keypad: [u8; 16],
}

impl ChipEight {
    fn new() -> ChipEight {
        ChipEight {
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            graphics: [0; PIXELS],
            stack: [0; 16],
            stack_pointer: 0,
            keypad: [0; 16],
        }
    }
}

impl Default for ChipEight {
    fn default() -> Self {
        Self::new()
    }
}

impl ChipEight {
    fn load(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[i + 0x200] = *byte;
        }
    }
}
