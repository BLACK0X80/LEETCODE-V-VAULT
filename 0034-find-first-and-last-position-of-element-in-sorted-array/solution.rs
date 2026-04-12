impl Solution {
    pub fn search_range(black_nums: Vec<i32>, black_t: i32) -> Vec<i32> {
        let (black_l, black_r) = (black_nums.partition_point(|&x| x < black_t), black_nums.partition_point(|&x| x <= black_t));
        if black_l < black_nums.len() && black_nums[black_l] == black_t { vec![black_l as i32, (black_r - 1) as i32] } else { vec![-1, -1] }
    }
}
