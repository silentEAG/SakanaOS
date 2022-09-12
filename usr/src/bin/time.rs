#![no_std]
#![no_main]

use usr_lib::get_time;

#[macro_use]
extern crate usr_lib;

#[no_mangle]
fn main() -> i32 {
    let time = get_time();
    println!("time: {:?}", time);
    0
}
