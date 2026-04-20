# Minimum Falling Path Sum II

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>Given an <code>n x n</code> integer matrix <code>grid</code>, return <em>the minimum sum of a <strong>falling path with non-zero shifts</strong></em>.</p>

<p>A <strong>falling path with non-zero shifts</strong> is a choice of exactly one element from each row of <code>grid</code> such that no two elements chosen in adjacent rows are in the same column.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/falling-grid.jpg" style="width: 244px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> 13
<strong>Explanation:</strong> 
The possible falling paths are:
[1,5,9], [1,5,7], [1,6,7], [1,6,8],
[2,4,8], [2,4,9], [2,6,7], [2,6,8],
[3,4,8], [3,4,9], [3,5,7], [3,5,9]
The falling path with the smallest sum is&nbsp;[1,5,7], so the answer is&nbsp;13.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[7]]
<strong>Output:</strong> 7
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 200</code></li>
	<li><code>-99 &lt;= grid[i][j] &lt;= 99</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let dp[i][j] be the answer for the first i rows such that column j is chosen from row i.
3. Use the concept of cumulative array to optimize the complexity of the solution.

## Solution

```rust
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut prev = grid[0].clone();
        for i in 1..n {
            let mut min1 = (i32::MAX, usize::MAX);
            let mut min2 = (i32::MAX, usize::MAX);
            for j in 0..n {
                if prev[j] < min1.0 { min2=min1; min1=(prev[j],j); }
                else if prev[j] < min2.0 { min2=(prev[j],j); }
            }
            let mut cur = vec![0; n];
            for j in 0..n {
                cur[j] = grid[i][j] + if j!=min1.1 { min1.0 } else { min2.0 };
            }
            prev = cur;
        }
        *prev.iter().min().unwrap()
    }
}
```