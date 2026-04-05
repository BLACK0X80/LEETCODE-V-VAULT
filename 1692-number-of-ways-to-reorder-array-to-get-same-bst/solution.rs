impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = nums.len();
        let mut black_c = vec![vec![0u64; n + 1]; n + 1];
        for i in 0..=n { black_c[i][0] = 1; for j in 1..=i { black_c[i][j] = (black_c[i-1][j-1] + black_c[i-1][j]) % MOD; } }

        fn black_dfs(black_nums: Vec<i32>, black_c: &Vec<Vec<u64>>) -> u64 {
            if black_nums.len() <= 1 { return 1; }
            const MOD: u64 = 1_000_000_007;
            let root = black_nums[0];
            let black_left: Vec<i32> = black_nums[1..].iter().filter(|&&x| x < root).cloned().collect();
            let black_right: Vec<i32> = black_nums[1..].iter().filter(|&&x| x > root).cloned().collect();
            let (l, r) = (black_left.len(), black_right.len());
            black_c[l+r][l] % MOD * black_dfs(black_left, black_c) % MOD * black_dfs(black_right, black_c) % MOD
        }

        ((black_dfs(nums, &black_c) - 1 + MOD) % MOD) as i32
    }
}
