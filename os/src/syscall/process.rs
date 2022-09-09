// use crate::sbi::shutdown;
use crate::batch::run_next_app;
use log::info;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("Application exited with code {}", exit_code);
    run_next_app()
}
