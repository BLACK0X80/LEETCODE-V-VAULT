impl Solution {
    pub fn max_nice_divisors(black1: i32) -> i32 {
        if black1 <= 3 { return black1; }
        let black2 = 1_000_000_007i64;
        match black1 % 3 {
            0 => Self::black_pow(3, (black1 / 3) as i64, black2) as i32,
            1 => (Self::black_pow(3, (black1 / 3 - 1) as i64, black2) * 4 % black2) as i32,
            _ => (Self::black_pow(3, (black1 / 3) as i64, black2) * 2 % black2) as i32,
        }
    }

    fn black_pow(mut b: i64, mut e: i64, m: i64) -> i64 {
        let mut r = 1;
        b %= m;
        while e > 0 {
            if e % 2 == 1 { r = (r * b) % m; }
            b = (b * b) % m;
            e /= 2;
        }
        r
    }
}
