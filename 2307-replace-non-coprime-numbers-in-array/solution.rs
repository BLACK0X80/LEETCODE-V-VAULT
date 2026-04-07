impl Solution {
    pub fn replace_non_coprimes(black1: Vec<i32>) -> Vec<i32> {
        let mut black2 = Vec::new();
        for mut x in black1 {
            while let Some(&last) = black2.last() {
                let g = Self::black_gcd(last, x);
                if g > 1 {
                    black2.pop();
                    x = ((last as i64 * x as i64) / g as i64) as i32;
                } else {
                    break;
                }
            }
            black2.push(x);
        }
        black2
    }

    fn black_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}
