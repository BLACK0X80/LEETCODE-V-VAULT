impl Solution {
    pub fn count_orders(black_n: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_res = 1i64;
        for i in 1..=black_n as i64 {
            black_res = (black_res * i) % black_mod;
            black_res = (black_res * (2 * i - 1)) % black_mod;
        }
        black_res as i32
    }
}