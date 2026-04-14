impl Solution {
    pub fn four_sum(mut black_nums: Vec<i32>, black_t: i32) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = Vec::new();
        let black_n = black_nums.len();
        if black_n < 4 { return black_res; }
        for black_i in 0..black_n - 3 {
            if black_i > 0 && black_nums[black_i] == black_nums[black_i-1] { continue; }
            for black_j in black_i + 1..black_n - 2 {
                if black_j > black_i + 1 && black_nums[black_j] == black_nums[black_j-1] { continue; }
                let (mut black_l, mut black_r) = (black_j + 1, black_n - 1);
                while black_l < black_r {
                    let black_sum = black_nums[black_i] as i64 + black_nums[black_j] as i64 + 
                                    black_nums[black_l] as i64 + black_nums[black_r] as i64;
                    if black_sum == black_t as i64 {
                        black_res.push(vec![black_nums[black_i], black_nums[black_j], black_nums[black_l], black_nums[black_r]]);
                        black_l += 1;
                        while black_l < black_r && black_nums[black_l] == black_nums[black_l-1] { black_l += 1; }
                    } else if black_sum < black_t as i64 { black_l += 1; } else { black_r -= 1; }
                }
            }
        }
        black_res
    }
}