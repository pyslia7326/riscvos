use core::ptr;

const UART: usize = 0x10000000;
const UART_THR: *mut u8 = (UART + 0x00) as *mut u8;
const UART_LSR: *mut u8 = (UART + 0x05) as *mut u8;
const LSR_EMPTY_BIT: u8 = 5;
const UART_LSR_EMPTY: u8 = 1 << LSR_EMPTY_BIT;

pub fn print_char(c: char) {
    unsafe {
        while (*UART_LSR & UART_LSR_EMPTY) == 0 {}
        ptr::write_volatile(UART_THR, c as u8);
    }
}

pub fn print_string(s: &str) {
    for c in s.chars() {
        print_char(c);
    }
}

pub fn print_integer(num: u64) {
    let mut buffer = [0u8; 20];
    let mut i = 0;
    let mut n = num;

    if n == 0 {
        print_char('0');
        return;
    }

    while n > 0 {
        buffer[i] = (b'0' + (n % 10) as u8) as u8;
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        print_char(buffer[i] as char);
    }
}

pub fn print_integerln(num: u64) {
    print_integer(num);
    print_char('\n');
}
