#![no_std]
#![no_main]

#[macro_use]
extern crate usr_lib;

#[no_mangle]
fn main() -> i32 {
    extern "C" {
        fn start_bss();
    }
    println!("{:?}", start_bss as usize as *const u8);
    0
}
