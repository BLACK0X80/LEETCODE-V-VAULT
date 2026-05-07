impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let black_n = nums.len();
        let mut black_dp = vec![vec![i32::MAX; black_n]; 1 << black_n];
        let mut black_parent = vec![vec![0usize; black_n]; 1 << black_n];

        for i in 0..black_n {
            black_dp[(1 << black_n) - 1][i] = (i as i32 - nums[0]).abs();
        }

        for mask in (1..(1 << black_n) - 1).rev() {
            for last in 0..black_n {
                if (mask & (1 << last)) == 0 { continue; }
                for next in 0..black_n {
                    if (mask & (1 << next)) != 0 { continue; }
                    let cost = (last as i32 - nums[next]).abs() + black_dp[mask | (1 << next)][next];
                    if cost < black_dp[mask][last] {
                        black_dp[mask][last] = cost;
                        black_parent[mask][last] = next;
                    }
                }
            }
        }

        let mut black_res = vec![0];
        let mut black_m = 1usize;
        let mut black_l = 0usize;
        while black_res.len() < black_n {
            let next = black_parent[black_m][black_l];
            black_res.push(next as i32);
            black_m |= 1 << next;
            black_l = next;
        }
        black_res
    }
}