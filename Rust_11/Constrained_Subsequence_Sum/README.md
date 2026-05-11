# Constrained Subsequence Sum

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Queue, Sliding Window, Heap (Priority Queue), Monotonic Queue

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return the maximum sum of a <strong>non-empty</strong> subsequence of that array such that for every two <strong>consecutive</strong> integers in the subsequence, <code>nums[i]</code> and <code>nums[j]</code>, where <code>i &lt; j</code>, the condition <code>j - i &lt;= k</code> is satisfied.</p>

<p>A <em>subsequence</em> of an array is obtained by deleting some number of elements (can be zero) from the array, leaving the remaining elements in their original order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,2,-10,5,20], k = 2
<strong>Output:</strong> 37
<b>Explanation:</b> The subsequence is [10, 2, 5, 20].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-1,-2,-3], k = 1
<strong>Output:</strong> -1
<b>Explanation:</b> The subsequence must be non-empty, so we choose the largest number.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,-2,-10,-5,20], k = 2
<strong>Output:</strong> 23
<b>Explanation:</b> The subsequence is [10, -2, -5, 20].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let dp[i] be the solution for the prefix of the array that ends at index i, if the element at index i is in the subsequence.
3. dp[i] = nums[i] + max(0, dp[i-k], dp[i-k+1], ..., dp[i-1])
4. Use a heap with the sliding window technique to optimize the dp.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut dq: VecDeque<i32> = VecDeque::new();
        let mut black_res = nums[0];
        let black_k = k as usize;
        
        for black_i in 0..nums.len() {
            if !dq.is_empty() {
                nums[black_i] += *dq.front().unwrap();
            }
            let black_curr = nums[black_i];
            black_res = black_res.max(black_curr);
            
            while !dq.is_empty() && black_curr > *dq.back().unwrap() {
                dq.pop_back();
            }
            if black_curr > 0 {
                dq.push_back(black_curr);
            }
            if black_i >= black_k && !dq.is_empty() && *dq.front().unwrap() == nums[black_i - black_k] {
                dq.pop_front();
            }
        }
        black_res
    }
}
```