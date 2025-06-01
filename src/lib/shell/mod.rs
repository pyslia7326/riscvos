use crate::syscall::{sys_read, sys_sleep, sys_write};
use crate::utils::cstr_to_str;

pub fn shell(_argc: u64, _argv: *const *const u8) {
    let buffer = [0u8; 128];
    sys_write("$ ");
    loop {
        if let Some(_read_len) = sys_read(&buffer) {
            match cstr_to_str(&buffer) {
                Ok(s) => {
                    sys_write("Input: ");
                    sys_write(s);
                    sys_write("\n$ ");
                }
                Err(_) => sys_write("Input Error\n$ "),
            }
        } else {
        }
        sys_sleep(100);
    }
}
