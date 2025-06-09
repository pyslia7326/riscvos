use crate::syscall::{sys_read, sys_sleep, sys_wait, sys_write, sys_write_u64};
use crate::task::scheduler::task_create;
use crate::utils::cstr::cstr_to_str;
use crate::utils::list::LinkedList;
use core::ptr::copy_nonoverlapping;

pub fn shell(_argc: u64, _argv: &[&str]) {
    let buffer = [0u8; 128];
    let list = LinkedList::<i32>::new();
    sys_write("====================\n");
    let explain = "List commands:\n\
        p   print all\n\
        ih  insert head [ih <v>]\n\
        it  insert tail [it <v>]\n\
        ph  pop head\n\
        pt  pop tail\n\
        Type 'help' to see this message\n";
    sys_write(explain);
    sys_write("$ ");
    loop {
        if let Some(_read_len) = sys_read(&buffer) {
            match cstr_to_str(&buffer) {
                Ok(s) => {
                    let mut tokens = s.trim().split(' ');
                    let cmd = tokens.next().unwrap_or("");

                    let mut func = None;
                    match cmd {
                        "echo" => {
                            func = Some(echo);
                        }
                        "p" => {
                            for val in list.iter().unwrap() {
                                sys_write_u64(val.get_ref().lock().value.unwrap() as u64);
                                sys_write(" ");
                            }
                        }
                        "ih" => {
                            let cmd2 = tokens.next().unwrap_or("");
                            match cmd2.parse::<i32>() {
                                Ok(value) => {
                                    if list.push_front(value).is_none() {
                                        sys_write("push_front failed");
                                    } else {
                                        sys_write_u64(value as u64)
                                    }
                                }
                                Err(_) => sys_write("push_front: not a valid value"),
                            }
                        }
                        "it" => {
                            let cmd2 = tokens.next().unwrap_or("");
                            match cmd2.parse::<i32>() {
                                Ok(value) => {
                                    if list.push_back(value).is_none() {
                                        sys_write("push_back failed");
                                    } else {
                                        sys_write_u64(value as u64)
                                    }
                                }
                                Err(_) => sys_write("push_back: not a valid value"),
                            }
                        }
                        "ph" => match list.pop_front() {
                            Some(v) => sys_write_u64(v.get_ref().lock().value.unwrap() as u64),
                            None => sys_write("pop_front: list empty"),
                        },
                        "pt" => match list.pop_back() {
                            Some(v) => sys_write_u64(v.get_ref().lock().value.unwrap() as u64),
                            None => sys_write("pop_back: list empty"),
                        },
                        "help" => sys_write(explain),
                        _ => sys_write("Unknown command"),
                    }
                    if func.is_some() {
                        if let Some(ptr) = copy_to_heap(s) {
                            if let Some(task_struct) = task_create(echo, ptr, s.len()) {
                                sys_wait(task_struct.id.unwrap() as usize);
                                sys_write("Task execute success");
                            } else {
                                sys_write("Task create failed");
                            }
                        } else {
                            sys_write("Task create failed");
                        }
                    }
                    sys_write("\n$ ");
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
