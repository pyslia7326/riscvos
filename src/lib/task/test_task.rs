use crate::ecall;
use crate::uart::{print_integerln, print_string};

pub fn user_task1() {
    print_string("User Task 1 is running!\n");
    let mut cycle_count = 0;
    loop {
        print_string("User Task 1 count: ");
        print_integerln(cycle_count);
        ecall!();
        // Simulate some work
        let mut i = 0;
        while i < 50000000 {
            i += 1;
        }

        cycle_count += 1;
    }
}
pub fn user_task2() {
    print_string("User Task 2 is running!\n");
    let mut cycle_count = 0;
    loop {
        print_string("User Task 2 count: ");
        print_integerln(cycle_count);
        ecall!();
        // Simulate some work
        let mut i = 0;
        while i < 50000000 {
            i += 1;
        }
        cycle_count += 1;
    }
}
