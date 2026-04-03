impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<(i32,i32,i32)> = (0..n)
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect();
        jobs.sort_unstable();

        let mut dp = vec![0i32; n + 1];

        for i in 1..=n {
            let (_, s, p) = jobs[i-1];
            let j = jobs[..i].partition_point(|x| x.0 <= s);
            dp[i] = dp[i-1].max(dp[j] + p);
        }

        dp[n]
    }
}
