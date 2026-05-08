# Maximum Strictly Increasing Cells in a Matrix

**Difficulty:** Hard
**Tags:** Array, Hash Table, Binary Search, Dynamic Programming, Memoization, Sorting, Matrix, Ordered Set

---

## Problem

<p>Given a <strong>1-indexed</strong>&nbsp;<code>m x n</code> integer matrix <code>mat</code>, you can select any cell in the matrix as your <strong>starting cell</strong>.</p>

<p>From the starting cell, you can move to any other cell <strong>in the</strong> <strong>same row or column</strong>, but only if the value of the destination cell is <strong>strictly greater</strong> than the value of the current cell. You can repeat this process as many times as possible, moving from cell to cell until you can no longer make any moves.</p>

<p>Your task is to find the <strong>maximum number of cells</strong> that you can visit in the matrix by starting from some cell.</p>

<p>Return <em>an integer denoting the maximum number of cells that can be visited.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag1drawio.png" style="width: 200px; height: 176px;" /></strong></p>

<pre>
<strong>Input:</strong> mat = [[3,1],[3,4]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The image shows how we can visit 2 cells starting from row 1, column 2. It can be shown that we cannot visit more than 2 cells no matter where we start from, so the answer is 2. 
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag3drawio.png" style="width: 200px; height: 176px;" /></strong></p>

<pre>
<strong>Input:</strong> mat = [[1,1],[1,1]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Since the cells must be strictly increasing, we can only visit one cell in this example. 
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag4drawio.png" style="width: 350px; height: 250px;" /></strong></p>

<pre>
<strong>Input:</strong> mat = [[3,1,6],[-9,5,7]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The image above shows how we can visit 4 cells starting from row 2, column 1. It can be shown that we cannot visit more than 4 cells no matter where we start from, so the answer is 4. 
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length&nbsp;</code></li>
	<li><code>n == mat[i].length&nbsp;</code></li>
	<li><code>1 &lt;= m, n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup>&nbsp;&lt;= mat[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. We can try to build the answer in a bottom-up fashion, starting from the smallest values and increasing to the larger values.
2. Going through the values in sorted order, we can store the maximum path we have seen so far for a row/column.
3. When we are at a cell, we check its row and column to find out the best previous smaller value that we’ve got so far, and we use it to increment the current value of the row and column.

## Solution

```rust
use std::collections::BTreeMap;
impl Solution {
    pub fn max_increasing_cells(black1: Vec<Vec<i32>>) -> i32 {
        let (black2, black3) = (black1.len(), black1[0].len());
        let mut black4: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for black5 in 0..black2 {
            for black6 in 0..black3 { black4.entry(black1[black5][black6]).or_default().push((black5, black6)); }
        }
        let (mut black7, mut black8) = (vec![0; black2], vec![0; black3]);
        for (_, black9) in black4 {
            let mut black10 = Vec::new();
            for &(black11, black12) in &black9 { black10.push(black7[black11].max(black8[black12]) + 1); }
            for (black13, &(black14, black15)) in black9.iter().enumerate() {
                black7[black14] = black7[black14].max(black10[black13]);
                black8[black15] = black8[black15].max(black10[black13]);
            }
        }
        *black7.iter().max().unwrap().max(black8.iter().max().unwrap())
    }
}
```