impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        let n = board.len();
        let g: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();
        let mut dp = vec![vec![(i64::MIN, 0i64); n]; n];
        dp[n-1][n-1] = (0, 1);
        for i in (0..n).rev() { for j in (0..n).rev() {
            if g[i][j] == b'X' || g[i][j] == b'S' && (i,j) != (n-1,n-1) { continue; }
            let val = if g[i][j].is_ascii_digit() { (g[i][j]-b'0') as i64 } else { 0 };
            if (i,j) == (n-1,n-1) { continue; }
            let mut best = (i64::MIN, 0i64);
            for (ni,nj) in [(i+1,j),(i,j+1),(i+1,j+1)] {
                if ni<n && nj<n && dp[ni][nj].1>0 {
                    let s = dp[ni][nj].0 + val;
                    if s > best.0 { best = (s, dp[ni][nj].1); }
                    else if s == best.0 { best.1 = (best.1 + dp[ni][nj].1) % MOD; }
                }
            }
            if best.1 > 0 { dp[i][j] = best; }
        }}
        if dp[0][0].1 == 0 { vec![0,0] } else { vec![dp[0][0].0 as i32, dp[0][0].1 as i32] }
    }
}