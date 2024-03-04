use std::vec;

const OP_RETURN: u8 = 1;
const OP_CONSTANT: u8 = 2;


trait DynamicArray<T: Default + Clone> {
    fn get_count(&self) -> i32;
    fn set_count(&mut self, count: i32);

    fn get_capacity(&self) -> i32;
    fn set_capacity(&mut self, capacity: i32);

    fn get_storage(&self) -> Option<Vec<T>>;
    fn set_storage(&mut self, code: Option<Vec<T>>);

    fn init_darray(&mut self) {
        self.set_count(0);
        self.set_capacity(0);
        self.set_storage(None);
    }

    fn free_darray(&mut self) {
        let mut code = self.get_storage();
        let capacity = self.get_capacity();

        free_array(code.take(), capacity);
        self.init_darray();
    }

    fn write_darray(&mut self, byte: T) {
        // println!("byte = {:#?}", byte);
        let mut capacity = self.get_capacity();
        let count = self.get_count();
        let code = self.get_storage().clone();

        if capacity < count + 1 {
            let old_capacity = capacity;
            capacity = grow_capacity(old_capacity);
            self.set_capacity(capacity);
            self.set_storage(Some(grow_array(code, old_capacity, capacity).unwrap()));
        }
        self.get_storage().as_mut().unwrap()[count as usize] = byte; 
        self.set_count(count + 1);
    }
}

struct CodeArray {
    count: i32,
    capacity: i32,
    code: Option<Vec<u8>>,
}

impl CodeArray {
    fn new() -> CodeArray {
        CodeArray {
            count: 0,
            capacity: 0,
            code: None,
        }
    }
}

impl DynamicArray<u8> for CodeArray {
    fn get_count(&self) -> i32 {
        self.count
    }

    fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    fn get_capacity(&self) -> i32 {
        self.capacity
    }

    fn set_capacity(&mut self, capacity: i32) {
        self.capacity = capacity;
    }

    fn get_storage(&self) -> Option<Vec<u8>> {
        self.code.clone()
    }

    fn set_storage(&mut self, code: Option<Vec<u8>>) {
        self.code = code;
    }
}

struct ValueArray {
    count: i32,
    capacity: i32,
    values: Option<Vec<f64>>,
}

impl ValueArray {
    fn new() -> ValueArray {
        ValueArray {
            count: 0,
            capacity: 0,
            values: None,
        }
    }
}

impl DynamicArray<f64> for ValueArray {
    fn get_count(&self) -> i32 {
        self.count
    }

    fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    fn get_capacity(&self) -> i32 {
        self.capacity
    }

    fn set_capacity(&mut self, capacity: i32) {
        self.capacity = capacity;
    }

    fn get_storage(&self) -> Option<Vec<f64>> {
        self.values.clone()
    }

    fn set_storage(&mut self, values: Option<Vec<f64>>) {
        self.values = values;
    }
}


struct Chunk {
    code_chunk: CodeArray,
    constants: ValueArray,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            code_chunk: CodeArray::new(),
            constants: ValueArray::new(),
        }
    }

    fn init_chunk(&mut self) {
        self.code_chunk.init_darray();
        self.constants.init_darray();
    }

    fn free_chunk(&mut self) {
        self.code_chunk.free_darray();
        self.constants.free_darray();
        self.init_chunk();
    }

    fn write_chunk(&mut self, byte: u8) {
        self.code_chunk.write_darray(byte);
    }

    fn add_constant(&mut self, value: f64) -> i32 {
        self.constants.write_darray(value);
        self.constants.get_count() - 1
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
    while offset < chunk.code_chunk.get_count() {
        offset = dissassemble_instruction(chunk, offset);        
    }
}

fn dissassemble_instruction(chunk: &Chunk, offset: i32) -> i32 {
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

fn main() {
    let mut chunk = Chunk::new();
    // let constant = chunk.add_constant(1.2);
    // chunk.write_chunk(OP_CONSTANT);
    // chunk.write_chunk(constant as u8);
    println!("OP_RETURN = {}", OP_RETURN);
    chunk.write_chunk(OP_RETURN);
    println!("code_chunk = {:?}", chunk.code_chunk.get_storage());
    dissassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}
