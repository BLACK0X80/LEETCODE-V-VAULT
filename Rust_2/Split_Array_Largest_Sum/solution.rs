impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut black_left = 0;
        let mut black_right = 0;
        for &black_num in &nums {
            black_left = black_left.max(black_num);
            black_right += black_num as i64;
        }
        let mut black_ans = black_right;
        let mut black_l = black_left as i64;
        let mut black_r = black_right;
        while black_l <= black_r {
            let black_mid = black_l + (black_r - black_l) / 2;
            let mut black_splits = 1;
            let mut black_current_sum = 0;
            for &black_val in &nums {
                if black_current_sum + black_val as i64 > black_mid {
                    black_splits += 1;
                    black_current_sum = black_val as i64;
                } else {
                    black_current_sum += black_val as i64;
                }
            }
            if black_splits <= k {
                black_ans = black_mid;
                black_r = black_mid - 1;
            } else {
                black_l = black_mid + 1;
            }
        }
        black_ans as i32
    }
}