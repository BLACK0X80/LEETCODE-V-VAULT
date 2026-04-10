impl Solution {
    pub fn count_palindromes(black_s: String) -> i32 {
        let black_mod = 1_000_000_007;
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        
        if black_n < 5 { return 0; }

        let mut black_pre = vec![vec![0i64; 100]; black_n];
        let mut black_cnt = [0i64; 10];
        
        for black_i in 0..black_n {
            let black_d = (black_bytes[black_i] - b'0') as usize;
            if black_i > 0 { black_pre[black_i] = black_pre[black_i - 1].clone(); }
            for black_prev in 0..10 {
                black_pre[black_i][black_prev * 10 + black_d] += black_cnt[black_prev];
            }
            black_cnt[black_d] += 1;
        }

        let mut black_suf = vec![vec![0i64; 100]; black_n];
        let mut black_cnt_suf = [0i64; 10];
        for black_i in (0..black_n).rev() {
            let black_d = (black_bytes[black_i] - b'0') as usize;
            if black_i < black_n - 1 { black_suf[black_i] = black_suf[black_i + 1].clone(); }
            for black_next in 0..10 {
                black_suf[black_i][black_next * 10 + black_d] += black_cnt_suf[black_next];
            }
            black_cnt_suf[black_d] += 1;
        }

        let mut black_res = 0i64;
        for black_i in 2..black_n - 2 {
            for black_pair in 0..100 {
                black_res = (black_res + black_pre[black_i - 1][black_pair] * black_suf[black_i + 1][black_pair]) % black_mod;
            }
        }
        
        black_res as i32
    }
}
