#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod init;
mod lang_items;
mod sbi;
#[macro_use]
mod io;

use core::arch::global_asm;
use log::info;

use crate::sbi::shutdown;

global_asm!(include_str!("asm/boot.asm"));
#[no_mangle]
pub fn sakana_main() {
    // System init
    init::sys_init();

    let message = "Hello, world!";
    info!("{}", message);
    shutdown();
}
