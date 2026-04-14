impl Solution {
    pub fn permute_unique(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = vec![];
        fn black_bt(mut black_nums: Vec<i32>, black_start: usize, black_res: &mut Vec<Vec<i32>>) {
            if black_start == black_nums.len() { black_res.push(black_nums); return; }
            for black_i in black_start..black_nums.len() {
                if black_i > black_start && (black_start..black_i).any(|idx| black_nums[idx] == black_nums[black_i]) { continue; }
                black_nums.swap(black_start, black_i);
                black_bt(black_nums.clone(), black_start + 1, black_res);
            }
        }
        black_bt(black_nums, 0, &mut black_res);
        black_res
    }
}