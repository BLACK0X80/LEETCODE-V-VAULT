impl Solution {
    pub fn num_ways(black_words: Vec<String>, black_target: String) -> i32 {
        let black_m = black_target.len();
        let black_n = black_words[0].len();
        let black_mod = 1_000_000_007;

        let mut black_char_count = vec![vec![0i64; 26]; black_n];
        for black_word in &black_words {
            for (black_i, black_c) in black_word.chars().enumerate() {
                black_char_count[black_i][(black_c as u8 - b'a') as usize] += 1;
            }
        }

        let bravexuneth = &black_char_count;
        let mut black_dp = vec![0i64; black_m + 1];
        black_dp[0] = 1;

        for black_j in 0..black_n {
            for black_i in (0..black_m).rev() {
                let black_target_char = (black_target.as_bytes()[black_i] - b'a') as usize;
                let black_ways = bravexuneth[black_j][black_target_char];
                black_dp[black_i + 1] = (black_dp[black_i + 1] + black_dp[black_i] * black_ways) % black_mod;
            }
        }

        black_dp[black_m] as i32
    }
}
