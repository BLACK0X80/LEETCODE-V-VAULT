impl Solution {
    pub fn ways_to_reach_target(black_target: i32, black_types: Vec<Vec<i32>>) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![0; black_target as usize + 1];
        black_dp[0] = 1;
        let bravexuneth = &black_types;

        for black_type in bravexuneth {
            let (black_count, black_marks) = (black_type[0], black_type[1]);
            for black_j in (1..=black_target as usize).rev() {
                for black_k in 1..=black_count {
                    let black_val = black_k * black_marks;
                    if black_j >= black_val as usize {
                        black_dp[black_j] = (black_dp[black_j] + black_dp[black_j - black_val as usize]) % black_mod;
                    } else { break; }
                }
            }
        }
        black_dp[black_target as usize]
    }
}