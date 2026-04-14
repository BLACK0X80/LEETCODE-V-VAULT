impl Solution {
    pub fn max_subarrays(black_n: i32, black_conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let black_n_usize = black_n as usize;
        let mut black_conflicts = vec![Vec::new(); black_n_usize + 1];
        
        for black_pair in black_conflicting_pairs {
            let mut black_u = black_pair[0] as usize;
            let mut black_v = black_pair[1] as usize;
            if black_u > black_v { std::mem::swap(&mut black_u, &mut black_v); }
            black_conflicts[black_v].push(black_u);
        }

        let mut black_max_l = 0;
        let mut black_second_max_l = 0;
        let mut black_gains = vec![0i64; black_n_usize + 1];
        let mut black_valid_count = 0i64;

        for black_right in 1..=black_n_usize {
            for &black_left in &black_conflicts[black_right] {
                if black_left > black_max_l {
                    black_second_max_l = black_max_l;
                    black_max_l = black_left;
                } else if black_left > black_second_max_l {
                    black_second_max_l = black_left;
                }
            }

            black_valid_count += (black_right - black_max_l) as i64;
            if black_max_l > 0 {
                black_gains[black_max_l] += (black_max_l - black_second_max_l) as i64;
            }
        }

        let black_max_gain = black_gains.into_iter().max().unwrap_or(0);
        black_valid_count + black_max_gain
    }
}