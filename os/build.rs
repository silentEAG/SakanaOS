use std::fs::{read_dir, File};
use std::io::{Result, Write};

use chrono::prelude::*;

static TARGET_PATH: &str = "../usr/target/riscv64gc-unknown-none-elf/release/";

fn main() -> Result<()> {
    let build_time = Utc::now().with_timezone(&chrono::Local).to_rfc2822();
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("(Unknown kernel name)");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("(Unknown kernel version)");
    let log = option_env!("OS_LOG").unwrap_or("TRACE");
    println!("cargo:rustc-env=KERNEL_NAME={}", name);
    println!("cargo:rustc-env=KERNEL_VERSION={}", version);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=OS_LOG={}", log);
    let mut f = File::create("src/asm/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../usr/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}
