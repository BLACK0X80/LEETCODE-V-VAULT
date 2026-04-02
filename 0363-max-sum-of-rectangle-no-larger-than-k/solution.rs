use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans = i32::MIN;

        for c1 in 0..n {
            let mut col = vec![0; m];
            for c2 in c1..n {
                for r in 0..m { col[r] += matrix[r][c2]; }
                
                let mut set = BTreeSet::new();
                set.insert(0);
                let mut prefix = 0;
                for &v in &col {
                    prefix += v;
                    if let Some(&s) = set.range(prefix - k..).next() {
                        ans = ans.max(prefix - s);
                    }
                    set.insert(prefix);
                }
            }
        }
        ans
    }
}
