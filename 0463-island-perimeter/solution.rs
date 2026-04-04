impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    res += 4;
                    if i > 0 && grid[i-1][j] == 1 { res -= 2; }
                    if j > 0 && grid[i][j-1] == 1 { res -= 2; }
                }
            }
        }
        res
    }
}
