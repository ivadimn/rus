use serde::{Serialize, Deserialize};
use serde_json;
use std::mem;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub unsafe fn struct_to_bytes<T>(val: &T) -> &[u8] {
    let size = mem::size_of::<T>();
    if size == 0 {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(val as *const T as *const u8, size) }
}

