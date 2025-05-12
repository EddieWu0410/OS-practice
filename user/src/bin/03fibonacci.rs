#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Fibonacci sequence first 10 numbers:");
    
    let mut a = 0;
    let mut b = 1;
    
    // Print first number
    print!("{} ", a);
    
    // Print second number
    print!("{} ", b);
    
    // Calculate and print the next 8 numbers
    for _i in 2..10 {
        let next = a + b;
        print!("{} ", next);
        a = b;
        b = next;
    }
    
    println!("\\nFibonacci sequence calculation completed!");
    0
}

