#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const SIZE: usize = 4096 * 3;

#[no_mangle]
fn main() -> i32 {
    let mut array = [0u8; SIZE];
    
    for i in 0..SIZE {
        array[i] = (i % 256) as u8;
    }
    
    for i in 0..SIZE {
        if array[i] != (i % 256) as u8 {
            println!("Memory error at position {}: expected {}, got {}", 
                     i, (i % 256) as u8, array[i]);
            return -1;
        }
    }
    
    println!("Memory boundary test passed!");
    0
}
