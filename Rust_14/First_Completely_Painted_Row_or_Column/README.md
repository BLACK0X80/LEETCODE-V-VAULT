# First Completely Painted Row or Column

**Difficulty:** Medium
**Tags:** Array, Hash Table, Matrix

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>arr</code>, and an <code>m x n</code> integer <strong>matrix</strong> <code>mat</code>. <code>arr</code> and <code>mat</code> both contain <strong>all</strong> the integers in the range <code>[1, m * n]</code>.</p>

<p>Go through each index <code>i</code> in <code>arr</code> starting from index <code>0</code> and paint the cell in <code>mat</code> containing the integer <code>arr[i]</code>.</p>

<p>Return <em>the smallest index</em> <code>i</code> <em>at which either a row or a column will be completely painted in</em> <code>mat</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="image explanation for example 1" /><img alt="image explanation for example 1" src="https://assets.leetcode.com/uploads/2023/01/18/grid1.jpg" style="width: 321px; height: 81px;" />
<pre>
<strong>Input:</strong> arr = [1,3,4,2], mat = [[1,4],[2,3]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The moves are shown in order, and both the first row and second column of the matrix become fully painted at arr[2].
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="image explanation for example 2" src="https://assets.leetcode.com/uploads/2023/01/18/grid2.jpg" style="width: 601px; height: 121px;" />
<pre>
<strong>Input:</strong> arr = [2,8,7,4,1,3,5,6,9], mat = [[3,2,5],[1,4,6],[8,7,9]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The second column becomes fully painted at arr[3].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n = mat[i].length</code></li>
	<li><code>arr.length == m * n</code></li>
	<li><code>1 &lt;= m, n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arr[i], mat[r][c] &lt;= m * n</code></li>
	<li>All the integers of <code>arr</code> are <strong>unique</strong>.</li>
	<li>All the integers of <code>mat</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. Can we use a frequency array?
2. Pre-process the positions of the values in the matrix.
3. Traverse the array and increment the corresponding row and column frequency using the pre-processed positions.
4. If the row frequency becomes equal to the number of columns, or vice-versa return the current index.

## Solution

```rust
impl Solution { pub fn first_complete_index(black_arr: Vec<i32>, black_mat: Vec<Vec<i32>>) -> i32 { let (black_m, black_n) = (black_mat.len(), black_mat[0].len()); let mut black_pos = vec![(0, 0); black_m * black_n + 1]; for i in 0..black_m { for j in 0..black_n { black_pos[black_mat[i][j] as usize] = (i, j); } } let (mut black_row_cnt, mut black_col_cnt) = (vec![0; black_m], vec![0; black_n]); for (i, &black_val) in black_arr.iter().enumerate() { let (r, c) = black_pos[black_val as usize]; black_row_cnt[r] += 1; black_col_cnt[c] += 1; if black_row_cnt[r] == black_n || black_col_cnt[c] == black_m { return i as i32; } } 0 } }
```