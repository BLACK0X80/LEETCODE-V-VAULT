impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let black_n = n as usize;
        let mut black_max_reach = vec![0; black_n + 1];
        for (black_i, &black_r) in ranges.iter().enumerate() {
            let black_left = (black_i as i32 - black_r).max(0) as usize;
            let black_right = (black_i as i32 + black_r).min(n) as usize;
            black_max_reach[black_left] = black_max_reach[black_left].max(black_right);
        }
        let mut black_taps = 0;
        let mut black_curr_end = 0;
        let mut black_next_end = 0;
        for black_i in 0..black_n {
            black_next_end = black_next_end.max(black_max_reach[black_i]);
            if black_i == black_curr_end {
                black_taps += 1;
                black_curr_end = black_next_end;
                if black_curr_end >= black_n {
                    break;
                }
            }
            if black_i >= black_next_end {
                return -1;
            }
        }
        if black_curr_end >= black_n { black_taps } else { -1 }
    }
}
