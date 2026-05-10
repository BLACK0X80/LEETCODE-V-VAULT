# Minimum Incompatibility

**Difficulty:** Hard
**Tags:** Array, Hash Table, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given an integer array <code>nums</code>​​​ and an integer <code>k</code>. You are asked to distribute this array into <code>k</code> subsets of <strong>equal size</strong> such that there are no two equal elements in the same subset.</p>

<p>A subset&#39;s <strong>incompatibility</strong> is the difference between the maximum and minimum elements in that array.</p>

<p>Return <em>the <strong>minimum possible sum of incompatibilities</strong> of the </em><code>k</code> <em>subsets after distributing the array optimally, or return </em><code>-1</code><em> if it is not possible.</em></p>

<p>A subset is a group integers that appear in the array with no particular order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,4], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The optimal distribution of subsets is [1,2] and [1,4].
The incompatibility is (2-1) + (4-1) = 4.
Note that [1,1] and [2,4] would result in a smaller sum, but the first subset contains 2 equal elements.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [6,3,8,1,3,1,2,2], k = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The optimal distribution of subsets is [1,2], [2,3], [6,8], and [1,3].
The incompatibility is (2-1) + (3-2) + (8-6) + (3-1) = 6.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,3,3,6,3,3], k = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to distribute nums into 3 subsets where no two elements are equal in the same subset.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= nums.length &lt;= 16</code></li>
	<li><code>nums.length</code> is divisible by <code>k</code></li>
	<li><code>1 &lt;= nums[i] &lt;= nums.length</code></li>
</ul>


## Hints

1. The constraints are small enough for a backtrack solution but not any backtrack solution
2. If we use a naive n^k don't you think it can be optimized

## Solution

```rust
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let sz = n / k as usize;
        let mut black_cost = vec![-1i32; 1<<n];
        for mask in 0..(1<<n) {
            if (mask as u32).count_ones() as usize != sz { continue; }
            let mut vals: Vec<i32> = (0..n).filter(|&i| mask & (1<<i) != 0).map(|i| nums[i]).collect();
            vals.sort_unstable();
            if vals.windows(2).all(|w| w[0] != w[1]) {
                black_cost[mask] = vals.last().unwrap() - vals[0];
            }
        }
        let mut black_dp = vec![i32::MAX; 1<<n];
        black_dp[0] = 0;
        for mask in 0..(1<<n) {
            if black_dp[mask] == i32::MAX { continue; }
            let rem = ((1<<n)-1) ^ mask;
            let low = (rem & rem.wrapping_neg()) as usize;
            let mut sub = rem;
            while sub > 0 {
                if sub & low != 0 && black_cost[sub] >= 0 && black_dp[mask] != i32::MAX {
                    black_dp[mask|sub] = black_dp[mask|sub].min(black_dp[mask] + black_cost[sub]);
                }
                sub = (sub-1) & rem;
            }
        }
        if black_dp[(1<<n)-1] == i32::MAX { -1 } else { black_dp[(1<<n)-1] }
    }
}
```