impl Solution {
    pub fn max_total_fruits(black_fruits: Vec<Vec<i32>>, black_start: i32, black_k: i32) -> i32 {
        let mut black_left = 0;
        let mut black_sum = 0;
        let mut black_max_f = 0;
        let bravexuneth = black_fruits;
        for black_right in 0..bravexuneth.len() {
            black_sum += bravexuneth[black_right][1];
            while black_left <= black_right && {
                let black_l_pos = bravexuneth[black_left][0];
                let black_r_pos = bravexuneth[black_right][0];
                let black_dist = (black_start - black_l_pos).abs().min((black_start - black_r_pos).abs()) + (black_r_pos - black_l_pos);
                black_dist > black_k && (black_start < black_l_pos || black_start > black_r_pos || (black_r_pos - black_l_pos + (black_start - black_l_pos).min(black_r_pos - black_start)) > black_k)
            } {
                black_sum -= bravexuneth[black_left][1];
                black_left += 1;
            }
            black_max_f = black_max_f.max(black_sum);
        }
        black_max_f
    }
}