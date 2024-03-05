use crate::chunk::DynamicArray;
use crate::chunk::{Chunk, OP_CONSTANT, OP_RETURN};


pub fn dissassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0; // Make offset mutable
    while offset < chunk.code_chunk.get_count() {
        offset = dissassemble_instruction(chunk, offset);        
    }
}

pub fn dissassemble_instruction(chunk: &Chunk, offset: i32) -> i32 {
    print!("{:04} ", offset);
    let instruction = chunk.code_chunk.get_storage().as_ref().unwrap()[offset as usize]; // Call .as_ref() to borrow the contents
    match instruction {
        OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset), // Pass chunk as a reference
        OP_RETURN => simple_instruction("OP_RETURN", offset), // Convert offset to u8
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }, 
    }
}

fn print_value(value: f64) {
    print!("{}", value);
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: i32) -> i32 { // Pass chunk as a reference
    let constant = chunk.code_chunk.get_storage().as_ref().unwrap()[(offset + 1) as usize]; // Call .as_ref() to borrow the contents
    print!("{:<16} {:4} ", name, constant);
    print_value(chunk.constants.get_storage().as_ref().unwrap()[constant as usize]); // Call .as_ref() to borrow the contents
    println!();
    offset + 2
}

fn simple_instruction(name: &str, offset: i32) -> i32 { // Change offset type to u8
    println!("{}", name);
    offset + 1 as i32 // Convert offset back to i32
}
