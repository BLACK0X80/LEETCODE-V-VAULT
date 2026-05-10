# Grid Teleportation Traversal

**Difficulty:** Medium
**Tags:** Array, Hash Table, Breadth-First Search, Matrix

---

## Problem

<p>You are given a 2D character grid <code>matrix</code> of size <code>m x n</code>, represented as an array of strings, where <code>matrix[i][j]</code> represents the cell at the intersection of the <code>i<sup>th</sup></code> row and <code>j<sup>th</sup></code> column. Each cell is one of the following:</p>

<ul>
	<li><code>&#39;.&#39;</code> representing an empty cell.</li>
	<li><code>&#39;#&#39;</code> representing an obstacle.</li>
	<li>An uppercase letter (<code>&#39;A&#39;</code>-<code>&#39;Z&#39;</code>) representing a teleportation portal.</li>
</ul>

<p>You start at the top-left cell <code>(0, 0)</code>, and your goal is to reach the bottom-right cell <code>(m - 1, n - 1)</code>. You can move from the current cell to any adjacent cell (up, down, left, right) as long as the destination cell is within the grid bounds and is not an obstacle<strong>.</strong></p>

<p>If you step on a cell containing a portal letter and you haven&#39;t used that portal letter before, you may instantly teleport to any other cell in the grid with the same letter. This teleportation does not count as a move, but each portal letter can be used<strong> at most </strong>once during your journey.</p>

<p>Return the <strong>minimum</strong> number of moves required to reach the bottom-right cell. If it is not possible to reach the destination, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">matrix = [&quot;A..&quot;,&quot;.A.&quot;,&quot;...&quot;]</span></p>

<p><strong>Output:</strong> 2</p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/15/example04140.png" style="width: 151px; height: 151px;" /></p>

<ul>
	<li>Before the first move, teleport from <code>(0, 0)</code> to <code>(1, 1)</code>.</li>
	<li>In the first move, move from <code>(1, 1)</code> to <code>(1, 2)</code>.</li>
	<li>In the second move, move from <code>(1, 2)</code> to <code>(2, 2)</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">matrix = [&quot;.#...&quot;,&quot;.#.#.&quot;,&quot;.#.#.&quot;,&quot;...#.&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">13</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/15/ezgifcom-animated-gif-maker.gif" style="width: 251px; height: 201px;" /></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == matrix.length &lt;= 10<sup>3</sup></code></li>
	<li><code>1 &lt;= n == matrix[i].length &lt;= 10<sup>3</sup></code></li>
	<li><code>matrix[i][j]</code> is either <code>&#39;#&#39;</code>, <code>&#39;.&#39;</code>, or an uppercase English letter.</li>
	<li><code>matrix[0][0]</code> is not an obstacle.</li>
</ul>


## Hints

1. Treat all portals with the same letter as connected-like one big super-node.
2. Each portal letter is used at most once, but that doesn't affect correctness since we visit each cell only once in the shortest path.
3. Use Breadth-First Search to find the minimum number of moves.

## Solution

```rust
use std::collections::{VecDeque, HashMap}; impl Solution { pub fn min_moves(black_mat: Vec<String>) -> i32 { let (black_m, black_n) = (black_mat.len(), black_mat[0].len()); let mut black_p: HashMap<u8, Vec<(usize, usize)>> = HashMap::new(); for r in 0..black_m { for c in 0..black_n { let b = black_mat[r].as_bytes()[c]; if b.is_ascii_uppercase() { black_p.entry(b).or_default().push((r, c)); } } } let mut black_q = VecDeque::from([(0, 0, 0)]); let mut black_v = vec![vec![false; black_n]; black_m]; let mut black_pu = [false; 26]; while let Some((r, c, black_st)) = black_q.pop_front() { if r == black_m - 1 && c == black_n - 1 { return black_st; } if black_v[r][c] { continue; } black_v[r][c] = true; let b = black_mat[r].as_bytes()[c]; if b.is_ascii_uppercase() { let i = (b - b'A') as usize; if !black_pu[i] { black_pu[i] = true; for &(nr, nc) in &black_p[&b] { if !black_v[nr][nc] { black_q.push_front((nr, nc, black_st)); } } } } for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < black_m as i32 && nc >= 0 && nc < black_n as i32 { let (nr, nc) = (nr as usize, nc as usize); if black_mat[nr].as_bytes()[nc] != b'#' && !black_v[nr][nc] { black_q.push_back((nr, nc, black_st + 1)); } } } } -1 } }
```