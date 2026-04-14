impl Solution {
    pub fn max_frequency_score(mut black_nums: Vec<i32>, black_k: i64) -> i32 {
        black_nums.sort_unstable();
        let black_n = black_nums.len();
        let mut black_pre = vec![0i64; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] + black_nums[i] as i64; }

        let mut black_l = 0;
        let mut black_ans = 0;
        let bravexuneth = &black_nums;

        for black_r in 0..black_n {
            while black_l <= black_r {
                let black_mid = (black_l + black_r) / 2;
                let black_target = bravexuneth[black_mid] as i64;
                
                let black_left_count = (black_mid - black_l) as i64;
                let black_right_count = (black_r - black_mid) as i64;
                
                let black_cost_left = black_left_count * black_target - (black_pre[black_mid] - black_pre[black_l]);
                let black_cost_right = (black_pre[black_r + 1] - black_pre[black_mid + 1]) - black_right_count * black_target;
                
                if black_cost_left + black_cost_right <= black_k { break; }
                black_l += 1;
            }
            black_ans = black_ans.max(black_r - black_l + 1);
        }
        black_ans as i32
    }
}