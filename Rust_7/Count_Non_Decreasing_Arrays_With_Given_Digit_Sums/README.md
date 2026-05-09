# Count Non Decreasing Arrays With Given Digit Sums

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given an integer array <code>digitSum</code> of length <code>n</code>.</p>

<p>An array <code>arr</code> of length <code>n</code> is considered <strong>valid</strong> if:</p>

<ul>
	<li><code>0 &lt;= arr[i] &lt;= 5000</code></li>
	<li>it is <strong>non-decreasing</strong>.</li>
	<li>the <strong>sum of the digits</strong> of <code>arr[i]</code> <strong>equals</strong> <code>digitSum[i]</code>.</li>
</ul>

<p>Return an integer denoting the number of <strong>distinct valid arrays</strong>. Since the answer may be large, return it modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>An array is said to be <strong>non-decreasing</strong> if each element is greater than or equal to the previous element, if it exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">digitSum = [25,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>Numbers whose sum of digits is 25 are 799, 889, 898, 979, 988, and 997.</p>

<p>The only number whose sum of digits is 1 that can appear after these values while keeping the array non-decreasing is 1000.</p>

<p>Thus, the valid arrays are <code>[799, 1000]</code>, <code>[889, 1000]</code>, <code>[898, 1000]</code>, <code>[979, 1000]</code>, <code>[988, 1000]</code>, and <code>[997, 1000]</code>.</p>

<p>Hence, the answer is 6.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">digitSum = [1]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid arrays are <code>[1]</code>, <code>[10]</code>, <code>[100]</code>, and <code>[1000]</code>.</p>

<p>Thus, the answer is 4.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">digitSum = [2,49,23]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no integer in the range [0, 5000] whose sum of digits is 49. Thus, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= digitSum.length &lt;= 1000</code></li>
	<li><code>0 &lt;= digitSum[i] &lt;= 50</code></li>
</ul>


## Hints

1. Use dynamic programming
2. Group every number from 0 to 5000 by its digit sum into sorted vectors once.
3. <code>dp[j]</code> holds the number of ways ending exactly at the j-th candidate for the current position.
4. Build prefix sums on the previous <code>dp</code>, then use <code>upper_bound</code> on the sorted previous list to count all values <= current.

## Solution

```rust
impl Solution {
    pub fn count_arrays(black_digit_sum: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_groups = vec![Vec::new(); 51];
        for black_i in 0..=5000 {
            let mut black_s = 0;
            let mut black_temp = black_i;
            while black_temp > 0 { black_s += black_temp % 10; black_temp /= 10; }
            if black_s <= 50 { black_groups[black_s as usize].push(black_i); }
        }

        let mut black_current_ds = black_digit_sum[0] as usize;
        if black_current_ds > 50 || black_groups[black_current_ds].is_empty() { return 0; }
        
        let mut black_dp = vec![1i64; black_groups[black_current_ds].len()];

        for black_idx in 1..black_digit_sum.len() {
            let black_prev_ds = black_digit_sum[black_idx - 1] as usize;
            let black_curr_ds = black_digit_sum[black_idx] as usize;
            if black_curr_ds > 50 || black_groups[black_curr_ds].is_empty() { return 0; }

            let mut black_next_dp = vec![0i64; black_groups[black_curr_ds].len()];
            let mut black_prefix_sum = 0i64;
            let mut black_p_ptr = 0;

            for (black_c_ptr, &black_curr_val) in black_groups[black_curr_ds].iter().enumerate() {
                while black_p_ptr < black_groups[black_prev_ds].len() && black_groups[black_prev_ds][black_p_ptr] <= black_curr_val {
                    black_prefix_sum = (black_prefix_sum + black_dp[black_p_ptr]) % black_mod;
                    black_p_ptr += 1;
                }
                black_next_dp[black_c_ptr] = black_prefix_sum;
            }
            black_dp = black_next_dp;
            if black_dp.iter().all(|&x| x == 0) { return 0; }
        }

        (black_dp.iter().sum::<i64>() % black_mod) as i32
    }
}
```