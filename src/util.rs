#[allow(unused)]
#[inline]
pub fn parse_initial_digits(digits: &[u8]) -> (i64, usize) {
    let mut len = 0;
    let mut res: i64 = 0;
    let mut negative = false;
    for &digit in digits {
        match digit {
            b'-' => negative = true,
            b'0'..=b'9' => {
                res *= 10;
                res += (digit - b'0') as i64;
            }
            _ => break,
        }
        len += 1;
    }
    if negative {
        res = -res;
    };
    (res, len)
}

#[inline]
pub fn parse_initial_digits_unsigned_i16(digits: &[u8]) -> (i16, usize) {
    let mut len = 0;
    let mut res: i16 = 0;
    for &digit in digits {
        match digit {
            b'0'..=b'9' => {
                res *= 10;
                res += (digit - b'0') as i16;
            }
            _ => break,
        }
        len += 1;
    }
    (res, len)
}

#[inline]
pub fn parse_initial_digits_unsigned_u32(digits: &[u8]) -> (u32, usize) {
    let mut len = 0;
    let mut res: u32 = 0;
    for &digit in digits {
        match digit {
            b'0'..=b'9' => {
                res *= 10;
                res += (digit - b'0') as u32;
            }
            _ => break,
        }
        len += 1;
    }
    (res, len)
}

#[inline]
pub fn parse_initial_digits_unsigned_u64(digits: &[u8]) -> (u64, usize) {
    let mut len = 0;
    let mut res: u64 = 0;
    for &digit in digits {
        match digit {
            b'0'..=b'9' => {
                res *= 10;
                res += (digit - b'0') as u64;
            }
            _ => break,
        }
        len += 1;
    }
    (res, len)
}

pub fn first_line_length(input: &[u8]) -> usize {
    input
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap_or(input.len())
}
