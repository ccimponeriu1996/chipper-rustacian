use super::memory::Memory;
use super::graphics::Graphics;
use super::sound::Sound;
use super::processor::Processor;
use super::keypad::Keypad;
use super::api::Tick;

pub struct ChipEight { processor: Processor }

impl ChipEight {
    pub fn new() -> ChipEight {
        ChipEight {
            processor: Processor::new(
                Memory::new(),
                Graphics::new(),
                Keypad::new(),
                Sound::new(),
            ),
        }
    }
    pub fn run(mut self, rom: &Vec<u8>) {
        self.processor.init(rom);
        loop { self.processor.tick(); }
    }
}
