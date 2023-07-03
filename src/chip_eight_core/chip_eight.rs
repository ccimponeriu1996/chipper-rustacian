use super::memory::Memory;
use super::graphics::Graphics;
use super::sound::Sound;
use super::processor::Processor;
use super::keypad::{Keypad, Position};
use super::api::Tick;

use crate::common::hex_utils;


pub struct ChipEight {
    memory: Memory,
    sound: Sound,
    graphics: Graphics,
    processor: Processor,
    keypad: Keypad,
    waiting: bool,
    key_register: usize,
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
            waiting: false,
            key_register: 0,
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
            0x6000..=0x6FFF => self.processor.load_into_register(),
            0x7000..=0x7FFF => self.processor.add_to_register(),
            0x8000..=0x8FFF => self.parse_register_operation(opcode),
            0x9000..=0x9FFF => self.processor.skip_if_not_regsiters(),
            0xA000..=0xAFFF => self.processor.load_into_index_register(),
            0xB000..=0xBFFF => self.processor.jump_plus_v0(),
            0xC000..=0xCFFF => self.processor.assign_random(),
            0xD000..=0xDFFF => {
                let starting_index: u16 = self.processor.get_index_register_value();
                let (x_coordinate, y_coordinate) = self.processor.get_opcode_register_values();

                let number_of_bytes: u8 = self.processor.get_opcode_nibble();
                let bytes: Vec<u8> = self.memory.get_bytes(starting_index, number_of_bytes);
                self.graphics.draw(bytes, x_coordinate, y_coordinate);
            }
            0xE000..=0xEFFF => self.parse_keyboard_operation(opcode),
            0xF000..=0xFFFF => self.parse_util_function(opcode),
            _ => panic!("Invalide opcode for general parsing")
        };
    }
    fn parse_register_operation(&mut self, opcode: u16) -> () {
        return match 0x000F & opcode {
            0x0 => self.processor.copy_register(),
            0x1 => self.processor.or_registers(),
            0x2 => self.processor.and_registers(),
            0x3 => self.processor.xor_registers(),
            0x4 => self.processor.add_registers(),
            0x5 => self.processor.sub_registers(),
            0x6 => self.processor.div_by_two(),
            0x7 => self.processor.sub_reverse_registers(),
            0xE => self.processor.multiply_by_two(),
            _ => panic!("Invalid subopcode for register operation"),
        }
    }
    fn parse_keyboard_operation(&mut self, opcode: u16) -> () {
        let subopcode = opcode & 0x00FF;
        if subopcode != 0x009E || subopcode != 0x00A1 {
            panic!("Invalid subopcode for keyboard operation");
        }

        let register_index: usize = hex_utils::right_shift(opcode & 0x0F00, 2) as usize;
        let key = self.processor.get_register_value(register_index);
        if (subopcode == 0x009E && self.keypad.get_key(key) == Position::DOWN) ||
                (subopcode == 0x00A1 &&self.keypad.get_key(key) == Position::UP) {
            self.processor.skip_instruction();
        }
    }
    fn parse_util_function(&mut self, subopcode: u16) -> () {
        let register_index: usize = hex_utils::right_shift(subopcode & 0x0F00, 2) as usize;
        match subopcode & 0x00FF {
            0x0007 => self.processor.set_delay_timer_to_register(register_index),
            0x000A => self.wait(register_index),
            0x0015 => self.processor.set_delay_timer(register_index),
            0x0018 => {
                let register_value: u8 = self.processor.get_register_value(register_index);
                self.sound.set_sound_timer(register_value);
            }
            _ => todo!(),
        }
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory.load(program);
    }
    pub fn wait(&mut self, register_index: usize) {
        self.key_register = register_index;
    }
    pub fn end_wait(&mut self, key: u8) {
        self.processor.set_register_value(self.key_register, key);
    }
    pub fn cycle(mut self) {
        self.processor.tick();
        self.sound.tick();
    }
}
