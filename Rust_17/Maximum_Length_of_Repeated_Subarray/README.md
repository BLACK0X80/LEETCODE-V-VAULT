# Maximum Length of Repeated Subarray

**Difficulty:** Medium
**Tags:** Array, Binary Search, Dynamic Programming, Sliding Window, Rolling Hash, Hash Function

---

## Problem

<p>Given two integer arrays <code>nums1</code> and <code>nums2</code>, return <em>the maximum length of a subarray that appears in <strong>both</strong> arrays</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The repeated subarray with maximum length is [3,2,1].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The repeated subarray with maximum length is [0,0,0,0,0].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 1000</code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 100</code></li>
</ul>


## Hints

1. Use dynamic programming.  dp[i][j] will be the longest common prefix of A[i:] and B[j:].
2. The answer is max(dp[i][j]) over all i, j.

## Solution

```rust
impl Solution { pub fn find_length(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 { let mut black_dp = vec![0; black_n2.len() + 1]; let mut black_max = 0; for &x in &black_n1 { for j in (1..=black_n2.len()).rev() { if x == black_n2[j - 1] { black_dp[j] = black_dp[j - 1] + 1; black_max = black_max.max(black_dp[j]); } else { black_dp[j] = 0; } } } black_max } }
```