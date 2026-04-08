impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = nums.len();
        let black_max = *nums.iter().max().unwrap() as usize;
        let mut black_dp = vec![1i64; nums[0] as usize + 1];
        black_dp.resize(black_max + 1, 1); 

        for i in 1..black_n {
            let mut black_next_dp = vec![0i64; black_max + 1];
            let mut black_pref = vec![0i64; black_max + 2];
            for j in 0..=black_max {
                black_pref[j + 1] = (black_pref[j] + black_dp[j]) % black_mod;
            }

            for j in 0..=nums[i] as usize {
                let black_k = j as i32 - (nums[i] - nums[i - 1]).max(0);
                if black_k >= 0 {
                    black_next_dp[j] = black_pref[black_k as usize + 1];
                }
            }
            black_dp = black_next_dp;
        }

        (black_dp.iter().sum::<i64>() % black_mod) as i32
    }
}
