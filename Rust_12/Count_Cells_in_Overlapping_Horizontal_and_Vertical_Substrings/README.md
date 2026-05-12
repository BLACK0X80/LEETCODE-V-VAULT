# Count Cells in Overlapping Horizontal and Vertical Substrings

**Difficulty:** Medium
**Tags:** Array, String, Rolling Hash, String Matching, Matrix, Hash Function

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>grid</code> consisting of characters and a string <code>pattern</code>.</p>

<p>A <strong data-end="264" data-start="240">horizontal substring</strong> is a contiguous sequence of characters read from left to right. If the end of a row is reached before the substring is complete, it wraps to the first column of the next row and continues as needed. You do <strong>not</strong> wrap from the bottom row back to the top.</p>

<p>A <strong data-end="484" data-start="462">vertical substring</strong> is a contiguous sequence of characters read from top to bottom. If the bottom of a column is reached before the substring is complete, it wraps to the first row of the next column and continues as needed. You do <strong>not</strong> wrap from the last column back to the first.</p>

<p>Count the number of cells in the matrix that satisfy the following condition:</p>

<ul>
	<li>The cell must be part of <strong>at least</strong> one horizontal substring and <strong>at least</strong> one vertical substring, where <strong>both</strong> substrings are equal to the given <code>pattern</code>.</li>
</ul>

<p>Return the count of these cells.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2025/03/03/gridtwosubstringsdrawio.png" style="width: 150px; height: 187px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;a&quot;,&quot;a&quot;,&quot;c&quot;,&quot;c&quot;],[&quot;b&quot;,&quot;b&quot;,&quot;b&quot;,&quot;c&quot;],[&quot;a&quot;,&quot;a&quot;,&quot;b&quot;,&quot;a&quot;],[&quot;c&quot;,&quot;a&quot;,&quot;a&quot;,&quot;c&quot;],[&quot;a&quot;,&quot;a&quot;,&quot;b&quot;,&quot;a&quot;]], pattern = &quot;abaca&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The pattern <code>&quot;abaca&quot;</code> appears once as a horizontal substring (colored blue) and once as a vertical substring (colored red), intersecting at one cell (colored purple).</p>
</div>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2025/03/03/gridexample2fixeddrawio.png" style="width: 150px; height: 150px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;c&quot;,&quot;a&quot;,&quot;a&quot;,&quot;a&quot;],[&quot;a&quot;,&quot;a&quot;,&quot;b&quot;,&quot;a&quot;],[&quot;b&quot;,&quot;b&quot;,&quot;a&quot;,&quot;a&quot;],[&quot;a&quot;,&quot;a&quot;,&quot;b&quot;,&quot;a&quot;]], pattern = &quot;aba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The cells colored above are all part of at least one horizontal and one vertical substring matching the pattern <code>&quot;aba&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;a&quot;]], pattern = &quot;a&quot;</span></p>

<p><strong>Output:</strong> 1</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= pattern.length &lt;= m * n</code></li>
	<li><code>grid</code> and <code>pattern</code> consist of only lowercase English letters.</li>
</ul>


## Hints

1. Use a string hashing or pattern matching algorithm to efficiently find all horizontal and vertical occurrences of the pattern in the grid.
2. Track the positions of each match and count only the cells that appear in both horizontal and vertical matches.

## Solution

```rust
impl Solution { pub fn count_cells(black_g: Vec<Vec<char>>, black_p: String) -> i32 { let (black_m, black_n, black_l) = (black_g.len(), black_g[0].len(), black_p.len()); let black_pat = black_p.as_bytes(); let mut black_pi = vec![0; black_l]; for i in 1..black_l { let mut j = black_pi[i-1]; while j > 0 && black_pat[i] != black_pat[j] { j = black_pi[j-1]; } if black_pat[i] == black_pat[j] { j += 1; } black_pi[i] = j; } let (mut black_h_d, mut black_v_d) = (vec![0i32; black_m * black_n + 1], vec![0i32; black_m * black_n + 1]); let (mut black_h_f, mut black_v_f) = (Vec::with_capacity(black_m * black_n), Vec::with_capacity(black_m * black_n)); for r in 0..black_m { for c in 0..black_n { black_h_f.push(black_g[r][c] as u8); } } for c in 0..black_n { for r in 0..black_m { black_v_f.push(black_g[r][c] as u8); } } let mut j = 0; for i in 0..black_h_f.len() { while j > 0 && black_h_f[i] != black_pat[j] { j = black_pi[j-1]; } if black_h_f[i] == black_pat[j] { j += 1; } if j == black_l { black_h_d[i + 1 - black_l] += 1; black_h_d[i + 1] -= 1; j = black_pi[j-1]; } } j = 0; for i in 0..black_v_f.len() { while j > 0 && black_v_f[i] != black_pat[j] { j = black_pi[j-1]; } if black_v_f[i] == black_pat[j] { j += 1; } if j == black_l { black_v_d[i + 1 - black_l] += 1; black_v_d[i + 1] -= 1; j = black_pi[j-1]; } } let (mut black_res, mut black_ch, mut black_cv) = (0, 0, 0); let mut black_v_m = vec![false; black_m * black_n]; for i in 0..black_m * black_n { black_cv += black_v_d[i]; if black_cv > 0 { black_v_m[(i % black_m) * black_n + (i / black_m)] = true; } } for i in 0..black_m * black_n { black_ch += black_h_d[i]; if black_ch > 0 && black_v_m[i] { black_res += 1; } } black_res } }
```