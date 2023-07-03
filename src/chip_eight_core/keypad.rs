pub struct Keypad {
    keys: [u8; 16],
}

pub enum Position { UP, DOWN };

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [Position::UP; 16],
        }
    }
    pub fn set_key(&mut self, key: u8, position: Position) {
        self.keys[key as usize] = position;
    }
    pub fn get_key(&self, key: u8) -> Position {
        return self.keys[key as usize]
    }
}
