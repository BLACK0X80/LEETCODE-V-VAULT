impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len();
        let mut black_sum = vec![0i32; 1<<n];
        for mask in 1..(1<<n) {
            let i = (mask as u32).trailing_zeros() as usize;
            black_sum[mask] = black_sum[mask^(1<<i)] + jobs[i];
        }
        let mut black_dp = vec![i32::MAX; 1<<n];
        black_dp[0] = 0;
        for mask in 1..(1<<n) {
            let mut sub = mask;
            while sub > 0 {
                if black_dp[mask^sub] != i32::MAX {
                    black_dp[mask] = black_dp[mask].min(black_dp[mask^sub].max(black_sum[sub]));
                }
                sub = (sub-1) & mask;
            }
        }
        let full = (1<<n)-1;
        let mut black_ans = vec![i32::MAX; 1<<n];
        black_ans[0] = 0;
        for _ in 0..k {
            for mask in (0..=full).rev() {
                if black_ans[mask] == i32::MAX { continue; }
                let rem = full ^ mask;
                let mut sub = rem;
                while sub > 0 {
                    black_ans[mask|sub] = black_ans[mask|sub].min(black_ans[mask].max(black_sum[sub]));
                    sub = (sub-1) & rem;
                }
            }
        }
        black_ans[full]
    }
}
