#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod init;
mod lang_items;
mod sbi;
#[macro_use]
mod io;
mod batch;
mod sync;
mod syscall;
mod trap;
mod trace;

use core::arch::global_asm;
use log::info;

// use crate::batch::APP_MANAGER;

// use crate::sbi::shutdown;

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/link_app.S"));
global_asm!(include_str!("asm/trap.S"));
#[no_mangle]
pub fn sakana_main() {
    // System init
    init::sys_init();
    let message = "Hello, world!";
    info!("{}", message);
    trap::init();
    batch::init();
    batch::run_next_app();
    // shutdown();
}
