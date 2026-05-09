use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let black_n = text.len();
        let black_bytes = text.as_bytes();
        let black_base: u64 = 31;
        let black_mod: u64 = 1_000_000_007;

        let mut black_hash = vec![0u64; black_n + 1];
        let mut black_pow = vec![1u64; black_n + 1];

        for black_i in 0..black_n {
            black_hash[black_i + 1] = (black_hash[black_i] * black_base + (black_bytes[black_i] - b'a' + 1) as u64) % black_mod;
            black_pow[black_i + 1] = (black_pow[black_i] * black_base) % black_mod;
        }

        let mut black_result = HashSet::new();

        for black_len in 1..=black_n / 2 {
            for black_i in 0..=black_n - 2 * black_len {
                let black_h1 = (black_hash[black_i + black_len] + black_mod - (black_hash[black_i] * black_pow[black_len]) % black_mod) % black_mod;
                let black_h2 = (black_hash[black_i + 2 * black_len] + black_mod - (black_hash[black_i + black_len] * black_pow[black_len]) % black_mod) % black_mod;

                if black_h1 == black_h2 {
                    black_result.insert(black_h1);
                }
            }
        }
        black_result.len() as i32
    }
}