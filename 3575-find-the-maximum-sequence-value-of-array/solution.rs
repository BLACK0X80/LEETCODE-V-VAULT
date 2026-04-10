impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let black_n = nums.len();
        let black_k = k as usize;
        
        fn black_get_ors(arr: &[i32], k: usize) -> Vec<std::collections::HashSet<i32>> {
            let n = arr.len();
            let mut dp = vec![vec![std::collections::HashSet::new(); k + 1]; n + 1];
            let mut res = vec![std::collections::HashSet::new(); n];
            
            dp[0][0].insert(0);
            for i in 0..n {
                let val = arr[i];
                for j in 0..=k {
                    let prev_set: Vec<i32> = dp[i][j].iter().cloned().collect();
                    for &prev_or in &prev_set {
                        dp[i+1][j].insert(prev_or);
                        if j < k {
                            dp[i+1][j+1].insert(prev_or | val);
                        }
                    }
                }
                res[i] = dp[i+1][k].clone();
            }
            res
        }

        let black_left_ors = black_get_ors(&nums, black_k);
        let mut black_reversed_nums = nums.clone();
        black_reversed_nums.reverse();
        let black_right_ors_rev = black_get_ors(&black_reversed_nums, black_k);
        
        let mut black_max_xor = 0;
        
        for i in (black_k - 1)..(black_n - black_k) {
            let left_set = &black_left_ors[i];
            let right_set = &black_right_ors_rev[black_n - i - 2];
            
            for &l_or in left_set {
                for &r_or in right_set {
                    black_max_xor = black_max_xor.max(l_or ^ r_or);
                }
            }
        }
        
        black_max_xor
    }
}
