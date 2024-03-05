pub mod chunk;
pub mod debug;
pub mod memory;

use crate::chunk::DynamicArray;
use crate::chunk::Chunk;
use crate::chunk::{OP_CONSTANT, OP_RETURN};
use crate::debug::dissassemble_chunk;

pub fn run() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OP_CONSTANT);
    chunk.write_chunk(constant as u8);
    println!("OP_RETURN = {}", OP_RETURN);
    chunk.write_chunk(OP_RETURN);
    println!("code_chunk = {:?}", chunk.code_chunk.get_storage());
    dissassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}