use std::vec;

const OP_RETURN: u8 = 1;

struct Chunk {
    count: i32,
    capacity: i32,
    code: Option<Vec<u8>>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            count: 0,
            capacity: 0,
            code:None,
        }
    }

    fn init_chunk(&mut self) {
        self.count = 0;
        self.capacity = 0;
        self.code = None;
    }

    fn free_chunk(&mut self) {
        self.code = free_array(self.code.take(), self.capacity);
        self.init_chunk();
    }

    fn write_chunk(&mut self, byte: u8) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.code = Some(grow_array(self.code.take(), old_capacity, self.capacity).unwrap()); // Wrap the Vec<_> value in Some variant
        }
        self.code.as_mut().unwrap()[self.count as usize] = byte; 
        self.count += 1;
    }
}

fn grow_capacity(capacity: i32) -> i32 {
    if capacity < 8 {
        return 8
    }
    capacity * 2
}

fn grow_array<T: Default + Clone>(pointer: Option<Vec<T>>, old_count: i32, new_count: i32) -> Option<Vec<T>> {
    let result = reallocate(pointer, old_count, new_count);
    if new_count > 0 && result.is_none() {
        std::process::exit(1);
    }
    result
}

fn free_array<T: Default + Clone>(pointer: Option<Vec<T>>, old_count: i32) -> Option<Vec<T>> {
    reallocate(pointer, old_count, 0)
}

fn reallocate<T: Default + Clone>(pointer: Option<Vec<T>>, old_size: i32, new_size: i32) -> Option<Vec<T>> {
    if new_size == 0 {
        return None;
    }
    match pointer {
        None => {
            Some(vec![T::default(); new_size as usize])
        }
        Some(pter) => {
            let delta = new_size - old_size;
            Some([pter.clone(), vec![T::default(); delta as usize]].concat())
        }
    }
}

fn dissassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0; // Make offset mutable
    while offset < chunk.count {
        offset = dissassemble_instruction(chunk, offset);        
    }
}

fn dissassemble_instruction(chunk: &Chunk, offset: i32) -> i32 {
    print!("{:04} ", offset);
    let instruction = chunk.code.as_ref().unwrap()[offset as usize]; // Call .as_ref() to borrow the contents
    match instruction {
        OP_RETURN => simple_instruction("OP_RETURN", offset), // Convert offset to u8
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }, 
    }
}

fn simple_instruction(name: &str, offset: i32) -> i32 { // Change offset type to u8
    println!("{}", name);
    offset + 1 as i32 // Convert offset back to i32
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OP_RETURN);
    dissassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}
