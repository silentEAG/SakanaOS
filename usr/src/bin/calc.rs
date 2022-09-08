#![no_std]
#![no_main]

#[macro_use]
extern crate usr_lib;

#[no_mangle]
fn main() -> i32 {
    let a = 1000_u32;
    let b = 2000_u32;
    println!("a * b = {}", a * b);
    0
}
