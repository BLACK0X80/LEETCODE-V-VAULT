impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans = 0;
        for r1 in 0..m {
            let mut col_sum = vec![0i32; n];
            for r2 in r1..m {
                for j in 0..n { col_sum[j] += matrix[r2][j]; }
                let mut map = HashMap::new();
                map.insert(0, 1);
                let mut prefix = 0;
                for j in 0..n {
                    prefix += col_sum[j];
                    ans += map.get(&(prefix - target)).copied().unwrap_or(0);
                    *map.entry(prefix).or_insert(0) += 1;
                }
            }
        }
        ans
    }
}
