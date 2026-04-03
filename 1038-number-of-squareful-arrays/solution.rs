impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        
        let is_sq = |a: i64, b: i64| { let s = ((a+b) as f64).sqrt() as i64; s*s == a+b };
        
        let mut dp = vec![vec![0i32; n]; 1<<n];
        for i in 0..n { dp[1<<i][i] = 1; }
        
        for mask in 1..1<<n {
            for last in 0..n {
                if dp[mask][last] == 0 { continue; }
                if mask & (1<<last) == 0 { continue; }
                for next in 0..n {
                    if mask & (1<<next) != 0 { continue; }
                    if !is_sq(nums[last] as i64, nums[next] as i64) { continue; }
                    dp[mask|(1<<next)][next] += dp[mask][last];
                }
            }
        }
        
        let full = (1<<n) - 1;
        let total: i32 = (0..n).map(|i| dp[full][i]).sum();
        
        
        let mut freq = std::collections::HashMap::new();
        for &x in &nums { *freq.entry(x).or_insert(0u64) += 1; }
        let div: u64 = freq.values().map(|&f| (1..=f).product::<u64>()).product();
        
        (total as u64 / div) as i32
    }
}
