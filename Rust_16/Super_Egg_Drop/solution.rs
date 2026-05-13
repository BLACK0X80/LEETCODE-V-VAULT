impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let (k, n) = (k as usize, n as usize);
        let mut dp = vec![0usize; k + 1];
        let mut m = 0;

        while dp[k] < n {
            m += 1;
            for j in (1..=k).rev() {
                dp[j] = dp[j-1] + dp[j] + 1;
            }
        }

        m
    }
}