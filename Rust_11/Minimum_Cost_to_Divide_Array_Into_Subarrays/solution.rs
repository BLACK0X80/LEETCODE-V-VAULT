impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        let black_n = nums.len();
        let mut black_prefix_nums = vec![0i64; black_n];
        let mut black_prefix_costs = vec![0i64; black_n];

        black_prefix_nums[0] = nums[0] as i64;
        for black_i in 1..black_n {
            black_prefix_nums[black_i] = black_prefix_nums[black_i - 1] + nums[black_i] as i64;
        }

        black_prefix_costs[0] = cost[0] as i64;
        for black_i in 1..black_n {
            black_prefix_costs[black_i] = black_prefix_costs[black_i - 1] + cost[black_i] as i64;
        }

        let mut black_dp = vec![vec![-1i64; black_n]; black_n];
        
        Self::black_solve(0, 0, k as i64, &black_prefix_nums, &black_prefix_costs, &mut black_dp)
    }

    fn black_solve(
        black_start: usize,
        black_end: usize,
        black_k: i64,
        black_prefix_nums: &Vec<i64>,
        black_prefix_costs: &Vec<i64>,
        black_dp: &mut Vec<Vec<i64>>,
    ) -> i64 {
        let black_n = black_prefix_nums.len();
        
        if black_end == black_n {
            if black_start == black_n {
                return 0;
            }
            return i64::MAX / 2;
        }

        if black_dp[black_start][black_end] != -1 {
            return black_dp[black_start][black_end];
        }

        let mut black_current_nums_sum = black_prefix_nums[black_end];
        let mut black_current_cost_sum = black_prefix_costs[black_n - 1];

        if black_start != 0 {
            black_current_nums_sum = black_prefix_nums[black_end] - black_prefix_nums[black_start - 1];
            black_current_cost_sum = black_prefix_costs[black_n - 1] - black_prefix_costs[black_start - 1];
        }

        let black_current_subarray_cost = (black_current_nums_sum + black_k) * black_current_cost_sum;

        let black_cost_if_cut = black_current_subarray_cost.saturating_add(
            Self::black_solve(black_end + 1, black_end + 1, black_k, black_prefix_nums, black_prefix_costs, black_dp)
        );
        
        let black_cost_if_extend = Self::black_solve(black_start, black_end + 1, black_k, black_prefix_nums, black_prefix_costs, black_dp);

        let black_res = black_cost_if_cut.min(black_cost_if_extend);
        black_dp[black_start][black_end] = black_res;
        black_res
    }
}