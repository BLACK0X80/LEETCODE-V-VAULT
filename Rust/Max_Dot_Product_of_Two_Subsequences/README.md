# Max Dot Product of Two Subsequences

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>Given two arrays <code>nums1</code>&nbsp;and <code><font face="monospace">nums2</font></code><font face="monospace">.</font></p>

<p>Return the maximum dot product&nbsp;between&nbsp;<strong>non-empty</strong> subsequences of nums1 and nums2 with the same length.</p>

<p>A subsequence of an array is a new array which is formed from the original array by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie,&nbsp;<code>[2,3,5]</code>&nbsp;is a subsequence of&nbsp;<code>[1,2,3,4,5]</code>&nbsp;while <code>[1,5,3]</code>&nbsp;is not).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [2,1,-2,5], nums2 = [3,0,-6]
<strong>Output:</strong> 18
<strong>Explanation:</strong> Take subsequence [2,-2] from nums1 and subsequence [3,-6] from nums2.
Their dot product is (2*3 + (-2)*(-6)) = 18.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,-2], nums2 = [2,-6,7]
<strong>Output:</strong> 21
<strong>Explanation:</strong> Take subsequence [3] from nums1 and subsequence [7] from nums2.
Their dot product is (3*7) = 21.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [-1,-1], nums2 = [1,1]
<strong>Output:</strong> -1
<strong>Explanation: </strong>Take subsequence [-1] from nums1 and subsequence [1] from nums2.
Their dot product is -1.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 500</code></li>
	<li><code>-1000 &lt;= nums1[i], nums2[i] &lt;= 1000</code></li>
</ul>


## Hints

1. Use dynamic programming, define DP[i][j] as the maximum dot product of two subsequences starting in the position i of nums1 and position j of nums2.

## Solution

```rust
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (black_n1, black_n2) = (nums1.len(), nums2.len());
        let mut black_dp = vec![vec![-1_000_000_000; black_n2 + 1]; black_n1 + 1];
        for black_i in 1..=black_n1 {
            for black_j in 1..=black_n2 {
                let black_p = nums1[black_i - 1] * nums2[black_j - 1];
                black_dp[black_i][black_j] = *[black_p, black_p + black_dp[black_i - 1][black_j - 1].max(0), black_dp[black_i - 1][black_j], black_dp[black_i][black_j - 1]].iter().max().unwrap();
            }
        }
        black_dp[black_n1][black_n2]
    }
}
```