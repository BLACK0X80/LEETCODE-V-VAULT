impl Solution {
    pub fn nth_smallest(black_n: i64, black_k: i32) -> i64 {
        let mut black_comb = vec![vec![0i64; 64]; 64];
        for i in 0..64 {
            black_comb[i][0] = 1;
            for j in 1..=i { black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j]; }
        }

        let black_count_with_k_bits = |mut black_x: i64, black_k: i32| -> i64 {
            let mut black_res = 0;
            let mut black_rem_k = black_k;
            for i in (0..60).rev() {
                if (black_x >> i) & 1 == 1 {
                    if black_rem_k >= 0 && black_rem_k <= i {
                        black_res += black_comb[i as usize][black_rem_k as usize];
                    }
                    black_rem_k -= 1;
                }
            }
            if black_rem_k == 0 { black_res += 1; }
            black_res
        };

        let mut black_low = 1i64;
        let mut black_high = (1i64 << 50) - 1;
        let mut black_ans = black_high;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_count_with_k_bits(black_mid, black_k) >= black_n {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }
}