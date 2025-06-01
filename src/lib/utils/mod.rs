use core::num::ParseIntError;
use core::str::Utf8Error;

pub fn cstr_to_str(cstr: &[u8]) -> Result<&str, Utf8Error> {
    for (i, &c) in cstr.iter().enumerate() {
        if c == b'\0' {
            return core::str::from_utf8(&cstr[0..i]);
        }
    }
    core::str::from_utf8(&cstr)
}

pub fn cstr_to_u64(cstr: &[u8]) -> Result<u64, ParseIntError> {
    let s = cstr_to_str(cstr).map_err(|_| "invalid utf-8".parse::<u64>().unwrap_err())?;
    s.parse::<u64>()
}

pub fn u64_to_str<'a>(num: u64, buffer: &'a mut [u8; 20]) -> Result<&'a str, Utf8Error> {
    buffer[19] = b'0';
    let mut n = num;

    let mut str_start = 19;
    for i in (0..20).rev() {
        if n > 0 {
            buffer[i] = b'0' + (n % 10) as u8;
            n /= 10;
            str_start = i;
        } else {
            break;
        }
    }
    cstr_to_str(&buffer[str_start..20])
}
