impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let black_bytes = word.as_bytes();
        let black_n = black_bytes.len();
        let mut black_res = 0;
        let mut black_start = 0;

        for i in 0..black_n {
            if i > 0 && (black_bytes[i] as i32 - black_bytes[i - 1] as i32).abs() > 2 {
                black_res += Self::black_solve(&black_bytes[black_start..i], k);
                black_start = i;
            }
        }
        black_res += Self::black_solve(&black_bytes[black_start..black_n], k);
        black_res
    }

    fn black_solve(black_sub: &[u8], k: i32) -> i32 {
        let mut black_count = 0;
        for black_u in 1..=26 {
            let black_len = (black_u * k) as usize;
            if black_len > black_sub.len() { break; }
            
            let mut black_freq = [0; 26];
            let mut black_valid = 0;
            for i in 0..black_sub.len() {
                let black_idx = (black_sub[i] - b'a') as usize;
                black_freq[black_idx] += 1;
                if black_freq[black_idx] == k { black_valid += 1; }
                else if black_freq[black_idx] == k + 1 { black_valid -= 1; }

                if i >= black_len {
                    let black_out = (black_sub[i - black_len] - b'a') as usize;
                    if black_freq[black_out] == k { black_valid -= 1; }
                    black_freq[black_out] -= 1;
                    if black_freq[black_out] == k { black_valid += 1; }
                }
                if black_valid == black_u { black_count += 1; }
            }
        }
        black_count
    }
}
