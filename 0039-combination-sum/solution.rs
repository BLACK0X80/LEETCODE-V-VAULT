impl Solution {
    pub fn combination_sum(black_cands: Vec<i32>, black_t: i32) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_bt(black_i: usize, black_rem: i32, black_path: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>, black_c: &[i32]) {
            if black_rem == 0 { black_res.push(black_path.clone()); return; }
            for black_idx in black_i..black_c.len() {
                if black_c[black_idx] <= black_rem {
                    black_path.push(black_c[black_idx]);
                    black_bt(black_idx, black_rem - black_c[black_idx], black_path, black_res, black_c);
                    black_path.pop();
                }
            }
        }
        black_bt(0, black_t, &mut vec![], &mut black_res, &black_cands);
        black_res
    }
}
