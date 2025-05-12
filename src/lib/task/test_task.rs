use crate::uart::{print_integerln, print_string};

pub fn user_task1(_argc: u64, _argv: *const *const u8) {
    print_string("User Task 1 is running!\n");
    for c in 0..=5 {
        print_string("User Task 1 count: ");
        print_integerln(c);
        // Simulate some work
        let mut i = 0;
        while i < 50000000 {
            i += 1;
        }
    }
}
pub fn user_task2(_argc: u64, _argv: *const *const u8) {
    print_string("User Task 2 is running!\n");
    for c in 0..=10 {
        print_string("User Task 2 count: ");
        print_integerln(c);
        // Simulate some work
        let mut i = 0;
        while i < 50000000 {
            i += 1;
        }
    }
}
