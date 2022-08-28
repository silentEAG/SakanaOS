#![no_std]
#![no_main]

#[macro_use]
extern crate usr_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world! from U mod!");
    0
}