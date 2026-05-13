# Get Biggest Three Rhombus Sums in a Grid

**Difficulty:** Medium
**Tags:** Array, Math, Sorting, Heap (Priority Queue), Matrix, Prefix Sum

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>grid</code>​​​.</p>

<p>A <strong>rhombus sum</strong> is the sum of the elements that form <strong>the</strong> <strong>border</strong> of a regular rhombus shape in <code>grid</code>​​​. The rhombus must have the shape of a square rotated 45 degrees with each of the corners centered in a grid cell. Below is an image of four valid rhombus shapes with the corresponding colored cells that should be included in each <strong>rhombus sum</strong>:</p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-desc-2.png" style="width: 385px; height: 385px;" />
<p>Note that the rhombus can have an area of 0, which is depicted by the purple rhombus in the bottom right corner.</p>

<p>Return <em>the biggest three <strong>distinct rhombus sums</strong> in the </em><code>grid</code><em> in <strong>descending order</strong></em><em>. If there are less than three distinct values, return all of them</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-ex1.png" style="width: 360px; height: 361px;" />
<pre>
<strong>Input:</strong> grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]]
<strong>Output:</strong> [228,216,211]
<strong>Explanation:</strong> The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
- Blue: 20 + 3 + 200 + 5 = 228
- Red: 200 + 2 + 10 + 4 = 216
- Green: 5 + 200 + 4 + 2 = 211
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-ex2.png" style="width: 217px; height: 217px;" />
<pre>
<strong>Input:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> [20,9,8]
<strong>Explanation:</strong> The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
- Blue: 4 + 2 + 6 + 8 = 20
- Red: 9 (area 0 rhombus in the bottom right corner)
- Green: 8 (area 0 rhombus in the bottom middle)
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[7,7,7]]
<strong>Output:</strong> [7]
<strong>Explanation:</strong> All three possible rhombus sums are the same, so return [7].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. You need to maintain only the biggest 3 distinct sums
2. The limits are small enough for you to iterate over all rhombus sizes then iterate over all possible borders to get the sums

## Solution

```rust
impl Solution { pub fn get_biggest_three(black_g: Vec<Vec<i32>>) -> Vec<i32> { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let mut black_res = std::collections::BTreeSet::new(); for black_i in 0..black_m { for black_j in 0..black_n { black_res.insert(black_g[black_i][black_j]); for black_l in 1..50 { if black_i >= black_l && black_i + black_l < black_m && black_j >= black_l && black_j + black_l < black_n { let mut black_s = 0; for black_k in 0..black_l { black_s += black_g[black_i - black_l + black_k][black_j + black_k] + black_g[black_i + black_k][black_j + black_l - black_k] + black_g[black_i + black_l - black_k][black_j - black_k] + black_g[black_i - black_k][black_j - black_l + black_k]; } black_res.insert(black_s); } else { break; } } } } black_res.into_iter().rev().take(3).collect() } }
```