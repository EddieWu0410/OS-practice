#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const SHARED_SIZE: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let mut shared_data = [0u32; SHARED_SIZE];
    
    for i in 0..SHARED_SIZE {
        shared_data[i] = i as u32;
    }
    
    for i in 0..SHARED_SIZE {
        shared_data[i] = shared_data[i].wrapping_mul(2);
    }
    
    for i in 0..SHARED_SIZE {
        if shared_data[i] != (i as u32).wrapping_mul(2) {
            println!("Data error at position {}: expected {}, got {}", 
                     i, (i as u32).wrapping_mul(2), shared_data[i]);
            return -1;
        }
    }
    
    println!("Task collaboration test passed!");
    0
}
