# Sakana-OS (experimental)

**Note: Update slowly**

Related to [rCore](http://rcore-os.cn/), learn OS with Rust and RISC-V.

## Plan 
* [] Finished all experiments in rCore
    * [x] ch1

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
[rustsbi] RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0
...(rustsbi logo)
[rustsbi] Implementation     : RustSBI-QEMU Version 0.1.1
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[DEBUG] .text [0x80200000, 0x80203000)
[DEBUG] .rodata [0x80203000, 0x80205000)
[DEBUG] .data [0x80205000, 0x80206000)
[DEBUG] boot_stack [0x80206000, 0x80216000)
[DEBUG] .bss [0x80216000, 0x80217000)
[INFO] sakana-os v0.1.0 built at Thu, 25 Aug 2022 16:36:37 +0800
[INFO] Hello, world!
[INFO] Shutdown...
```

## Reference
- RISC-V-Reader
- [rCore](http://rcore-os.cn/)
- [no_std book](https://docs.rust-embedded.org/book/intro/no-std.html)
- [MoeOS](https://github.com/KernelErr/MoeOS)
- [Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/)
- [The Adventures of OS](https://osblog.stephenmarz.com/index.html)