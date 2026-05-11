impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let half = n/2;
        let mut black_left = vec![0i64];
        for i in 0..half {
            let mut tmp = black_left.clone();
            for &s in &black_left { tmp.push(s + nums[i] as i64); }
            black_left = tmp;
        }
        let mut black_right = vec![0i64];
        for i in half..n {
            let mut tmp = black_right.clone();
            for &s in &black_right { tmp.push(s + nums[i] as i64); }
            black_right = tmp;
        }
        black_right.sort_unstable();
        let mut black_ans = i64::MAX;
        for &l in &black_left {
            let t = goal as i64 - l;
            let p = black_right.partition_point(|&x| x <= t);
            if p < black_right.len() { black_ans = black_ans.min((l + black_right[p] - goal as i64).abs()); }
            if p > 0 { black_ans = black_ans.min((l + black_right[p-1] - goal as i64).abs()); }
        }
        black_ans as i32
    }
}