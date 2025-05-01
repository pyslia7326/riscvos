use super::csr;
use super::uart::{print_integerln, print_string};

pub fn trap_handler() {
    print_string("!!! TRAP OCCURRED !!!\n");
    let cause = csr::read_mcause();
    print_integerln(cause as u64);
    loop {}
}
