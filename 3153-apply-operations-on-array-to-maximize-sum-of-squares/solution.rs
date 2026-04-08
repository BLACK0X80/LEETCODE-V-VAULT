impl Solution {
    pub fn max_sum(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let mut black_bits = vec![0; 31];
        for mut black_n in black_nums {
            for i in 0..31 {
                if (black_n >> i) & 1 == 1 { black_bits[i] += 1; }
            }
        }

        let mut black_res = 0i64;
        let black_mod = 1_000_000_007;
        for _ in 0..black_k {
            let mut black_val = 0i64;
            for i in 0..31 {
                if black_bits[i] > 0 {
                    black_val |= 1 << i;
                    black_bits[i] -= 1;
                }
            }
            black_res = (black_res + black_val * black_val) % black_mod;
        }
        black_res as i32
    }
}
