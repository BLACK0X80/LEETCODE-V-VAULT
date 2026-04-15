impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut dp = vec![-1i32; 5001];
        dp[0] = 0;
        for r in rods {
            let prev = dp.clone();
            for d in 0..=5000 {
                if prev[d] < 0 { continue; }
                let h = prev[d];
                dp[d + r as usize] = dp[d + r as usize].max(h);
                let nd = (d as i32 - r).unsigned_abs() as usize;
                dp[nd] = dp[nd].max(h + r.min(d as i32));
            }
        }
        dp[0]
    }
}