impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let rows = pizza.len();
        let cols = pizza[0].len();
        let mut pref = vec![vec![0; cols + 1]; rows + 1];
        for r in 0..rows {
            for c in 0..cols {
                pref[r+1][c+1] = pref[r][c+1] + pref[r+1][c] - pref[r][c] + (if pizza[r].as_bytes()[c] == b'A' { 1 } else { 0 });
            }
        }
        let apples = |r1: usize, c1: usize, r2: usize, c2: usize| {
            pref[r2+1][c2+1] - pref[r1][c2+1] - pref[r2+1][c1] + pref[r1][c1]
        };
        let mod_val = 1_000_000_007i32;
        let mut dp = vec![vec![0i32; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                dp[r][c] = if apples(r, c, rows-1, cols-1) > 0 { 1 } else { 0 };
            }
        }
        for _ in 2..=k {
            let mut new_dp = vec![vec![0i32; cols]; rows];
            for r in 0..rows {
                for c in 0..cols {
                    let mut ways = 0i32;
                    for i in r+1..rows {
                        if apples(r, c, i-1, cols-1) > 0 { ways = (ways + dp[i][c]) % mod_val; }
                    }
                    for j in c+1..cols {
                        if apples(r, c, rows-1, j-1) > 0 { ways = (ways + dp[r][j]) % mod_val; }
                    }
                    new_dp[r][c] = ways;
                }
            }
            dp = new_dp;
        }
        dp[0][0]
    }
}