all:
	cargo build

test:
	cargo build
	echo "Press Ctrl-A and then X to exit QEMU"
	qemu-system-riscv64 -nographic -smp 4 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/debug/riscvos

test_release:
	cargo build --release
	echo "Press Ctrl-A and then X to exit QEMU"
	qemu-system-riscv64 -nographic -smp 4 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/riscvos

debug:
	cargo build
	echo "Press Ctrl-A and then X to exit QEMU"
	qemu-system-riscv64 -nographic -smp 4 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/debug/riscvos -S -gdb tcp::61234

gdb:
	echo "Press Ctrl-Z to exit gdb"
	gdb-multiarch target/riscv64gc-unknown-none-elf/release/riscvos