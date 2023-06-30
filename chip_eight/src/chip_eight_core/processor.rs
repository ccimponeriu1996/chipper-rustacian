use crate::common::hex_utils;

use super::api::Tick;


pub struct Processor {
    opcode: u16,
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            opcode: 0,
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
        }
    }
    pub fn get_opcode(&mut self) -> u16 {
        return self.opcode;
    }
    pub fn set_opcode(&mut self, opcode: u16) {
        self.opcode = opcode;
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
    pub fn call(&mut self) {
        // Store current address in the PC to the top of the stack and set the PC to the new addr
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = self.dehydrate_opcode();
    }
    // TODO: refactor these skip ifs into something magical if possible
    pub fn skip_on_equal(&mut self) {
        let opcode_data: u16 = self.dehydrate_opcode();
        let register_pointer: usize = hex_utils::right_shift(opcode_data, 2) as usize;
        let comparitor: u8 = hex_utils::left_pad(opcode_data, 2) as u8;
        if self.registers[register_pointer] == comparitor {
            self.skip_instruction();
        }
    }
    pub fn skip_on_not_equal(&mut self) {
        let opcode_data: u16 = self.dehydrate_opcode();
        let register_pointer: usize = hex_utils::right_shift(opcode_data, 2) as usize;
        let comparitor: u8 = hex_utils::left_pad(opcode_data, 2) as u8;
        if self.registers[register_pointer] != comparitor {
            self.skip_instruction();
        }
    }
    pub fn skip_if_registers(&mut self) {
        let opcode_data: u16 = self.dehydrate_opcode();
        let left_register_pointer: usize = hex_utils::right_shift(opcode_data, 3) as usize;
        let right_register_pointer: usize = hex_utils::right_shift(opcode_data, 2) as usize;
        if self.registers[left_register_pointer] == self.registers[right_register_pointer] {
            self.skip_instruction();
        }
    }
    pub fn load_into_register(&mut self) {
        let opcode_data: u16 = self.dehydrate_opcode();
    }
    // TODO: dehydrate the mains, nnn, xkk, and xy0..9. Also, come up with better names if possible.
    fn dehydrate_large_number(self) -> u16 {
        return self.dehydrate_opcode();
    }
    fn dehydrate_register_and_value() {

    }
    // TODO: look into definition of hydration and dehydration in software
    fn dehydrate_opcode(&mut self) -> u16 {
        return hex_utils::left_pad(self.opcode, 1);
    }
    fn skip_instruction(&mut self) {
        // Hypothetically adding two to the pc is supposed to skip the instruction but god knows.
        self.program_counter += 2;
    }
}

impl Tick for Processor {
    fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
