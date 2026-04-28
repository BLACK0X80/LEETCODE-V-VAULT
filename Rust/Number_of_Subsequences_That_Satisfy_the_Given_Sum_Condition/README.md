# Number of Subsequences That Satisfy the Given Sum Condition

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting

---

## Problem

<p>You are given an array of integers <code>nums</code> and an integer <code>target</code>.</p>

<p>Return <em>the number of <strong>non-empty</strong> subsequences of </em><code>nums</code><em> such that the sum of the minimum and maximum element on it is less or equal to </em><code>target</code>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,5,6,7], target = 9
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 subsequences that satisfy the condition.
[3] -&gt; Min value + max value &lt;= target (3 + 3 &lt;= 9)
[3,5] -&gt; (3 + 5 &lt;= 9)
[3,5,6] -&gt; (3 + 6 &lt;= 9)
[3,6] -&gt; (3 + 6 &lt;= 9)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,3,6,8], target = 10
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
[3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,3,4,6,7], target = 12
<strong>Output:</strong> 61
<strong>Explanation:</strong> There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
Number of valid subsequences (63 - 2 = 61).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= target &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Sort the array nums.
2. Use two pointers approach: Given an index i (choose it as the minimum in a subsequence) find the maximum j where j ≥ i and nums[i] +nums[j] ≤ target.
3. Count the number of subsequences.

## Solution

```rust
impl Solution { pub fn num_subseq(mut black_n: Vec<i32>, black_t: i32) -> i32 { black_n.sort_unstable(); let (black_mod, mut black_res, mut black_p) = (1_000_000_007, 0, vec![1; black_n.len()]); for black_i in 1..black_n.len() { black_p[black_i] = (black_p[black_i - 1] * 2) % black_mod; } let (mut black_l, mut black_r) = (0, black_n.len() as i32 - 1); while black_l <= black_r { if black_n[black_l as usize] + black_n[black_r as usize] <= black_t { black_res = (black_res + black_p[(black_r - black_l) as usize]) % black_mod; black_l += 1; } else { black_r -= 1; } } black_res } }
```