use crate::exit;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}:{}, {}",
            location.file(),
            location.line(),
            location.column(),
            panic_info.message().unwrap()
        );
    } else {
        println!("Paniced occurred: {}", panic_info.message().unwrap());
    }
    exit(-1)
}
