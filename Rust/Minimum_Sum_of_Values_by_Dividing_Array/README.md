# Minimum Sum of Values by Dividing Array

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Bit Manipulation, Segment Tree, Queue

---

## Problem

<p>You are given two arrays <code>nums</code> and <code>andValues</code> of length <code>n</code> and <code>m</code> respectively.</p>

<p>The <strong>value</strong> of an array is equal to the <strong>last</strong> element of that array.</p>

<p>You have to divide <code>nums</code> into <code>m</code> <strong>disjoint contiguous</strong> <span data-keyword="subarray-nonempty">subarrays</span> such that for the <code>i<sup>th</sup></code> subarray <code>[l<sub>i</sub>, r<sub>i</sub>]</code>, the bitwise <code>AND</code> of the subarray elements is equal to <code>andValues[i]</code>, in other words, <code>nums[l<sub>i</sub>] &amp; nums[l<sub>i</sub> + 1] &amp; ... &amp; nums[r<sub>i</sub>] == andValues[i]</code> for all <code>1 &lt;= i &lt;= m</code>, where <code>&amp;</code> represents the bitwise <code>AND</code> operator.</p>

<p>Return <em>the <strong>minimum</strong> possible sum of the <strong>values</strong> of the </em><code>m</code><em> subarrays </em><code>nums</code><em> is divided into</em>. <em>If it is not possible to divide </em><code>nums</code><em> into </em><code>m</code><em> subarrays satisfying these conditions, return</em> <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,4,3,3,2], andValues = [0,3,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>The only possible way to divide <code>nums</code> is:</p>

<ol>
	<li><code>[1,4]</code> as <code>1 &amp; 4 == 0</code>.</li>
	<li><code>[3]</code> as the bitwise <code>AND</code> of a single element subarray is that element itself.</li>
	<li><code>[3]</code> as the bitwise <code>AND</code> of a single element subarray is that element itself.</li>
	<li><code>[2]</code> as the bitwise <code>AND</code> of a single element subarray is that element itself.</li>
</ol>

<p>The sum of the values for these subarrays is <code>4 + 3 + 3 + 2 = 12</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,5,7,7,7,5], andValues = [0,7,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">17</span></p>

<p><strong>Explanation:</strong></p>

<p>There are three ways to divide <code>nums</code>:</p>

<ol>
	<li><code>[[2,3,5],[7,7,7],[5]]</code> with the sum of the values <code>5 + 7 + 5 == 17</code>.</li>
	<li><code>[[2,3,5,7],[7,7],[5]]</code> with the sum of the values <code>7 + 7 + 5 == 19</code>.</li>
	<li><code>[[2,3,5,7,7],[7],[5]]</code> with the sum of the values <code>7 + 7 + 5 == 19</code>.</li>
</ol>

<p>The minimum possible sum of the values is <code>17</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4], andValues = [2]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>The bitwise <code>AND</code> of the entire array <code>nums</code> is <code>0</code>. As there is no possible way to divide <code>nums</code> into a single subarray to have the bitwise <code>AND</code> of elements <code>2</code>, return <code>-1</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= m == andValues.length &lt;= min(n, 10)</code></li>
	<li><code>1 &lt;= nums[i] &lt; 10<sup>5</sup></code></li>
	<li><code>0 &lt;= andValues[j] &lt; 10<sup>5</sup></code></li>
</ul>


## Hints

1. Let <code>dp[i][j]</code> be the optimal answer to split  <code>nums[0..(i - 1)]</code> into the first <code>j</code> andValues.
2. <code>dp[i][j] = min(dp[(i - z)][j - 1]) + nums[i - 1]</code> over all <code>x <= z <= y</code> and <code>dp[0][0] = 0</code>, where <code>x</code> and <code>y</code> are the longest and shortest subarrays ending with <code>nums[i - 1]</code> and the bitwise-and of all the values in it is <code>andValues[j - 1]</code>.
3. The answer is <code>dp[n][m]</code>.
4. To calculate <code>x</code> and <code>y</code>, we can use binary search (or sliding window). Note that the more values we have, the smaller the <code>AND</code> value is.
5. To calculate the result, we need to support RMQ (range minimum query). Segment tree is one way to do it in <code>O(log(n))</code>. But we can use Monotonic Queue since the ranges are indeed “sliding to right” which can be reduced to the classical minimum value in sliding window problem, for a <code>O(n)</code> solution.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_value_sum(black_nums: Vec<i32>, black_and_values: Vec<i32>) -> i32 {
        let mut black_memo = HashMap::new();

        fn black_solve(
            black_idx: usize,
            black_and_idx: usize,
            black_cur_and: i32,
            black_nums: &[i32],
            black_target: &[i32],
            black_memo: &mut HashMap<(usize, usize, i32), i32>,
        ) -> i32 {
            if black_idx == black_nums.len() {
                return if black_and_idx == black_target.len() { 0 } else { 1_000_000_000 };
            }
            if black_and_idx == black_target.len() { return 1_000_000_000; }

            let black_state = (black_idx, black_and_idx, black_cur_and);
            if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }

            let black_next_and = black_cur_and & black_nums[black_idx];
            if (black_next_and & black_target[black_and_idx]) < black_target[black_and_idx] {
                return 1_000_000_000;
            }

            let mut black_ans = black_solve(black_idx + 1, black_and_idx, black_next_and, black_nums, black_target, black_memo);

            if black_next_and == black_target[black_and_idx] {
                let black_res = black_solve(black_idx + 1, black_and_idx + 1, (1 << 20) - 1, black_nums, black_target, black_memo);
                if black_res != 1_000_000_000 {
                    black_ans = black_ans.min(black_nums[black_idx] + black_res);
                }
            }

            black_memo.insert(black_state, black_ans);
            black_ans
        }

        let black_res = black_solve(0, 0, (1 << 20) - 1, &black_nums, &black_and_values, &mut black_memo);
        if black_res >= 1_000_000_000 { -1 } else { black_res }
    }
}
```