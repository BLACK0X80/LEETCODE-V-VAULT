impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut dp = vec![i32::MAX / 2; 1024];
        dp[0] = 0;
        for i in 0..k {
            let mut freq = [0i32; 1024];
            let mut len = 0;
            for j in (i..n).step_by(k) {
                freq[nums[j] as usize] += 1;
                len += 1;
            }
            let min_prev = dp.iter().copied().min().unwrap();
            let mut next_dp = vec![min_prev + len; 1024];
            for v in 0..1024 {
                if freq[v] == 0 { continue; }
                let cost = len - freq[v];
                for x in 0..1024 {
                    if dp[x] > 1000000 { continue; }
                    let val = dp[x] + cost;
                    if val < next_dp[x ^ v] {
                        next_dp[x ^ v] = val;
                    }
                }
            }
            dp = next_dp;
        }
        dp[0]
    }
}
