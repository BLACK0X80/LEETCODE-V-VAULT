impl Solution {
    pub fn three_sum(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = Vec::new();
        let black_n = black_nums.len();
        for black_i in 0..black_n {
            if black_i > 0 && black_nums[black_i] == black_nums[black_i-1] { continue; }
            let (mut black_l, mut black_r) = (black_i + 1, black_n - 1);
            while black_l < black_r {
                let black_s = black_nums[black_i] + black_nums[black_l] + black_nums[black_r];
                if black_s == 0 {
                    black_res.push(vec![black_nums[black_i], black_nums[black_l], black_nums[black_r]]);
                    black_l += 1;
                    while black_l < black_r && black_nums[black_l] == black_nums[black_l-1] { black_l += 1; }
                } else if black_s < 0 { black_l += 1; } else { black_r -= 1; }
            }
        }
        black_res
    }
}
