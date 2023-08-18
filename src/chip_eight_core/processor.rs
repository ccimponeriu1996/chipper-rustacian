use crate::common::hex_utils;

use super::api::Tick;
use super::memory::Memory;
use super::graphics::Graphics;
use super::keypad::{Keypad, Position};
use super::sound::Sound;


pub struct Processor {
    // Internal registers
    opcode: u16,
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,

    // Bus to other components
    memory: Memory,
    graphics: Graphics,
    keypad: Keypad,
    sound: Sound,

    // Misc
    waiting: bool,
    key_register: usize,
}

const F_REGISTER_POINTER: usize = 0xF;

impl Processor {
    pub fn new(memory: Memory, graphics: Graphics, keypad: Keypad, sound: Sound) -> Processor {
        Processor {
            opcode: 0,
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,

            memory,
            graphics,
            keypad,
            sound,

            waiting: false,
            key_register: 0,
        }
    }
    pub fn init(&mut self, rom: &Vec<u8>) -> () {
        self.memory.load(&rom);
    }
    // Execute the current opcode
    fn execute(&mut self) -> () {
        return match self.opcode & 0xF000 {
            0x0000 => self.parse_end_function(),
            0x1000 => self.jump(),
            0x2000 => self.call(),
            0x3000 => self.skip_on_equal(),
            0x4000 => self.skip_on_not_equal(),
            0x5000 => self.skip_if_registers(),
            0x6000 => self.load_into_register(),
            0x7000 => self.add_to_register(),
            0x8000 => self.parse_register_operation(),
            0x9000 => self.skip_if_not_regsiters(),
            0xA000 => self.load_into_index_register(),
            0xB000 => self.jump_plus_v0(),
            0xC000 => self.assign_random(),
            0xD000 => {
                let starting_index: u16 = self.index_register;
                let (x_coordinate, y_coordinate) = self.get_opcode_register_values();

                let number_of_bytes: u8 = self.get_opcode_nibble();
                let bytes: Vec<u8> = self.memory.get_bytes(starting_index, number_of_bytes);
                self.graphics.draw(bytes, x_coordinate, y_coordinate);
            }
            0xE000 => self.parse_keyboard_operation(),
            0xF000 => self.parse_util_function(),
            _ => panic!("Invalide opcode for general parsing")
        };
    }
    fn parse_end_function(&mut self) -> () {
        if self.opcode == 0x00E0 {
            self.graphics.clear();
        } else if self.opcode == 0x00EE {
            self.subroutine_return();
        } else {
            panic!("Invalid subopcode for end function");
        }

    }
    fn parse_register_operation(&mut self) -> () {
        return match 0x000F & self.opcode {
            0x0 => self.copy_register(),
            0x1 => self.or_registers(),
            0x2 => self.and_registers(),
            0x3 => self.xor_registers(),
            0x4 => self.add_registers(),
            0x5 => self.sub_registers(),
            0x6 => self.div_by_two(),
            0x7 => self.sub_reverse_registers(),
            0xE => self.multiply_by_two(),
            _ => panic!("Invalid subopcode for register operation"),
        }
    }
    fn parse_keyboard_operation(&mut self) -> () {
        let subopcode = self.opcode & 0x00FF;
        if subopcode != 0x009E || subopcode != 0x00A1 {
            panic!("Invalid subopcode for keyboard operation");
        }

        let register_index: usize = hex_utils::right_shift(self.opcode & 0x0F00, 2) as usize;
        let key = self.registers[register_index];
        if (subopcode == 0x009E && self.keypad.get_key(key) == Position::DOWN) ||
                (subopcode == 0x00A1 &&self.keypad.get_key(key) == Position::UP) {
            self.increment_program_counter();
        }
    }
    fn parse_util_function(&mut self) -> () {
        let register_index: usize = hex_utils::right_shift(self.opcode & 0x0F00, 2) as usize;
        match self.opcode & 0x00FF {
            0x0007 => self.set_delay_timer_to_register(register_index),
            0x000A => self.wait(register_index),
            0x0015 => self.set_delay_timer(register_index),
            0x0018 => {
                let register_value: u8 = self.registers[register_index];
                self.sound.set_sound_timer(register_value);
            }
            0x001E => self.add_register_to_i(register_index),
            0x0029 => {
                let register_value: u8 = self.registers[register_index];
                let hex_location: u8 = self.memory.get_hex_sprite_index(register_value);
                self.set_index_register(hex_location as u16);
            }
            0x0033 => {
                let register_value: u8 = self.registers[register_index];
                let starting_index: usize = self.index_register as usize;
                let bcd: [u8; 3] = [
                    register_value / 100, // Mod not required, u8 cannot go past 255
                    register_value / 10 % 10,
                    register_value % 10, // Div not required, only need ones place
                ];
                self.memory.set_bytes(starting_index, bcd.to_vec());
            }
            0x0055 => {
                let bytes: Vec<u8> = self.get_values_upto_register(register_index);
                let starting_index: usize = self.index_register as usize;
                self.memory.set_bytes(starting_index, bytes);
            }
            0x0065 => {
                let starting_index: u16 = self.index_register;
                let bytes: Vec<u8> = self.memory.get_bytes(starting_index, register_index as u8);
                self.set_values_upto_register(bytes);
            }
            _ => panic!(),
        }
    }
    pub fn load(&mut self, program: &Vec<u8>) {
        self.memory.load(program);
    }
    pub fn wait(&mut self, register_index: usize) {
        self.key_register = register_index;
    }
    pub fn end_wait(&mut self, key: u8) {
        self.set_register_value(self.key_register, key);
    }
    fn increment_program_counter(&mut self) {
        self.program_counter += 2; // Two bytes per instruction
    }
    pub fn get_values_upto_register(&mut self, register_index: usize) -> Vec<u8> {
        return self.registers[0..=register_index].to_vec();
    }
    pub fn set_values_upto_register(&mut self, bytes: Vec<u8>) -> () {
        for (i, byte) in bytes.iter().enumerate() {
            self.registers[i] = *byte;
        }
    }
    pub fn set_register_value(&mut self, register_index: usize, value: u8) -> () {
        self.registers[register_index] = value;
    }
    pub fn set_index_register(&mut self, value: u16) -> () {
        self.index_register = value;
    }
    pub fn get_opcode_register_values(&mut self) -> (usize, usize) {
        return self.dehydrate_registers();
    }
    pub fn get_opcode_nibble(&mut self) -> u8 {
        return 0x000F & self.opcode as u8;
    }
    pub fn set_delay_timer_to_register(&mut self, register_index: usize) -> () {
        self.registers[register_index] = self.delay_timer;
    }
    pub fn set_delay_timer(&mut self, register_index: usize) -> () {
        self.delay_timer = self.registers[register_index];
    }
    pub fn subroutine_return(&mut self) {
        // Set the PC to the address at the top of the stack and decrement stack by one.
        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
    }
    pub fn jump(&mut self) {
        // Jump to the address of the last 12 bits of the opcode
        self.program_counter = self.dehydrate_opcode();
    }
    pub fn jump_plus_v0(&mut self) {
        // Jump to the address of the last 12 bits of the opcode
        self.program_counter = self.dehydrate_opcode() + self.registers[0] as u16;
    }
    pub fn assign_random(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        self.registers[register_pointer] = value & hex_utils::random_byte();
    }
    pub fn call(&mut self) {
        // Store current address in the PC to the top of the stack and set the PC to the new addr
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = self.dehydrate_opcode();
    }
    // TODO: refactor these skip ifs into something magical if possible
    pub fn skip_on_equal(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        if self.registers[register_pointer] == value {
            self.increment_program_counter();
        }
    }
    pub fn skip_on_not_equal(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        if self.registers[register_pointer] != value {
            self.increment_program_counter();
        }
    }
    pub fn load_into_register(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        self.registers[register_pointer] = value;
    }
    pub fn copy_register(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[x_register] = self.registers[y_register];
    }
    pub fn or_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[x_register] |= self.registers[y_register];
    }
    pub fn and_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[x_register] &= self.registers[y_register];
    }
    pub fn xor_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[x_register] ^= self.registers[y_register];
    }
    pub fn add_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        let sum: u16 = self.registers[x_register] as u16
            + self.registers[y_register] as u16;

        self.registers[F_REGISTER_POINTER] = if sum > 0x00FF {1} else {0};
        self.registers[x_register] = sum as u8;
    }
    pub fn add_register_to_i(&mut self, register_index: usize) {
        self.index_register += self.registers[register_index] as u16;
    }
    pub fn sub_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[F_REGISTER_POINTER] = 
            if self.registers[x_register] < self.registers[y_register]
                {1} else {0};

        self.registers[x_register] -= self.registers[y_register];
    }
    pub fn sub_reverse_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        self.registers[F_REGISTER_POINTER] = 
            if self.registers[x_register] > self.registers[y_register]
                {1} else {0};

        self.registers[x_register] = self.registers[y_register] -
            self.registers[x_register];
    }
    pub fn div_by_two(&mut self) {
        let (x_register, _y_register) = self.dehydrate_registers();
        self.registers[F_REGISTER_POINTER] = if 0b00000001 & self.registers[x_register] == 0x01
            {1} else {0};

        self.registers[x_register] /= 2;
    }
    pub fn multiply_by_two(&mut self) {
        let (x_register, _y_register) = self.dehydrate_registers();
        self.registers[F_REGISTER_POINTER] = if 0b10000000 & self.registers[x_register] == 0xF0
            {1} else {0};

        self.registers[x_register] *= 2;
    }
    pub fn load_into_index_register(&mut self) {
        self.index_register = self.dehydrate_opcode();
    }
    pub fn add_to_register(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        self.registers[register_pointer] += value;
    }
    pub fn skip_if_registers(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        if self.registers[x_register] == self.registers[y_register] {
            self.increment_program_counter();
        }
    }
    pub fn skip_if_not_regsiters(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        if self.registers[x_register] != self.registers[y_register] {
            self.increment_program_counter();
        }
    }
    // Dehydration BEGIN
    // TODO: dehydrate the mains, nnn, xkk, and xy0..9. Also, come up with better names if possible.
    fn dehydrate_opcode(&mut self) -> u16 {
        return hex_utils::left_pad(self.opcode, 1);
    }
    fn dehydrate_register_and_value(&mut self) -> (usize, u8) {
        let opcode_data: u16 = self.dehydrate_opcode();
        let register_pointer: usize = hex_utils::right_shift(opcode_data, 2) as usize;
        let value: u8 = hex_utils::left_pad(opcode_data, 2) as u8;
        return (register_pointer, value);
    }
    fn dehydrate_registers(&mut self) -> (usize, usize) {
        let opcode_data: u16 = self.dehydrate_opcode();
        let x_register: usize = hex_utils::right_shift(opcode_data, 3) as usize;
        let y_register: usize = hex_utils::right_shift(opcode_data, 2) as usize;
        return (x_register, y_register);
    }
    // Dehydration END
}

impl Tick for Processor {
    fn tick(&mut self) {
        self.opcode = self.memory.get_opcode(self.program_counter as usize);
        self.increment_program_counter();
        self.execute();
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
