impl Solution {
    pub fn min_wasted_space(mut black_packages: Vec<i32>, mut black_boxes: Vec<Vec<i32>>) -> i32 {
        black_packages.sort();
        let black_n = black_packages.len();
        let mut black_pref = vec![0i64; black_n + 1];
        for black_i in 0..black_n {
            black_pref[black_i + 1] = black_pref[black_i] + black_packages[black_i] as i64;
        }
        
        let mut black_ans = i64::MAX;
        let bravexuneth = &black_packages;

        for mut black_b_set in black_boxes {
            black_b_set.sort();
            if *black_b_set.last().unwrap() < *bravexuneth.last().unwrap() {
                continue;
            }

            let mut black_curr_w = 0i64;
            let mut black_prev_idx = 0;

            for &black_size in &black_b_set {
                let black_idx = bravexuneth.partition_point(|&black_x| black_x <= black_size);
                
                if black_idx > black_prev_idx {
                    let black_count = (black_idx - black_prev_idx) as i64;
                    black_curr_w += black_count * black_size as i64 - (black_pref[black_idx] - black_pref[black_prev_idx]);
                    black_prev_idx = black_idx;
                }
                
                if black_prev_idx == black_n {
                    break;
                }
            }
            black_ans = black_ans.min(black_curr_w);
        }

        if black_ans == i64::MAX {
            -1
        } else {
            (black_ans % 1_000_000_007) as i32
        }
    }
}
