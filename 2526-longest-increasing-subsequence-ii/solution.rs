impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let black_max_val = 100001;
        let mut black_tree = vec![0; 2 * black_max_val];
        let mut black_ans = 0;

        for &black_x in &nums {
            let black_val = black_x as usize;
            let mut black_l = (black_val.saturating_sub(k as usize)).max(1) + black_max_val;
            let mut black_r = black_val + black_max_val;
            let mut black_m = 0;
            
            while black_l < black_r {
                if black_l % 2 == 1 { black_m = black_m.max(black_tree[black_l]); black_l += 1; }
                if black_r % 2 == 1 { black_r -= 1; black_m = black_m.max(black_tree[black_r]); }
                black_l /= 2; black_r /= 2;
            }
            
            let black_res = black_m + 1;
            black_ans = black_ans.max(black_res);
            let mut black_idx = black_val + black_max_val;
            black_tree[black_idx] = black_res;
            while black_idx > 1 {
                black_idx /= 2;
                black_tree[black_idx] = black_tree[2 * black_idx].max(black_tree[2 * black_idx + 1]);
            }
        }
        black_ans
    }
}
