impl Solution {
    pub fn sum_of_powers(mut black_nums: Vec<i32>, black_k: i32) -> i32 {
        black_nums.sort_unstable();
        let black_n = black_nums.len();
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![vec![std::collections::HashMap::new(); black_k as usize + 1]; black_n];

        let bravexuneth = &black_nums;

        for black_i in 0..black_n {
            black_dp[black_i][1].insert(i32::MAX, 1);
            for black_j in 0..black_i {
                let black_diff = bravexuneth[black_i] - bravexuneth[black_j];
                for black_len in 2..=black_k as usize {
                    let black_prev_map = black_dp[black_j][black_len - 1].clone();
                    for (&black_min_d, &black_count) in black_prev_map.iter() {
                        let black_new_min = black_min_d.min(black_diff);
                        let black_entry = black_dp[black_i][black_len].entry(black_new_min).or_insert(0);
                        *black_entry = (*black_entry + black_count) % black_mod;
                    }
                }
            }
        }

        let mut black_res = 0i64;
        for black_i in 0..black_n {
            for (&black_d, &black_c) in black_dp[black_i][black_k as usize].iter() {
                black_res = (black_res + black_d as i64 * black_c as i64) % black_mod as i64;
            }
        }
        black_res as i32
    }
}
