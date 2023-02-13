struct ChipEight {
    memory: [u8; 4096],
}

impl ChipEight {
    fn new() -> ChipEight {
        ChipEight {
            memory: [0; 4096],
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


fn main() {
    ChipEight chip_eight = ChipEight::new();
    println!("Hello, world!");
}
