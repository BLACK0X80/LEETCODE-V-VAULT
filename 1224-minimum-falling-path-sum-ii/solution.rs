impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut prev = grid[0].clone();
        for i in 1..n {
            let mut min1 = (i32::MAX, usize::MAX);
            let mut min2 = (i32::MAX, usize::MAX);
            for j in 0..n {
                if prev[j] < min1.0 { min2=min1; min1=(prev[j],j); }
                else if prev[j] < min2.0 { min2=(prev[j],j); }
            }
            let mut cur = vec![0; n];
            for j in 0..n {
                cur[j] = grid[i][j] + if j!=min1.1 { min1.0 } else { min2.0 };
            }
            prev = cur;
        }
        *prev.iter().min().unwrap()
    }
}
