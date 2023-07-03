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

const F_REGISTER_POINTER: usize = 0xF;

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
    pub fn skip_instruction(&mut self) {
        self.program_counter += 2;
    }
    pub fn get_index_register_value(&mut self) -> u16 {
        return self.index_register;
    }
    pub fn get_register_value(&mut self, register_index: usize) -> u8 {
        return self.registers[register_index];
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
            self.skip_instruction();
        }
    }
    pub fn skip_on_not_equal(&mut self) {
        let (register_pointer, value) = self.dehydrate_register_and_value();
        if self.registers[register_pointer] != value {
            self.skip_instruction();
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
            self.skip_instruction();
        }
    }
    pub fn skip_if_not_regsiters(&mut self) {
        let (x_register, y_register) = self.dehydrate_registers();
        if self.registers[x_register] != self.registers[y_register] {
            self.skip_instruction();
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
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
