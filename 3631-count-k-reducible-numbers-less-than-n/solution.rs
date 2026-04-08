impl Solution {
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        let black_n = s.len();
        let black_mod = 1_000_000_007i64;
        
        let mut black_memo = vec![0; black_n + 1];
        for i in 2..=black_n {
            black_memo[i] = 1 + black_memo[i.count_ones() as usize];
        }

        let mut black_comb = vec![vec![0i64; black_n + 1]; black_n + 1];
        for i in 0..=black_n {
            black_comb[i][0] = 1;
            for j in 1..=i { black_comb[i][j] = (black_comb[i-1][j-1] + black_comb[i-1][j]) % black_mod; }
        }

        let mut black_ans = 0i64;
        let mut black_current_bits = 0;
        let black_chars: Vec<char> = s.chars().collect();

        for i in 0..black_n {
            if black_chars[i] == '1' {
                let black_remaining = black_n - 1 - i;
                for b in 0..=black_remaining {
                    let black_total_bits = black_current_bits + b;
                    if black_total_bits > 0 && black_memo[black_total_bits] < k {
                        black_ans = (black_ans + black_comb[black_remaining][b]) % black_mod;
                    }
                }
                black_current_bits += 1;
            }
        }
        
        
        black_ans as i32
    }
}
