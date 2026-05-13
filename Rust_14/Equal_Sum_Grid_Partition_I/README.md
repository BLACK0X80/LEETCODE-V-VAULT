# Equal Sum Grid Partition I

**Difficulty:** Medium
**Tags:** Array, Matrix, Enumeration, Prefix Sum

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>grid</code> of positive integers. Your task is to determine if it is possible to make <strong>either one horizontal or one vertical cut</strong> on the grid such that:</p>

<ul>
	<li>Each of the two resulting sections formed by the cut is <strong>non-empty</strong>.</li>
	<li>The sum of the elements in both sections is <strong>equal</strong>.</li>
</ul>

<p>Return <code>true</code> if such a partition exists; otherwise return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,4],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/30/lc.png" style="width: 200px;" /><img alt="" src="https://assets.leetcode.com/uploads/2025/03/30/lc.jpeg" style="width: 200px; height: 200px;" /></p>

<p>A horizontal cut between row 0 and row 1 results in two non-empty sections, each with a sum of 5. Thus, the answer is <code>true</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,3],[2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p>No horizontal or vertical cut results in two non-empty sections with equal sums. Thus, the answer is <code>false</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == grid.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= n == grid[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. There are two types of cuts: a <code>horizontal</code> cut or a <code>vertical</code> cut.
2. For a <code>horizontal</code> cut at row <code>r</code> (0 <= r <m - 1), split <code>grid</code> into rows 0...r vs. r+1...m-1 and compare their sums.
3. For a <code>vertical</code> cut at column <code>c</code> (0 <= c < n - 1), split <code>grid</code> into columns 0...c vs. c+1...n-1 and compare their sums.
4. Brute‑force all possible <code>r</code> and <code>c</code> cuts; if any yields equal section sums, return <code>true</code>.

## Solution

```rust
impl Solution { pub fn can_partition_grid(black_g: Vec<Vec<i32>>) -> bool { let black_m = black_g.len(); let black_n = black_g[0].len(); let black_tot: i64 = black_g.iter().map(|r| r.iter().map(|&v| v as i64).sum::<i64>()).sum(); if black_tot % 2 != 0 { return false; } let black_target = black_tot / 2; let mut black_curr = 0i64; for i in 0..black_m - 1 { black_curr += black_g[i].iter().map(|&v| v as i64).sum::<i64>(); if black_curr == black_target { return true; } } black_curr = 0; for j in 0..black_n - 1 { for i in 0..black_m { black_curr += black_g[i][j] as i64; } if black_curr == black_target { return true; } } false } }
```