// use crate::sbi::shutdown;
use crate::{batch::run_next_app, timer::get_time_us};
use log::info;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}
