# Minimize the Difference Between Target and Chosen Elements

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>mat</code> and an integer <code>target</code>.</p>

<p>Choose one integer from <strong>each row</strong> in the matrix such that the <strong>absolute difference</strong> between <code>target</code> and the <strong>sum</strong> of the chosen elements is <strong>minimized</strong>.</p>

<p>Return <em>the <strong>minimum absolute difference</strong></em>.</p>

<p>The <strong>absolute difference</strong> between two numbers <code>a</code> and <code>b</code> is the absolute value of <code>a - b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1.png" style="width: 181px; height: 181px;" />
<pre>
<strong>Input:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], target = 13
<strong>Output:</strong> 0
<strong>Explanation:</strong> One possible choice is to:
- Choose 1 from the first row.
- Choose 5 from the second row.
- Choose 7 from the third row.
The sum of the chosen elements is 13, which equals the target, so the absolute difference is 0.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1-1.png" style="width: 61px; height: 181px;" />
<pre>
<strong>Input:</strong> mat = [[1],[2],[3]], target = 100
<strong>Output:</strong> 94
<strong>Explanation:</strong> The best possible choice is to:
- Choose 1 from the first row.
- Choose 2 from the second row.
- Choose 3 from the third row.
The sum of the chosen elements is 6, and the absolute difference is 94.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1-3.png" style="width: 301px; height: 61px;" />
<pre>
<strong>Input:</strong> mat = [[1,2,9,8,7]], target = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> The best choice is to choose 7 from the first row.
The absolute difference is 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n == mat[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 70</code></li>
	<li><code>1 &lt;= mat[i][j] &lt;= 70</code></li>
	<li><code>1 &lt;= target &lt;= 800</code></li>
</ul>


## Hints

1. The sum of chosen elements will not be too large. Consider using a hash set to record all possible sums while iterating each row.
2. Instead of keeping track of all possible sums, since in each row, we are adding positive numbers, only keep those that can be a candidate, not exceeding the target by too much.

## Solution

```rust
impl Solution { pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 { let mut black_sums = std::collections::HashSet::new(); black_sums.insert(0); for black_row in mat { let mut black_next = std::collections::HashSet::new(); for &black_v in &black_row { for &black_s in &black_sums { black_next.insert(black_s + black_v); } } let black_min_over = black_next.iter().filter(|&&s| s > target).min().cloned(); black_sums = black_next.into_iter().filter(|&s| s <= target).collect(); if let Some(black_m) = black_min_over { black_sums.insert(black_m); } } black_sums.into_iter().map(|s| (target - s).abs()).min().unwrap() } }
```