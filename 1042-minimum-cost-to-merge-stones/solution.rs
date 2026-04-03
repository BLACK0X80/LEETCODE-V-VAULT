impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;
        if (n - 1) % (k - 1) != 0 { return -1; }

        let mut prefix = vec![0i32; n + 1];
        for i in 0..n { prefix[i+1] = prefix[i] + stones[i]; }

        let mut dp = vec![vec![0i32; n]; n];

        for len in k..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                dp[i][j] = i32::MAX;
                let mut m = i;
                while m < j {
                    let val = dp[i][m] + dp[m+1][j];
                    dp[i][j] = dp[i][j].min(val);
                    m += k - 1;
                }
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += prefix[j+1] - prefix[i];
                }
            }
        }

        dp[0][n-1]
    }
}
