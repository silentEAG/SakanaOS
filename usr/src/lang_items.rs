#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    let message = match panic_info.message() {
        Some(m) => m.as_str().unwrap_or("No more message provided."),
        None => "No more message provided.",
    };
    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}:{}, {}",
            location.file(),
            location.line(),
            location.column(),
            message
        );
    } else {
        println!("Paniced occurred: {}", message);
    }
    loop {}
}
