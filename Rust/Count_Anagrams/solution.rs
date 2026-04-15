impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_f = vec![1i64; s.len() + 1];
        let mut black_inv = vec![1i64; s.len() + 1];
        for black_i in 1..=s.len() { black_f[black_i] = (black_f[black_i - 1] * black_i as i64) % black_mod; }
        fn black_p(mut black_b: i64, mut black_e: i64, black_m: i64) -> i64 {
            let mut black_r = 1;
            while black_e > 0 {
                if black_e % 2 == 1 { black_r = (black_r * black_b) % black_m; }
                black_b = (black_b * black_b) % black_m;
                black_e /= 2;
            }
            black_r
        }
        black_inv[s.len()] = black_p(black_f[s.len()], black_mod - 2, black_mod);
        for black_i in (0..s.len()).rev() { black_inv[black_i] = (black_inv[black_i + 1] * (black_i + 1) as i64) % black_mod; }
        let mut black_ans = 1i64;
        for black_word in s.split_whitespace() {
            let mut black_cnt = [0i64; 26];
            for &black_b in black_word.as_bytes() { black_cnt[(black_b - b'a') as usize] += 1; }
            let mut black_res = black_f[black_word.len()];
            for &black_c in &black_cnt { if black_c > 1 { black_res = (black_res * black_inv[black_c as usize]) % black_mod; } }
            black_ans = (black_ans * black_res) % black_mod;
        }
        black_ans as i32
    }
}