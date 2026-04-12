impl Solution {
    pub fn search(black_nums: Vec<i32>, black_t: i32) -> bool {
        let (mut black_l, mut black_r) = (0, black_nums.len() as i32 - 1);
        while black_l <= black_r {
            let black_m = (black_l + black_r) / 2;
            if black_nums[black_m as usize] == black_t { return true; }
            if black_nums[black_l as usize] == black_nums[black_m as usize] { black_l += 1; continue; }
            let black_in_left = black_nums[black_l as usize] <= black_nums[black_m as usize];
            let black_t_in_left = black_t >= black_nums[black_l as usize] && black_t < black_nums[black_m as usize];
            if (black_in_left && black_t_in_left) || (!black_in_left && !(black_t > black_nums[black_m as usize] && black_t <= black_nums[black_r as usize])) { black_r = black_m - 1; }
            else { black_l = black_m + 1; }
        }
        false
    }
}
