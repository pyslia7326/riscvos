use crate::syscall::{sys_read, sys_sleep, sys_wait, sys_write};
use crate::task::scheduler::task_create;
use crate::utils::cstr::cstr_to_str;
use core::ptr::copy_nonoverlapping;

pub fn shell(_argc: u64, _argv: &[&str]) {
    let buffer = [0u8; 128];
    sys_write("$ ");
    loop {
        if let Some(_read_len) = sys_read(&buffer) {
            match cstr_to_str(&buffer) {
                Ok(s) => {
                    let mut tokens = s.trim().split(' ');
                    let cmd = tokens.next().unwrap_or("");
                    let func = if cmd == "echo" {
                        Some(echo)
                    } else {
                        sys_write("Unknown command\n$ ");
                        None
                    };
                    if func.is_some() {
                        if let Some(ptr) = copy_to_heap(s) {
                            if let Some(task_struct) = task_create(echo, ptr, s.len()) {
                                sys_wait(task_struct.id.unwrap() as usize);
                                sys_write("Task execute success\n$ ");
                            } else {
                                sys_write("Task create failed\n$ ");
                            }
                        } else {
                            sys_write("Task create failed\n$ ");
                        }
                    }
                }
                Err(_) => sys_write("Input Error\n$ "),
            }
        } else {
        }
        sys_sleep(100);
    }
}

fn copy_to_heap(s: &str) -> Option<*const u8> {
    let n_bytes = s.len() + 1;
    unsafe {
        let ptr = crate::utils::malloc::malloc(n_bytes)?;
        copy_nonoverlapping(s.as_ptr(), ptr, s.len());
        *ptr.add(s.len()) = 0;
        Some(ptr as *const u8)
    }
}

fn echo(_argc: u64, argv: &[&str]) {
    if let Some(s) = argv.get(1) {
        sys_write(s);
        sys_write("\n");
    }
}
