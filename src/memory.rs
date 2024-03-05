use std::vec;

pub fn grow_capacity(capacity: i32) -> i32 {
    if capacity < 8 {
        return 8
    }
    capacity * 2
}

pub fn grow_array<T: Default + Clone>(pointer: Option<Vec<T>>, old_count: i32, new_count: i32) -> Option<Vec<T>> {
    let result = reallocate(pointer, old_count, new_count);
    if new_count > 0 && result.is_none() {
        std::process::exit(1);
    }
    result
}

pub fn free_array<T: Default + Clone>(pointer: Option<Vec<T>>, old_count: i32) -> Option<Vec<T>> {
    reallocate(pointer, old_count, 0)
}

pub fn reallocate<T: Default + Clone>(pointer: Option<Vec<T>>, old_size: i32, new_size: i32) -> Option<Vec<T>> {
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
