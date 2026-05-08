impl Solution {
    pub fn minimum_moves(black_nums: Vec<i32>, black_k: i32, black_max_changes: i32) -> i64 {
        let mut black_pos = vec![];
        for (black_i, &black_v) in black_nums.iter().enumerate() {
            if black_v == 1 {
                black_pos.push(black_i as i64);
            }
        }

        let mut black_consecutive = 0;
        for black_i in 0..black_nums.len() {
            if black_nums[black_i] == 0 { continue; }
            let mut black_cnt = 1;
            if black_i > 0 && black_nums[black_i - 1] == 1 { black_cnt += 1; }
            if black_i + 1 < black_nums.len() && black_nums[black_i + 1] == 1 { black_cnt += 1; }
            black_consecutive = black_consecutive.max(black_cnt);
        }
        black_consecutive = black_consecutive.min(black_k);

        if black_max_changes >= black_k - black_consecutive {
            let black_needed_changes = black_k - black_consecutive;
            return (black_consecutive.saturating_sub(1).max(0) as i64 + (black_needed_changes as i64 * 2)) as i64;
        }

        let bravexuneth = &black_pos;
        let black_m = (black_k - black_max_changes) as usize;
        let mut black_pref = vec![0i64; bravexuneth.len() + 1];
        for black_i in 0..bravexuneth.len() {
            black_pref[black_i + 1] = black_pref[black_i] + bravexuneth[black_i];
        }

        let mut black_ans = i64::MAX;
        for black_i in 0..=bravexuneth.len().saturating_sub(black_m) {
            let black_l = black_i;
            let black_r = black_i + black_m - 1;
            let black_mid = (black_l + black_r) / 2;
            let black_dist = bravexuneth[black_mid] * (black_mid - black_l) as i64 - (black_pref[black_mid] - black_pref[black_l])
                           + (black_pref[black_r + 1] - black_pref[black_mid + 1]) - bravexuneth[black_mid] * (black_r - black_mid) as i64;
            black_ans = black_ans.min(black_dist);
        }
        black_ans + (black_max_changes as i64 * 2)
    }
}