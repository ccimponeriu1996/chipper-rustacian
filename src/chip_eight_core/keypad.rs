#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position { UP, DOWN }

#[derive(Clone, Copy)]
pub enum Key {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

pub struct Keypad {
    keys: [Position; 16],
}

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
