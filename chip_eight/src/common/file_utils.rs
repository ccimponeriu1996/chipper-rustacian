use std::{fs::File, io::{Read, Error}, vec::Vec};


pub fn read_in_rom(path: &str) -> Result<Vec<u8>, Error> {
    println!("Path to rom file: {}", path);
    let result = File::open(path);
    let result: Result<File, Error> = result;
    let mut file: File = result?;

    let mut test_vector: Vec<u8> = Vec::new();
    println!("Resulting String of Garbage:\n{:?}", file.read_to_end(&mut test_vector));
    return Ok(test_vector);
}

