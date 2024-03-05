

use crate::memory::{grow_capacity, grow_array, free_array};


pub const OP_RETURN: u8 = 1;
pub const OP_CONSTANT: u8 = 2;


pub trait DynamicArray<T: Default + Clone> {
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


pub struct CodeArray {
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


pub struct ValueArray {
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


pub struct Chunk {
    pub code_chunk: CodeArray,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code_chunk: CodeArray::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn init_chunk(&mut self) {
        self.code_chunk.init_darray();
        self.constants.init_darray();
    }

    pub fn free_chunk(&mut self) {
        self.code_chunk.free_darray();
        self.constants.free_darray();
        self.init_chunk();
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code_chunk.write_darray(byte);
    }

    pub fn add_constant(&mut self, value: f64) -> i32 {
        self.constants.write_darray(value);
        self.constants.get_count() - 1
    }
}

