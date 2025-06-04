# RISC-V OS Kernel (Learning Project)

This project is a simple RISC-V kernel written in Rust, intended for learning
and experimentation. It is designed to help beginners (including myself)
understand the basics of operating system development, RISC-V architecture,
and low-level programming in Rust.

## Features

* Minimal kernel written in Rust (`#![no_std]`, `#![no_main]`)
* Pure Rust kernel (no use of `extern "C"`, no C dependencies)
* UART serial output and input with interrupt-driven buffering
* Trap and interrupt handling (timer, external, syscall)
* Basic multitasking with a round-robin scheduler
* System call interface (yield, exit, sleep, read, write)
* Simple shell for user interaction

## Notes on PMP and Memory Alignment

* Physical Memory Protection (PMP) is configured to allow full access
  in supervisor mode. If PMP is not properly configured, any access from
  S-mode will cause a trap/exception.

* Trap vector setup (e.g., `mtvec`, `stvec`) requires proper function address
  alignment. Misaligned function pointers will lead to undefined behavior or
  failed trap handling. Ensure your `.cargo/config.toml` includes the following:

  ```toml
  [build]
  target = "riscv64gc-unknown-none-elf"

  [target.riscv64gc-unknown-none-elf]
  rustflags = [
    "-Clink-arg=-Tlinker.ld",
    "-Cllvm-args=-align-all-functions=2"
  ]
  ```

  This forces 4-byte alignment for all functions to satisfy architectural
  requirements on trap vector base addresses.

## Getting Started

### Prerequisites

* Rust toolchain
* Install required tools:

  ```sh
  rustup target add riscv64gc-unknown-none-elf
  cargo install cargo-binutils
  rustup component add llvm-tools
  ```
* QEMU for RISC-V (`qemu-system-riscv64`)
* Optionally: `gdb-multiarch` or [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain)

### Building

```sh
make
```

### Running in QEMU

```sh
make test
```

You should see the startup message and a shell prompt in your terminal.

### Debugging

To run with GDB support:

```sh
make debug
# In another terminal:
make gdb
```

## Project Structure

* `src/main.rs`: Kernel entry point and initialization
* `src/start.rs`: Startup code (sets up stack, jumps to `main`)
* `src/lib/`: Kernel modules (UART, scheduler, syscall, etc.)
* `src/lib/shell/`: Simple shell implementation
* `src/lib/task/`: Task management and scheduling
* `src/lib/trap/`: Trap and interrupt handling
* `src/lib/uart/`: UART driver
* `src/lib/syscall/`: System call definitions and handlers

## How It Works

1. The kernel starts in machine mode, sets up the stack, and jumps to `main`
2. Initializes UART, timer, and trap handlers
3. Sets up PMP for full memory access
4. Switches to supervisor mode and starts the scheduler
5. Launches a simple shell as the initial user task
6. Handles system calls and interrupts (timer, UART input)

## Why This Project?

* To learn and experiment with OS concepts on RISC-V
* To explore Rust's potential for low-level systems programming
* To provide a clean, hackable codebase for beginners

## TODO / Ideas

* Add memory allocator
* Support dynamic task creation from the shell
* Improve shell functionality (command parsing, extensibility)
* Add basic file system or storage support
* Update user task function signature to a more idiomatic Rust style:

  ```rust
  // From:
  type RawTaskFn = fn(argc: u64, argv: *const *const u8);

  // To:
  type RawTaskFn = fn(argc: u64, argv: &[&str]);
  ```

## References

* [QEMU RISC-V Documentation](https://wiki.qemu.org/Documentation/Platforms/RISCV)
* [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual)
