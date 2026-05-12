# Maximum Number of Operations With the Same Score II

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Memoization

---

## Problem

<p>Given an array of integers called <code>nums</code>, you can perform <strong>any</strong> of the following operation while <code>nums</code> contains <strong>at least</strong> <code>2</code> elements:</p>

<ul>
	<li>Choose the first two elements of <code>nums</code> and delete them.</li>
	<li>Choose the last two elements of <code>nums</code> and delete them.</li>
	<li>Choose the first and the last elements of <code>nums</code> and delete them.</li>
</ul>

<p>The<strong> score</strong> of the operation is the sum of the deleted elements.</p>

<p>Your task is to find the <strong>maximum</strong> number of operations that can be performed, such that <strong>all operations have the same score</strong>.</p>

<p>Return <em>the <strong>maximum</strong> number of operations possible that satisfy the condition mentioned above</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,1,2,3,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We perform the following operations:
- Delete the first two elements, with score 3 + 2 = 5, nums = [1,2,3,4].
- Delete the first and the last elements, with score 1 + 4 = 5, nums = [2,3].
- Delete the first and the last elements, with score 2 + 3 = 5, nums = [].
We are unable to perform any more operations as nums is empty.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,6,1,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We perform the following operations:
- Delete the first two elements, with score 3 + 2 = 5, nums = [6,1,4].
- Delete the last two elements, with score 1 + 4 = 5, nums = [6].
It can be proven that we can perform at most 2 operations.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 2000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 1000</code></li>
</ul>


## Hints

1. After the first operation, the score of other operations is fixed.
2. For the fixed score use dynamic programming <code>dp[l][r]</code> to find a maximum number of operations on the subarray <code>nums[l..r]</code>.

## Solution

```rust
impl Solution { pub fn max_operations(nums: Vec<i32>) -> i32 { fn black_solve(l: usize, r: usize, s: i32, n: &[i32], m: &mut Vec<Vec<i32>>) -> i32 { if l >= r { return 0; } if m[l][r] != -1 { return m[l][r]; } let mut res = 0; if n[l] + n[l+1] == s { res = res.max(1 + black_solve(l+2, r, s, n, m)); } if r >= 1 && n[r-1] + n[r] == s { res = res.max(1 + black_solve(l, r.saturating_sub(2), s, n, m)); } if n[l] + n[r] == s { res = res.max(1 + black_solve(l+1, r.saturating_sub(1), s, n, m)); } m[l][r] = res; res } let n_len = nums.len(); let mut res = 0; for s in [nums[0]+nums[1], nums[n_len-2]+nums[n_len-1], nums[0]+nums[n_len-1]] { res = res.max(black_solve(0, n_len-1, s, &nums, &mut vec![vec![-1; n_len]; n_len])); } res } }
```