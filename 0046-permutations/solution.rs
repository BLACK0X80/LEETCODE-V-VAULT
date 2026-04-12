impl Solution {
    pub fn permute(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_bt(black_start: usize, black_nums: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>) {
            if black_start == black_nums.len() { black_res.push(black_nums.clone()); return; }
            for black_i in black_start..black_nums.len() {
                black_nums.swap(black_start, black_i);
                black_bt(black_start + 1, black_nums, black_res);
                black_nums.swap(black_start, black_i);
            }
        }
        black_bt(0, &mut black_nums, &mut black_res);
        black_res
    }
}
