impl Solution {
    pub fn subsets_with_dup(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = vec![vec![]];
        let (mut black_start, mut black_end) = (0, 0);
        for black_i in 0..black_nums.len() {
            black_start = if black_i > 0 && black_nums[black_i] == black_nums[black_i-1] { black_end } else { 0 };
            black_end = black_res.len();
            for black_j in black_start..black_end {
                let mut black_subset = black_res[black_j].clone();
                black_subset.push(black_nums[black_i]);
                black_res.push(black_subset);
            }
        }
        black_res
    }
}
