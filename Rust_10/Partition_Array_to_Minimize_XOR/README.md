# Partition Array to Minimize XOR

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Bit Manipulation, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>Your task is to partition <code>nums</code> into <code>k</code><strong> </strong>non-empty <strong><span data-keyword="subarray-nonempty">subarrays</span></strong>. For each subarray, compute the bitwise <strong>XOR</strong> of all its elements.</p>

<p>Return the <strong>minimum</strong> possible value of the <strong>maximum XOR</strong> among these <code>k</code> subarrays.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal partition is <code>[1]</code> and <code>[2, 3]</code>.</p>

<ul>
	<li>XOR of the first subarray is <code>1</code>.</li>
	<li>XOR of the second subarray is <code>2 XOR 3 = 1</code>.</li>
</ul>

<p>The maximum XOR among the subarrays is 1, which is the minimum possible.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,3,2], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal partition is <code>[2]</code>, <code>[3, 3]</code>, and <code>[2]</code>.</p>

<ul>
	<li>XOR of the first subarray is <code>2</code>.</li>
	<li>XOR of the second subarray is <code>3 XOR 3 = 0</code>.</li>
	<li>XOR of the third subarray is <code>2</code>.</li>
</ul>

<p>The maximum XOR among the subarrays is 2, which is the minimum possible.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,3,1], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal partition is <code>[1, 1]</code> and <code>[2, 3, 1]</code>.</p>

<ul>
	<li>XOR of the first subarray is <code>1 XOR 1 = 0</code>.</li>
	<li>XOR of the second subarray is <code>2 XOR 3 XOR 1 = 0</code>.</li>
</ul>

<p>The maximum XOR among the subarrays is 0, which is the minimum possible.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 250</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Precompute <code>pre[i] = nums[0] ^ … ^ nums[i-1]</code> so any subarray XOR is <code>pre[r] ^ pre[l]</code>.
3. Define <code>dp[i][j]</code> = minimum possible “max‑XOR” when splitting the first <code>i</code> elements into <code>j</code> parts.
4. For each <code>dp[i][j]</code>, try all splits <code>t < i</code> and take the minimum over <code>max(dp[t][j-1], pre[i] ^ pre[t])</code>.

## Solution

```rust
impl Solution { pub fn min_xor(black_nums: Vec<i32>, black_k: i32) -> i32 { let black_n = black_nums.len(); let mut black_dp = vec![vec![i32::MAX; black_k as usize + 1]; black_n + 1]; black_dp[0][0] = 0; for black_j in 1..=black_k as usize { for black_i in 1..=black_n { let mut black_x = 0; for black_p in (0..black_i).rev() { black_x ^= black_nums[black_p]; if black_dp[black_p][black_j - 1] != i32::MAX { black_dp[black_i][black_j] = black_dp[black_i][black_j].min(black_dp[black_p][black_j - 1].max(black_x)); } } } } black_dp[black_n][black_k as usize] } }
```