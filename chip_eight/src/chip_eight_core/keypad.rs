pub struct Keypad {
    keys: [u8; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [0; 16],
        }
    }
    pub fn set_key(&mut self, key: u8, value: u8) {
        self.keys[key as usize] = value;
    }
    pub fn get_key(&self, key: u8) -> u8 {
        self.keys[key as usize]
    }
}
