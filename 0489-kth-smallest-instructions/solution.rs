impl Solution {
    pub fn kth_smallest_path(black_dest: Vec<i32>, mut black_k: i32) -> String {
        let (black_v_total, black_h_total) = (black_dest[0] as usize, black_dest[1] as usize);
        let mut black_comb = vec![vec![0i32; 31]; 31];
        for i in 0..31 {
            black_comb[i][0] = 1;
            for j in 1..=i { black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j]; }
        }

        let mut black_res = String::new();
        let (mut black_h, mut black_v) = (black_h_total, black_v_total);

        for _ in 0..(black_v_total + black_h_total) {
            if black_h > 0 {
                let black_count = black_comb[black_h + black_v - 1][black_v];
                if black_k <= black_count {
                    black_res.push('H');
                    black_h -= 1;
                } else {
                    black_res.push('V');
                    black_k -= black_count;
                    black_v -= 1;
                }
            } else {
                black_res.push('V');
                black_v -= 1;
            }
        }
        black_res
    }
}
