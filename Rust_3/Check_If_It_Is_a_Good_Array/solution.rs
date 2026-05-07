impl Solution {
    pub fn is_good_array(black1: Vec<i32>) -> bool {
        let mut black2 = black1[0];
        for x in black1 {
            black2 = Self::black_gcd(black2, x);
            if black2 == 1 { return true; }
        }
        black2 == 1
    }

    fn black_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}