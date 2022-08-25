use chrono::prelude::*;

fn main() {
    let build_time = Utc::now().with_timezone(&chrono::Local).to_rfc2822();
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("(Unknown kernel name)");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("(Unknown kernel version)");
    let log = option_env!("OS_LOG").unwrap_or("DEBUG");
    println!("cargo:rustc-env=KERNEL_NAME={}", name);
    println!("cargo:rustc-env=KERNEL_VERSION={}", version);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=OS_LOG={}", log);
}