impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const M: u64 = 1_000_000_007;
        let n = n as usize;
        let mut dp = vec![[0u64; 6]; n + 1];
        for j in 0..6 { dp[1][j] = 1; }

        for i in 2..=n {
            for j in 0..6 {
                for k in 1..=roll_max[j] as usize {
                    if i < k { break; }
                    if k == i {
                        dp[i][j] = (dp[i][j] + 1) % M;
                    } else {
                        let sum: u64 = (0..6).filter(|&f| f != j)
                            .map(|f| dp[i-k][f]).sum::<u64>() % M;
                        dp[i][j] = (dp[i][j] + sum) % M;
                    }
                }
            }
        }

        (dp[n].iter().sum::<u64>() % M) as i32
    }
}