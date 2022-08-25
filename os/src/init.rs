use log::{debug, SetLoggerError, LevelFilter, info};

use crate::io::console::SakanaLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    static LOGGER: SakanaLogger = SakanaLogger;
    log::set_logger(&LOGGER).map(|()| log::set_max_level(
        match option_env!("OS_LOG") {
            Some("ERROR") => LevelFilter::Error,
            Some("WARN") => LevelFilter::Warn,
            Some("INFO") => LevelFilter::Info,
            Some("DEBUG") => LevelFilter::Debug,
            Some("TRACE") => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }))
}

fn space_info() {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack(); // stack bottom
        fn boot_stack_top(); // stack top
    }
    debug!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    debug!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

fn build_info() {
    let kernel_name = env!("KERNEL_NAME");
    let kernel_version = env!("KERNEL_VERSION");
    let build_time = env!("BUILD_TIME");
    info!("{} v{} built at {}", kernel_name, kernel_version, build_time);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| unsafe {(p as *mut u8).write_volatile(0) });
}

pub fn sys_init() {
    // Clean bss data
    clear_bss();
    // Init logger system
    init_logger().unwrap();
    // Output space info
    space_info();
    // Output build info
    build_info();
}