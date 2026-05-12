impl Solution {
    pub fn possible_string_count(black_word: String, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_chars: Vec<char> = black_word.chars().collect();
        let black_n = black_chars.len();
        let black_k = black_k as usize;

        let mut black_groups = Vec::new();
        let mut black_i = 0;
        while black_i < black_n {
            let mut black_j = black_i;
            while black_j < black_n && black_chars[black_j] == black_chars[black_i] {
                black_j += 1;
            }
            black_groups.push((black_j - black_i) as i64);
            black_i = black_j;
        }

        let black_m = black_groups.len();
        if black_m >= black_k {
            let mut black_total_ways = 1i64;
            for black_g in black_groups {
                black_total_ways = (black_total_ways * black_g) % black_mod;
            }
            return black_total_ways as i32;
        }

        let mut black_dp = vec![0i64; black_k];
        black_dp[0] = 1;

        for black_g in black_groups.iter().map(|&x| x as usize) {
            let mut black_next_dp = vec![0i64; black_k];
            let mut black_prefix_sum = vec![0i64; black_k + 1];
            for black_idx in 0..black_k {
                black_prefix_sum[black_idx + 1] = (black_prefix_sum[black_idx] + black_dp[black_idx]) % black_mod;
            }

            for black_s in 0..black_k {
                let black_low = black_s.saturating_sub(black_g);
                let black_high = black_s.saturating_sub(1);
                if black_s > 0 {
                    let black_val = (black_prefix_sum[black_high + 1] - black_prefix_sum[black_low] + black_mod) % black_mod;
                    black_next_dp[black_s] = black_val;
                }
            }
            black_dp = black_next_dp;
        }

        let mut black_invalid_ways = 0i64;
        for black_v in black_dp {
            black_invalid_ways = (black_invalid_ways + black_v) % black_mod;
        }

        let mut black_total_all = 1i64;
        for black_g in black_groups {
            black_total_all = (black_total_all * black_g) % black_mod;
        }

        ((black_total_all - black_invalid_ways + black_mod) % black_mod) as i32
    }
}