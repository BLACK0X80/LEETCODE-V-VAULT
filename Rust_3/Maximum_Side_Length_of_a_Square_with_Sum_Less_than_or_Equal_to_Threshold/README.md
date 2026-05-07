# Maximum Side Length of a Square with Sum Less than or Equal to Threshold

**Difficulty:** Medium
**Tags:** Array, Binary Search, Matrix, Prefix Sum

---

## Problem

<p>Given a <code>m x n</code> matrix <code>mat</code> and an integer <code>threshold</code>, return <em>the maximum side-length of a square with a sum less than or equal to </em><code>threshold</code><em> or return </em><code>0</code><em> if there is no such square</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/05/e1.png" style="width: 335px; height: 186px;" />
<pre>
<strong>Input:</strong> mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> The maximum side length of square with sum less than or equal to 4 is 2 as shown.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n == mat[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 300</code></li>
	<li><code>0 &lt;= mat[i][j] &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= threshold &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Store prefix sum of all grids in another 2D array.
2. Try all possible solutions and if you cannot find one return 0.
3. If x is a valid answer then any y < x is also valid answer. Use binary search to find answer.

## Solution

```rust
impl Solution { pub fn max_side_length(black_mat: Vec<Vec<i32>>, black_t: i32) -> i32 { let (black_m, black_n) = (black_mat.len(), black_mat[0].len()); let mut black_p = vec![vec![0; black_n + 1]; black_m + 1]; for black_i in 1..=black_m { for black_j in 1..=black_n { black_p[black_i][black_j] = black_mat[black_i-1][black_j-1] + black_p[black_i-1][black_j] + black_p[black_i][black_j-1] - black_p[black_i-1][black_j-1]; } } let (mut black_ans, mut black_k) = (0, 1); for black_i in 1..=black_m { for black_j in 1..=black_n { while black_i >= black_k && black_j >= black_k && black_p[black_i][black_j] - black_p[black_i-black_k][black_j] - black_p[black_i][black_j-black_k] + black_p[black_i-black_k][black_j-black_k] <= black_t { black_ans = black_k as i32; black_k += 1; } } } black_ans } }
```