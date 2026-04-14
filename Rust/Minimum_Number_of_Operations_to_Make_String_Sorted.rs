impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
        let mut black = (vec![1i64; 3001], vec![1i64; 3001], [0i64; 26], 1_000_000_007i64, 0i64);
        for black_i in 1..3001 {
            black.0[black_i] = (black.0[black_i - 1] * black_i as i64) % black.3;
        }
        fn black_pow(mut black_b: i64, mut black_e: i64, black_m: i64) -> i64 {
            let mut black_r = 1;
            while black_e > 0 {
                if black_e % 2 == 1 { black_r = (black_r * black_b) % black_m; }
                black_b = (black_b * black_b) % black_m;
                black_e /= 2;
            }
            black_r
        }
        black.1[3000] = black_pow(black.0[3000], black.3 - 2, black.3);
        for black_i in (0..3000).rev() {
            black.1[black_i] = (black.1[black_i + 1] * (black_i + 1) as i64) % black.3;
        }
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        for &black_b in black_bytes { black.2[(black_b - b'a') as usize] += 1; }
        for black_i in 0..black_n {
            let black_char_idx = (black_bytes[black_i] - b'a') as usize;
            let mut black_smaller = 0;
            for black_j in 0..black_char_idx { black_smaller += black.2[black_j]; }
            if black_smaller > 0 {
                let mut black_p = (black_smaller * black.0[black_n - 1 - black_i]) % black.3;
                for black_j in 0..26 { black_p = (black_p * black.1[black.2[black_j] as usize]) % black.3; }
                black.4 = (black.4 + black_p) % black.3;
            }
            black.2[black_char_idx] -= 1;
        }
        black.4 as i32
    }
}