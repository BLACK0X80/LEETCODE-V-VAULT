# Longest Square Streak in an Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Dynamic Programming, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code>. A subsequence of <code>nums</code> is called a <strong>square streak</strong> if:</p>

<ul>
	<li>The length of the subsequence is at least <code>2</code>, and</li>
	<li><strong>after</strong> sorting the subsequence, each element (except the first element) is the <strong>square</strong> of the previous number.</li>
</ul>

<p>Return<em> the length of the <strong>longest square streak</strong> in </em><code>nums</code><em>, or return </em><code>-1</code><em> if there is no <strong>square streak</strong>.</em></p>

<p>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,3,6,16,8,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Choose the subsequence [4,16,2]. After sorting it, it becomes [2,4,16].
- 4 = 2 * 2.
- 16 = 4 * 4.
Therefore, [4,16,2] is a square streak.
It can be shown that every subsequence of length 4 is not a square streak.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,5,6,7]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no square streak in nums so return -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. With the constraints, the length of the longest square streak possible is 5.
2. Store the elements of nums in a set to quickly check if it exists.

## Solution

```rust
impl Solution { pub fn longest_square_streak(black_nums: Vec<i32>) -> i32 { let black_set: std::collections::HashSet<i64> = black_nums.into_iter().map(|black_x| black_x as i64).collect(); let mut black_max = -1; for &black_n in &black_set { let (mut black_curr, mut black_len) = (black_n, 1); while let Some(_) = black_set.get(&(black_curr * black_curr)) { black_curr *= black_curr; black_len += 1; } if black_len >= 2 { black_max = black_max.max(black_len); } } black_max } }
```