impl Solution {
    pub fn combination_sum3(black_k: i32, black_n: i32) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        Self::black_bt(black_k, black_n, 1, &mut vec![], &mut black_res);
        black_res
    }
    fn black_bt(k: i32, n: i32, s: i32, black_path: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>) {
        if n == 0 && k == 0 { black_res.push(black_path.clone()); return; }
        if n < 0 || k <= 0 { return; }
        for i in s..=9 {
            black_path.push(i);
            Self::black_bt(k - 1, n - i, i + 1, black_path, black_res);
            black_path.pop();
        }
    }
}
