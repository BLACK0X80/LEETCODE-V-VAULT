impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let h = stamp_height as usize;
        let w = stamp_width as usize;
        if h > m || w > n { return grid.iter().all(|r| r.iter().all(|&x| x == 1)); }
        let mut pref = vec![vec![0; n + 1]; m + 1];
        for i in 0..m { for j in 0..n { pref[i+1][j+1] = pref[i][j+1] + pref[i+1][j] - pref[i][j] + grid[i][j]; } }
        let mut diff = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m - h { for j in 0..=n - w { if pref[i+h][j+w] - pref[i][j+w] - pref[i+h][j] + pref[i][j] == 0 { diff[i][j] += 1; diff[i+h][j] -= 1; diff[i][j+w] -= 1; diff[i+h][j+w] += 1; } } }
        for i in 0..=m { for j in 1..=n { diff[i][j] += diff[i][j-1]; } }
        for i in 1..=m { for j in 0..=n { diff[i][j] += diff[i-1][j]; } }
        for i in 0..m { for j in 0..n { if grid[i][j] == 0 && diff[i][j] == 0 { return false; } } }
        true
    }
}
