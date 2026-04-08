impl Solution {
    pub fn popcount_depth(black_n: i64, black_k: i32) -> i64 {
        if black_k == 0 { return if black_n >= 1 { 1 } else { 0 }; }
        
        let mut black_depth = vec![0; 65];
        black_depth[1] = 0;
        for i in 2..65 {
            black_depth[i] = 1 + black_depth[i.count_ones() as usize];
        }

        let mut black_comb = vec![vec![0i64; 65]; 65];
        for i in 0..65 {
            black_comb[i][0] = 1;
            for j in 1..=i {
                black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j];
            }
        }

        let mut black_ans = 0;
        let mut black_set_bits = 0;
        let black_bits: Vec<i32> = format!("{:b}", black_n).chars().map(|c| c.to_digit(2).unwrap() as i32).collect();
        let black_len = black_bits.len();

        for i in 0..black_len {
            if black_bits[i] == 1 {
                let black_rem = black_len - 1 - i;
                for b in 0..=black_rem {
                    let black_total = black_set_bits + b;
                    if black_total > 0 && black_depth[black_total] == black_k - 1 {
                        black_ans += black_comb[black_rem][b];
                    }
                }
                black_set_bits += 1;
            }
        }

        if black_depth[black_set_bits as usize] == black_k - 1 {
            black_ans += 1;
        }

        if black_k == 1 { black_ans -= 1; }
        black_ans
    }
}
