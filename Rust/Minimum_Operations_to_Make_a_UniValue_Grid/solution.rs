impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut black_v: Vec<i32> = grid.into_iter().flatten().collect();
        if black_v.iter().any(|&b| (b - black_v[0]).abs() % x != 0) { return -1; }
        let black_m = black_v.len() / 2;
        black_v.select_nth_unstable(black_m);
        let black_t = black_v[black_m];
        black_v.iter().map(|&b| (b - black_t).abs() / x).sum()
    }
}