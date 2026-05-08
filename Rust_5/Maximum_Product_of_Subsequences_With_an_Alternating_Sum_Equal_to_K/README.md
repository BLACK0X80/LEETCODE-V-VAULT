# Maximum Product of Subsequences With an Alternating Sum Equal to K

**Difficulty:** Hard
**Tags:** Array, Hash Table, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers, <code>k</code> and <code>limit</code>. Your task is to find a non-empty <strong><span data-keyword="subsequence-array">subsequence</span></strong> of <code>nums</code> that:</p>

<ul>
	<li>Has an <strong>alternating sum</strong> equal to <code>k</code>.</li>
	<li><strong>Maximizes</strong> the product of all its numbers <em>without the product exceeding</em> <code>limit</code>.</li>
</ul>

<p>Return the <em>product</em> of the numbers in such a subsequence. If no subsequence satisfies the requirements, return -1.</p>

<p>The <strong>alternating sum</strong> of a <strong>0-indexed</strong> array is defined as the <strong>sum</strong> of the elements at <strong>even</strong> indices <strong>minus</strong> the <strong>sum</strong> of the elements at <strong>odd</strong> indices.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2, limit = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequences with an alternating sum of 2 are:</p>

<ul>
	<li><code>[1, 2, 3]</code>

	<ul>
		<li>Alternating Sum: <code>1 - 2 + 3 = 2</code></li>
		<li>Product: <code>1 * 2 * 3 = 6</code></li>
	</ul>
	</li>
	<li><code>[2]</code>
	<ul>
		<li>Alternating Sum: 2</li>
		<li>Product: 2</li>
	</ul>
	</li>
</ul>

<p>The maximum product within the limit is 6.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,2,3], k = -5, limit = 12</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>A subsequence with an alternating sum of exactly -5 does not exist.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2,3,3], k = 0, limit = 9</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequences with an alternating sum of 0 are:</p>

<ul>
	<li><code>[2, 2]</code>

	<ul>
		<li>Alternating Sum: <code>2 - 2 = 0</code></li>
		<li>Product: <code>2 * 2 = 4</code></li>
	</ul>
	</li>
	<li><code>[3, 3]</code>
	<ul>
		<li>Alternating Sum: <code>3 - 3 = 0</code></li>
		<li>Product: <code>3 * 3 = 9</code></li>
	</ul>
	</li>
	<li><code>[2, 2, 3, 3]</code>
	<ul>
		<li>Alternating Sum: <code>2 - 2 + 3 - 3 = 0</code></li>
		<li>Product: <code>2 * 2 * 3 * 3 = 36</code></li>
	</ul>
	</li>
</ul>

<p>The subsequence <code>[2, 2, 3, 3]</code> has the greatest product with an alternating sum equal to <code>k</code>, but <code>36 &gt; 9</code>. The next greatest product is 9, which is within the limit.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 150</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 12</code></li>
	<li><code>-10<sup>5</sup> &lt;= k &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= limit &lt;= 5000</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Save all possible products with a particular sum.
3. Handle the case where a subsequence has a product of <code>0</code> and an alternating sum of <code>k</code>.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let mut black_dp: HashMap<u64, i32> = HashMap::new();
        let black_n = nums.len();
        let mut black_dp2: Vec<Vec<Vec<Vec<Option<bool>>>>> = vec![vec![vec![vec![None; 2]; 2]; 1801]; black_n + 1];
        
        let mut black_ans = -1;
        if k > -36 && k < 36 {
            black_ans = Self::black_solve(0, 0, 1, 1, &nums, k, limit, 0, &mut black_dp);
        }

        let black_does_contains_zero = Self::black_can_be(0, 0, 0, &nums, 1, k, &mut black_dp2);
        
        if black_ans == -1 {
            if black_does_contains_zero { 0 } else { -1 }
        } else {
            black_ans
        }
    }

    fn black_can_be(
        black_i: usize,
        black_sum: i32,
        black_zero_taken: usize,
        black_arr: &Vec<i32>,
        black_is_odd: usize,
        black_req: i32,
        black_dp2: &mut Vec<Vec<Vec<Vec<Option<bool>>>>>
    ) -> bool {
        if black_i >= black_arr.len() {
            return black_sum == black_req && black_zero_taken == 1;
        }

        let black_sum_idx = (black_sum + 900) as usize;
        if let Some(black_res) = black_dp2[black_i][black_sum_idx][black_zero_taken][black_is_odd] {
            return black_res;
        }

        let black_take = if black_is_odd == 1 {
            Self::black_can_be(
                black_i + 1,
                black_sum + black_arr[black_i],
                black_zero_taken | (if black_arr[black_i] == 0 { 1 } else { 0 }),
                black_arr,
                0,
                black_req,
                black_dp2
            )
        } else {
            Self::black_can_be(
                black_i + 1,
                black_sum - black_arr[black_i],
                black_zero_taken | (if black_arr[black_i] == 0 { 1 } else { 0 }),
                black_arr,
                1,
                black_req,
                black_dp2
            )
        };

        let black_not_take = Self::black_can_be(black_i + 1, black_sum, black_zero_taken, black_arr, black_is_odd, black_req, black_dp2);
        
        let black_final_res = black_take || black_not_take;
        black_dp2[black_i][black_sum_idx][black_zero_taken][black_is_odd] = Some(black_final_res);
        black_final_res
    }

    fn black_solve(
        black_i: usize,
        black_sum: i32,
        black_pro: i32,
        black_is_odd: usize,
        black_arr: &Vec<i32>,
        black_req: i32,
        black_limit: i32,
        black_start: usize,
        black_dp: &mut HashMap<u64, i32>
    ) -> i32 {
        if black_sum <= -40 || black_sum >= 40 || black_pro > black_limit {
            return -1;
        }

        if black_i >= black_arr.len() {
            return if black_sum == black_req && black_pro <= black_limit && black_start == 1 {
                black_pro
            } else {
                -1
            };
        }

        let black_key = Self::black_encode_state(black_i, black_sum, black_pro, black_is_odd, black_start, black_limit);
        if let Some(&black_res) = black_dp.get(&black_key) {
            return black_res;
        }

        let mut black_pick = -1;
        if black_arr[black_i] != 0 {
            if black_is_odd == 1 {
                black_pick = Self::black_solve(
                    black_i + 1,
                    black_sum + black_arr[black_i],
                    black_pro * black_arr[black_i],
                    0,
                    black_arr,
                    black_req,
                    black_limit,
                    1,
                    black_dp
                );
            } else {
                black_pick = Self::black_solve(
                    black_i + 1,
                    black_sum - black_arr[black_i],
                    black_pro * black_arr[black_i],
                    1,
                    black_arr,
                    black_req,
                    black_limit,
                    1,
                    black_dp
                );
            }
        }

        let black_not_pick = Self::black_solve(black_i + 1, black_sum, black_pro, black_is_odd, black_arr, black_req, black_limit, black_start, black_dp);
        let black_final_res = black_pick.max(black_not_pick);
        black_dp.insert(black_key, black_final_res);
        black_final_res
    }

    fn black_encode_state(black_i: usize, black_sum: i32, black_pro: i32, black_is_odd: usize, black_start: usize, black_limit: i32) -> u64 {
        let black_f3 = 4u64;
        let black_f2 = (black_limit as u64 + 1) * black_f3;
        let black_f1 = 81u64 * black_f2;
        (black_i as u64) * black_f1 + (black_sum + 40) as u64 * black_f2 + (black_pro as u64) * black_f3 + (black_is_odd as u64 * 2 + black_start as u64)
    }
}
```