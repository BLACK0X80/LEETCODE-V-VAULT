impl Solution {
    pub fn my_pow(black_x: f64, black_n: i32) -> f64 {
        let mut black_res = 1.0;
        let (mut black_base, mut black_exp) = (black_x, black_n as i64);
        if black_exp < 0 { black_base = 1.0 / black_base; black_exp = -black_exp; }
        while black_exp > 0 {
            if black_exp % 2 == 1 { black_res *= black_base; }
            black_base *= black_base;
            black_exp /= 2;
        }
        black_res
    }
}
