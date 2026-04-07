impl Solution {
    pub fn min_operations(mut black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let mut black3 = black2[0];
        for &x in &black2 {
            black3 = Self::black_gcd(black3, x);
        }
        black1.sort();
        for i in 0..black1.len() {
            if black3 % black1[i] == 0 {
                return i as i32;
            }
        }
        -1
    }

    fn black_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}
