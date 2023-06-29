use super::memory::Memory;
use super::graphics::Graphics;
use super::sound::Sound;
use super::processor::Processor;
use super::keypad::Keypad;
use super::api::Tick;


pub struct ChipEight {
    memory: Memory,
    sound: Sound,
    graphics: Graphics,
    processor: Processor,
    keypad: Keypad,
}

// TODO: If i could go back in time i would call this the bus
impl ChipEight {
    pub fn new() -> ChipEight {
        ChipEight {
            memory: Memory::new(),
            sound: Sound::new(),
            graphics: Graphics::new(),
            processor: Processor::new(),
            keypad: Keypad::new(),
        }
    }
    // Maybe this should go in the chip_eight.rs file instead.
    fn parse_opcode(&mut self) -> () {
        let opcode: u16 = self.processor.get_opcode();
        return match opcode {
            // Any garbage i say, i am reading the bits left to right.
            0x00E0 => self.graphics.clear(),
            0x00EE => self.processor.subroutine_return(),
            0x1000..=0x1FFF => self.processor.jump(),
            0x2000..=0x2FFF => self.processor.call(),
            0x3000..=0x3FFF => self.processor.skip_on_equal(),
            0x4000..=0x4FFF => self.processor.skip_on_not_equal(),
            0x5000..=0x5FFF => self.processor.skip_if_registers(),
            0x6000..=0x6FFF => (),
            0x7000..=0x7FFF => (),
            0x8000..=0x8FFF => self.parse_register_operation(opcode),
            0x9000..=0x9FFF => (),
            0xA000..=0xAFFF => (),
            0xB000..=0xBFFF => (),
            0xC000..=0xCFFF => (),
            0xD000..=0xDFFF => (),
            0xE000..=0xEFFF => (),
            0xF000..=0xFFFF => self.parse_util_function(opcode),
            _ => todo!(),
        };
    }
    fn parse_register_operation(&mut self, subopcode: u16) -> () {
        return match subopcode % 0x0010 {
            0x0001 => (),
            _ => todo!(),
        }
    }
    fn parse_keyboard_operation(&mut self, subopcode: u16) -> () {
        if subopcode % 0x0100 == 0x009E {
            return ();
        } else {
            return ();
        }
    }
    fn parse_util_function(&mut self, subopcode: u16) -> () {
        match subopcode % 0x0100 {
            0x0007 => (),
            0x000A => (),
            0x0015 => (),
            0x0018 => (),
            _ => todo!(),
        }
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory.load(program);
    }
    pub fn cycle(mut self) {
        self.processor.tick();
        self.sound.tick();
    }
}
