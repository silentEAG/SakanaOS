use core::panic::PanicInfo;
use log::error;

use crate::sbi::shutdown;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = match panic_info.message() {
        Some(m) => m.as_str().unwrap_or("No more message provided."),
        None => "No more message provided.",
    };
    if let Some(location) = panic_info.location() {
        error!(
            "Panicked at {}:{}:{}, {}",
            location.file(),
            location.line(),
            location.column(),
            message
        );
    } else {
        error!("Paniced occurred: {}", message);
    }
    shutdown()
}
