# Minimum Cost to Divide Array Into Subarrays

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given two integer arrays, <code>nums</code> and <code>cost</code>, of the same size, and an integer <code>k</code>.</p>

<p>You can divide <code>nums</code> into <span data-keyword="subarray-nonempty">subarrays</span>. The cost of the <code>i<sup>th</sup></code> subarray consisting of elements <code>nums[l..r]</code> is:</p>

<ul>
	<li><code>(nums[0] + nums[1] + ... + nums[r] + k * i) * (cost[l] + cost[l + 1] + ... + cost[r])</code>.</li>
</ul>

<p><strong>Note</strong> that <code>i</code> represents the order of the subarray: 1 for the first subarray, 2 for the second, and so on.</p>

<p>Return the <strong>minimum</strong> total cost possible from any valid division.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,4], cost = [4,6,6], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">110</span></p>

<p><strong>Explanation:</strong></p>
The minimum total cost possible can be achieved by dividing <code>nums</code> into subarrays <code>[3, 1]</code> and <code>[4]</code>.

<ul>
	<li>The cost of the first subarray <code>[3,1]</code> is <code>(3 + 1 + 1 * 1) * (4 + 6) = 50</code>.</li>
	<li>The cost of the second subarray <code>[4]</code> is <code>(3 + 1 + 4 + 1 * 2) * 6 = 60</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,8,5,1,14,2,2,12,1], cost = [7,2,8,4,2,2,1,1,2], k = 7</span></p>

<p><strong>Output:</strong> 985</p>

<p><strong>Explanation:</strong></p>
The minimum total cost possible can be achieved by dividing <code>nums</code> into subarrays <code>[4, 8, 5, 1]</code>, <code>[14, 2, 2]</code>, and <code>[12, 1]</code>.

<ul>
	<li>The cost of the first subarray <code>[4, 8, 5, 1]</code> is <code>(4 + 8 + 5 + 1 + 7 * 1) * (7 + 2 + 8 + 4) = 525</code>.</li>
	<li>The cost of the second subarray <code>[14, 2, 2]</code> is <code>(4 + 8 + 5 + 1 + 14 + 2 + 2 + 7 * 2) * (2 + 2 + 1) = 250</code>.</li>
	<li>The cost of the third subarray <code>[12, 1]</code> is <code>(4 + 8 + 5 + 1 + 14 + 2 + 2 + 12 + 1 + 7 * 3) * (1 + 2) = 210</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>cost.length == nums.length</code></li>
	<li><code>1 &lt;= nums[i], cost[i] &lt;= 1000</code></li>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
</ul>


## Hints

1. <code>dp[i]</code> is the minimum cost to split the array suffix starting at <code>i</code>.
2. Observe that no matter how many subarrays we have, if we have the first subarray on the left, the total cost of the previous subarrays increases by <code>k * total_cost_of_the_subarray</code>. This is because when we increase <code>i</code> to <code>(i + 1)</code>, the cost increase is just the suffix sum of the cost array.

## Solution

```rust
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
```