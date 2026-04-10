impl Solution {
    pub fn min_cost(black_s: String, black_enc_cost: i32, black_flat_cost: i32) -> i64 {
        let mut black_psum = vec![0];
        for black_ch in black_s.chars() {
            black_psum.push(black_psum.last().unwrap() + (if black_ch == '1' { 1 } else { 0 }));
        }
        Self::solve(0, black_s.len(), black_enc_cost as i64, black_flat_cost as i64, &black_psum)
    }

    fn solve(black_l: usize, black_r: usize, black_enc: i64, black_flat: i64, black_psum: &Vec<i32>) -> i64 {
        let black_cnt = (black_psum[black_r] - black_psum[black_l]) as i64;
        let black_sz = (black_r - black_l) as i64;
        
        let mut black_cost = if black_cnt == 0 {
            black_flat
        } else {
            black_cnt * black_sz * black_enc
        };

        if black_cnt > 0 && black_sz % 2 == 0 {
            let black_mid = black_l + (black_r - black_l) / 2;
            let black_split_cost = Self::solve(black_l, black_mid, black_enc, black_flat, black_psum) +
                                  Self::solve(black_mid, black_r, black_enc, black_flat, black_psum);
            black_cost = black_cost.min(black_split_cost);
        }
        
        black_cost
    }
}
