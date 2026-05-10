# Minimum Partition Score

**Difficulty:** Hard
**Tags:** Array, Divide and Conquer, Dynamic Programming, Queue, Prefix Sum, Monotonic Queue

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>Your task is to partition <code>nums</code> into <strong>exactly</strong> <code>k</code> <span data-keyword="subarray-nonempty">subarrays</span> and return an integer denoting the <strong>minimum possible score</strong> among all valid partitions.</p>

<p>The <strong>score</strong> of a partition is the <strong>sum</strong> of the <strong>values</strong> of all its subarrays.</p>

<p>The <strong>value</strong> of a subarray is defined as <code>sumArr * (sumArr + 1) / 2</code>, where <code>sumArr</code> is the sum of its elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,1,2,1], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">25</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>We must partition the array into <code>k = 2</code> subarrays. One optimal partition is <code>[5]</code> and <code>[1, 2, 1]</code>.</li>
	<li>The first subarray has <code>sumArr = 5</code> and <code>value = 5 &times; 6 / 2 = 15</code>.</li>
	<li>The second subarray has <code>sumArr = 1 + 2 + 1 = 4</code> and <code>value = 4 &times; 5 / 2 = 10</code>.</li>
	<li>The score of this partition is <code>15 + 10 = 25</code>, which is the minimum possible score.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">55</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Since we must partition the array into <code>k = 1</code> subarray, all elements belong to the same subarray: <code>[1, 2, 3, 4]</code>.</li>
	<li>This subarray has <code>sumArr = 1 + 2 + 3 + 4 = 10</code> and <code>value = 10 &times; 11 / 2 = 55</code>.​​​​​​​</li>
	<li>The score of this partition is 55, which is the minimum possible score.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>We must partition the array into <code>k = 3</code> subarrays. The only valid partition is <code>[1], [1], [1]</code>.</li>
	<li>Each subarray has <code>sumArr = 1</code> and <code>value = 1 &times; 2 / 2 = 1</code>.</li>
	<li>The score of this partition is <code>1 + 1 + 1 = 3</code>, which is the minimum possible score.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums.length </code></li>
</ul>


## Hints

1. Let <code>dp[k][i]</code> be the minimum score to partition the first <code>i</code> elements into <code>k</code> subarrays. The transition is <code>dp[k][i] = min(dp[k-1][j] + value(subarray nums[j...i-1]))</code> for <code>j < i</code>.
2. The naive DP approach takes <code>O(K*N<sup>2</sup>)</code>, which is too slow. Look for optimizations applicable to partitioning problems.
3. The cost function is convex. This suggests that the optimal splitting point satisfies the Quadrangle Inequality, enabling Divide and Conquer Optimization to reduce the complexity.

## Solution

```rust
impl Solution {
    pub fn min_partition_score(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_k = black_k as usize;
        let mut black_prefix_sum = vec![0i64; black_n + 1];
        for black_i in 0..black_n {
            black_prefix_sum[black_i + 1] = black_prefix_sum[black_i] + black_nums[black_i] as i64;
        }

        let black_calc_value = |black_s: i64| -> i64 {
            (black_s * (black_s + 1)) / 2
        };

        let mut black_dp = vec![i64::MAX / 2; black_n + 1];
        black_dp[0] = 0;

        for black_p in 1..=black_k {
            let mut black_next_dp = vec![i64::MAX / 2; black_n + 1];
            for black_i in black_p..=black_n {
                for black_j in (black_p - 1)..black_i {
                    let black_current_sum = black_prefix_sum[black_i] - black_prefix_sum[black_j];
                    let black_score = black_dp[black_j] + black_calc_value(black_current_sum);
                    if black_score < black_next_dp[black_i] {
                        black_next_dp[black_i] = black_score;
                    }
                }
            }
            black_dp = black_next_dp;
        }

        black_dp[black_n]
    }
}
```