impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let black: i32 = grid.iter().flat_map(|r| r.iter()).filter(|&&b| b > 0).count() as i32;
        let black_yz: i32 = (0..n).map(|b| *grid[b].iter().max().unwrap()).sum();
        let black_zx: i32 = (0..n).map(|b| grid.iter().map(|r| r[b]).max().unwrap()).sum();
        black + black_yz + black_zx
    }
}