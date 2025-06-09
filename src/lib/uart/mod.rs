use crate::mutex::Lock;
use crate::mutex::SpinLock;
use core::ptr;

const UART: usize = 0x10000000;
const UART_THR: *mut u8 = (UART + 0b000) as *mut u8;
const UART_RHR: *mut u8 = (UART + 0b000) as *mut u8;
const UART_IER: *mut u8 = (UART + 0b001) as *mut u8;
const _UART_ISR: *mut u8 = (UART + 0b010) as *mut u8;
const _UART_LCR: *mut u8 = (UART + 0b011) as *mut u8;
const UART_LSR: *mut u8 = (UART + 0b101) as *mut u8;
const LSR_THR_EMPTY_BIT: u8 = 5;
const UART_THR_EMPTY: u8 = 1 << LSR_THR_EMPTY_BIT;
const IER_DATA_READY_BIT: u8 = 0;
const UART_RX_ENABLE: u8 = 1 << IER_DATA_READY_BIT;
const LSR_DATA_READY_BIT: u8 = 0;
const UART_DATA_READY: u8 = 1 << LSR_DATA_READY_BIT;

const UART_WRITE_BUFFER_SIZE: usize = 256;
static mut UART_WRITE_BUFFER: [u8; UART_WRITE_BUFFER_SIZE] = [0; UART_WRITE_BUFFER_SIZE];
static mut UART_WRITE_HEAD: usize = 0;
static mut UART_WRITE_TAIL: usize = 0;
static UART_WRITE_LOCK: SpinLock = SpinLock::new();

const UART_READ_BUFFER_SIZE: usize = 256;
static mut UART_READ_BUFFER: [u8; UART_READ_BUFFER_SIZE] = [0; UART_READ_BUFFER_SIZE];
static mut UART_READ_HEAD: usize = 0;
static mut UART_READ_TAIL: usize = 0;
static mut UART_NEWLINE_CNT: isize = 0;
static UART_READ_LOCK: SpinLock = SpinLock::new();

macro_rules! write_reg {
    ($addr: expr, $value: expr) => {
        unsafe { ptr::write_volatile($addr, $value) }
    };
}

pub fn uart_init() {
    write_reg!(UART_IER, UART_RX_ENABLE);
}

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

pub fn uart_read(buffer: *mut u8, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    UART_READ_LOCK.lock();
    let mut read_len = 0;
    let mut newline = false;
    unsafe {
        if UART_NEWLINE_CNT <= 0 {
            UART_READ_LOCK.unlock();
            return None;
        }

        while UART_READ_HEAD != UART_READ_TAIL {
            let byte = UART_READ_BUFFER[UART_READ_HEAD];
            UART_READ_HEAD = (UART_READ_HEAD + 1) % UART_READ_BUFFER_SIZE;
            if read_len + 1 < len {
                if byte == b'\n' {
                    *buffer.add(read_len) = b'\0';
                    read_len += 1;
                    newline = true;
                    break;
                } else {
                    *buffer.add(read_len) = byte;
                    read_len += 1;
                }
            } else {
                *buffer.add(len - 1) = b'\0';
                read_len = len;
                if byte == b'\n' {
                    newline = true;
                    break;
                }
            }
        }
        if newline {
            UART_NEWLINE_CNT -= 1;
        }
    }

    UART_READ_LOCK.unlock();
    Some(read_len)
}

pub fn uart_irq_handler() {
    UART_READ_LOCK.lock();
    unsafe {
        while *UART_LSR & UART_DATA_READY != 0 {
            let byte = ptr::read_volatile(UART_RHR);
            let mut store_byte = byte;
            let mut is_newline = false;

            if byte == b'\r' || byte == b'\n' {
                store_byte = b'\n';
                is_newline = true;
            }

            let next_tail = (UART_READ_TAIL + 1) % UART_READ_BUFFER_SIZE;
            if next_tail != UART_READ_HEAD {
                UART_READ_BUFFER[UART_READ_TAIL] = store_byte;
                UART_READ_TAIL = next_tail;
                if is_newline {
                    UART_NEWLINE_CNT += 1;
                }
                print_char(store_byte as char);
            }
        }
    }
    UART_READ_LOCK.unlock();
}

pub fn print_char(c: char) {
    unsafe {
        while (*UART_LSR & UART_THR_EMPTY) == 0 {}
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
