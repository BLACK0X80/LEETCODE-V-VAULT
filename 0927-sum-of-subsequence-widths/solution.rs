impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        nums.sort();
        let n = nums.len();
        let mut pow2 = vec![1i64; n];
        for i in 1..n { pow2[i] = pow2[i-1] * 2 % MOD; }
        let mut ans = 0i64;
        for i in 0..n {
            ans = (ans + nums[i] as i64 * (pow2[i] - pow2[n-1-i])) % MOD;
        }
        ((ans + MOD) % MOD) as i32
    }
}
