use core::arch::global_asm;

global_asm!(
    r#"
    .section .text._start
    .global _start

    .equ STACK_SIZE, 8192

    _start:
        csrr a0, mhartid
        bnez a0, park
        la   sp, stacks + STACK_SIZE
        j    main

    park:
        wfi
        j park

        .section .bss.stacks, "aw", @nobits
        .global stacks
    stacks:
        .skip STACK_SIZE
"#
);
