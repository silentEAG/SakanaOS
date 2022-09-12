use core::panic::PanicInfo;
use log::error;

use crate::{sbi::shutdown, trace::stack_trace};

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        error!(
            "Panicked at {}:{}:{}, {}",
            location.file(),
            location.line(),
            location.column(),
            panic_info.message().unwrap()
        );
    } else {
        error!("Paniced occurred: {}", panic_info.message().unwrap());
    }
    unsafe {
        stack_trace();
    }
    shutdown()
}
