# Regions Cut By Slashes

**Difficulty:** Medium
**Tags:** Array, Hash Table, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>An <code>n x n</code> grid is composed of <code>1 x 1</code> squares where each <code>1 x 1</code> square consists of a <code>&#39;/&#39;</code>, <code>&#39;\&#39;</code>, or blank space <code>&#39; &#39;</code>. These characters divide the square into contiguous regions.</p>

<p>Given the grid <code>grid</code> represented as a string array, return <em>the number of regions</em>.</p>

<p>Note that backslash characters are escaped, so a <code>&#39;\&#39;</code> is represented as <code>&#39;\\&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/1.png" style="width: 200px; height: 200px;" />
<pre>
<strong>Input:</strong> grid = [&quot; /&quot;,&quot;/ &quot;]
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/2.png" style="width: 200px; height: 198px;" />
<pre>
<strong>Input:</strong> grid = [&quot; /&quot;,&quot;  &quot;]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/4.png" style="width: 200px; height: 200px;" />
<pre>
<strong>Input:</strong> grid = [&quot;/\\&quot;,&quot;\\/&quot;]
<strong>Output:</strong> 5
<strong>Explanation: </strong>Recall that because \ characters are escaped, &quot;\\/&quot; refers to \/, and &quot;/\\&quot; refers to /\.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 30</code></li>
	<li><code>grid[i][j]</code> is either <code>&#39;/&#39;</code>, <code>&#39;\&#39;</code>, or <code>&#39; &#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn regions_by_slashes(black_g: Vec<String>) -> i32 { let black_n = black_g.len(); let mut black_f: Vec<usize> = (0..4*black_n*black_n).collect(); let mut black_count = 4 * black_n * black_n; for black_r in 0..black_n { for (black_c, black_b) in black_g[black_r].bytes().enumerate() { let black_root = 4 * (black_r * black_n + black_c); match black_b { b' ' => { black_union(black_root, black_root+1, &mut black_f, &mut black_count); black_union(black_root+1, black_root+2, &mut black_f, &mut black_count); black_union(black_root+2, black_root+3, &mut black_f, &mut black_count); }, b'/' => { black_union(black_root, black_root+3, &mut black_f, &mut black_count); black_union(black_root+1, black_root+2, &mut black_f, &mut black_count); }, _ => { black_union(black_root, black_root+1, &mut black_f, &mut black_count); black_union(black_root+2, black_root+3, &mut black_f, &mut black_count); } } if black_r + 1 < black_n { black_union(black_root+2, black_root+4*black_n, &mut black_f, &mut black_count); } if black_c + 1 < black_n { black_union(black_root+1, black_root+7, &mut black_f, &mut black_count); } } } black_count as i32 } } fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } fn black_union(i: usize, j: usize, f: &mut Vec<usize>, c: &mut usize) { let (r1, r2) = (black_find(i, f), black_find(j, f)); if r1 != r2 { f[r1] = r2; *c -= 1; } }
```