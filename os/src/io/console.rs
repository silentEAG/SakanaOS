use crate::sbi::puts;
use core::fmt::Write;
use log::{Level, Metadata, Record};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        puts(s);
        Ok(())
    }
}

/// raw stdout impl
pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {
        $crate::io::console::print(format_args!($($args:tt)+));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::io::console::print(format_args!("\n"));
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::io::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// pub fn colour_stdout(colour_code: usize, s: &str) -> &str {
//     concat!("\u{1b}[", colour_code, "m", s, "\u{1b}[0m")
// }

/// Console log settings
pub struct SakanaLogger;

impl log::Log for SakanaLogger {
    /// Enable all info level by default
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    /// Make stdout format
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let colour_code = match record.level() {
                Level::Error => 31_usize,
                Level::Warn => 93_usize,
                Level::Info => 34_usize,
                Level::Debug => 32_usize,
                Level::Trace => 90_usize,
            };
            println!(
                "\u{1b}[{}m[{}] {}\u{1b}[0m",
                colour_code,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
