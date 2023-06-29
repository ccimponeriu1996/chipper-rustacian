const BITS_IN_A_HEX: u16 = 0x0004;

// TODO: figure out how to add default values to variables
pub fn right_shift(opcode: u16, hex_places: u8) -> u16 {
    println!("{}", opcode);
    let mut i: u8 = 0;
    let mut left_shifted_opcode: u16 = opcode;
    while i < hex_places {
        left_shifted_opcode = left_shifted_opcode >> BITS_IN_A_HEX;
        i += 1;
        println!("{}", left_shifted_opcode);
    }
    return left_shifted_opcode;
}

pub fn left_pad(opcode: u16, hex_places: u8) -> u16 {
    let padding: u16 = right_shift(0x1000, hex_places - 1);
    return opcode % padding;
}
