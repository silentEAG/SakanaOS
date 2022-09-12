# Sakana-OS (experimental)

**Note: Update slowly**

Related to [rCore](http://rcore-os.cn/), learn OS with Rust and RISC-V.

## Plan 
* [ ] Finished all experiments in rCore
    * [x] ch1
    * [x] ch2
    * [ ] ch3
* [ ] ?

(To be continue...)

## How to use

**Unfinished**

### Previous work

- Rust devlopment environment (ngintly).
- `qemu-system-riscv64` needed (7.0).

Wanting to get more setup infomation just view [rCore-setup-env](http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html).

### Run

Use `cd os && make run` to start.
```text
[KERNEL|DEBUG] .text [0x80200000, 0x80204000)
[KERNEL|DEBUG] .rodata [0x80204000, 0x8020c000)
[KERNEL|DEBUG] .data [0x8020c000, 0x80214000)
[KERNEL|DEBUG] boot_stack [0x80214000, 0x80224000)
[KERNEL|DEBUG] .bss [0x80224000, 0x80225000)
[KERNEL|INFO] sakana-os v0.1.0 built at Mon, 12 Sep 2022 15:22:15 +0800
[KERNEL|INFO] Hello, world!
[KERNEL|TRACE] num_app = 4
[KERNEL|TRACE] app_0 [0x8020c030, 0x8020d8d8)
[KERNEL|TRACE] app_1 [0x8020d8d8, 0x8020f1d0)
[KERNEL|TRACE] app_2 [0x8020f1d0, 0x80210dd8)
[KERNEL|TRACE] app_3 [0x80210dd8, 0x80212dc0)
[KERNEL|TRACE] Loading app_0
[KERNEL|INFO] PageFault in application, kernel killed it.
[KERNEL|TRACE] Loading app_1
Hello, SilentE!
[KERNEL|INFO] Application exited with code 0
[KERNEL|TRACE] Loading app_2
time: 49
[KERNEL|INFO] Application exited with code 0
[KERNEL|TRACE] Loading app_3
[KERNEL|ERROR] Unsupported fd in sys_write!
string from data section
strinstring from stack section
strin
Test write1 OK!
[KERNEL|INFO] Application exited with code 0
[KERNEL|ERROR] Panicked at src/batch/mod.rs:70:13, All applications completed!
==Stack Tracing Begin==
Return Address: 0x0000000080200bbc, fp: 0x0000000080206c50
Return Address: 0x0000000080201ca2, fp: 0x0000000080206ce0
Return Address: 0x00000000802007a2, fp: 0x0000000080206e10
Return Address: 0x00000000802016c6, fp: 0x0000000080206e70
Return Address: 0x0000000080200dec, fp: 0x0000000080206ef0
Return Address: 0x00000000802008cc, fp: 0x0000000080208fd0
Return Address: 0x00000000804002f4, fp: 0x0000000080208fe0
Return Address: 0x0000000080400094, fp: 0x0000000080209000
Return Address: 0x0000000000000000, fp: 0x0000000000000000
==Stack Tracing End==
[KERNEL|INFO] Shutdown...
```

## Reference
- RISC-V-Reader
- [rCore](http://rcore-os.cn/)
- [no_std book](https://docs.rust-embedded.org/book/intro/no-std.html)
- [MoeOS](https://github.com/KernelErr/MoeOS)
- [Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/)
- [The Adventures of OS](https://osblog.stephenmarz.com/index.html)
