impl Solution {
    pub fn super_pow(black_a: i32, black_b: Vec<i32>) -> i32 {
        black_b.iter().fold(1, |black_res, &black_digit| { let (black_a_m, black_m) = (black_a % 1337, 1337); let black_pow = |mut black_base: i32, mut black_exp: i32| { let mut black_p = 1; while black_exp > 0 { if black_exp % 2 == 1 { black_p = (black_p * black_base) % black_m; } black_base = (black_base * black_base) % black_m; black_exp /= 2; } black_p }; (black_pow(black_res, 10) * black_pow(black_a_m, black_digit)) % black_m })
    }
}