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
