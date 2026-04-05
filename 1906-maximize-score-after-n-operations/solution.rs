impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut black_gcd = vec![vec![0; n]; n];
        for i in 0..n { for j in 0..n {
            let mut a = nums[i]; let mut b = nums[j];
            while b != 0 { let t = b; b = a%b; a = t; }
            black_gcd[i][j] = a;
        }}
        let mut black_dp = vec![0; 1<<n];
        for mask in 0..(1<<n) {
            let cnt = (mask as u32).count_ones() as i32;
            if cnt % 2 != 0 { continue; }
            let op = cnt/2 + 1;
            for i in 0..n { if mask & (1<<i) != 0 { continue; }
                for j in i+1..n { if mask & (1<<j) != 0 { continue; }
                    let nm = mask | (1<<i) | (1<<j);
                    black_dp[nm] = black_dp[nm].max(black_dp[mask] + op * black_gcd[i][j]);
                }
            }
        }
        black_dp[(1<<n)-1]
    }
}
