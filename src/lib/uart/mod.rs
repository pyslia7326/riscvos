use crate::mutex::Mutex;
use crate::mutex::SpinLock;
use core::ptr;

const UART: usize = 0x10000000;
const UART_THR: *mut u8 = (UART + 0x00) as *mut u8;
const UART_LSR: *mut u8 = (UART + 0x05) as *mut u8;
const LSR_EMPTY_BIT: u8 = 5;
const UART_LSR_EMPTY: u8 = 1 << LSR_EMPTY_BIT;

const UART_WRITE_BUFFER_SIZE: usize = 256;
static mut UART_WRITE_BUFFER: [u8; UART_WRITE_BUFFER_SIZE] = [0; UART_WRITE_BUFFER_SIZE];
static mut UART_WRITE_HEAD: usize = 0;
static mut UART_WRITE_TAIL: usize = 0;
static UART_WRITE_LOCK: SpinLock = SpinLock::new();

// Write data into the UART buffer.
// If the buffer is full, new data will be dropped.
pub fn uart_write(ptr: *const u8, len: usize) {
    UART_WRITE_LOCK.lock();
    for i in 0..len {
        unsafe {
            let next_tail = (UART_WRITE_TAIL + 1) % UART_WRITE_BUFFER_SIZE;
            // If buffer is full, stop writing
            if next_tail == UART_WRITE_HEAD {
                break;
            }
            UART_WRITE_BUFFER[UART_WRITE_TAIL] = *ptr.add(i);
            UART_WRITE_TAIL = next_tail;
        }
    }
    UART_WRITE_LOCK.unlock();
}

// Flush the UART buffer: send all buffered data to UART hardware.
pub fn uart_write_buffer_flush() {
    UART_WRITE_LOCK.lock();
    unsafe {
        while UART_WRITE_HEAD != UART_WRITE_TAIL {
            print_char(UART_WRITE_BUFFER[UART_WRITE_HEAD] as char);
            UART_WRITE_HEAD = (UART_WRITE_HEAD + 1) % UART_WRITE_BUFFER_SIZE;
        }
    }
    UART_WRITE_LOCK.unlock();
}

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
