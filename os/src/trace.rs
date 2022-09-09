use core::arch::asm;

use crate::println;

pub unsafe fn stack_trace() {
    let mut fp : *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("==Stack Tracing Begin==");
    while !fp.is_null() {
        let ra = *fp.sub(1);
        let rp = *fp.sub(2);
        println!("Return Address: 0x{:016x}, fp: 0x{:016x}", ra, rp);
        fp = rp as * const usize;
    }
    println!("==Stack Tracing End==");
}