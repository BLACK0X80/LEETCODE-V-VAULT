# Game of Life

**Difficulty:** Medium
**Tags:** Array, Matrix, Simulation

---

## Problem

<p>According to <a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life" target="_blank">Wikipedia&#39;s article</a>: &quot;The <b>Game of Life</b>, also known simply as <b>Life</b>, is a cellular automaton devised by the British mathematician John Horton Conway in 1970.&quot;</p>

<p>The board is made up of an <code>m x n</code> grid of cells, where each cell has an initial state: <b>live</b> (represented by a <code>1</code>) or <b>dead</b> (represented by a <code>0</code>). Each cell interacts with its <a href="https://en.wikipedia.org/wiki/Moore_neighborhood" target="_blank">eight neighbors</a> (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):</p>

<ol>
	<li>Any live cell with fewer than two live neighbors dies as if caused by under-population.</li>
	<li>Any live cell with two or three live neighbors lives on to the next generation.</li>
	<li>Any live cell with more than three live neighbors dies, as if by over-population.</li>
	<li>Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.</li>
</ol>

<p><span>The next state of the board is determined by applying the above rules simultaneously to every cell in the current state of the <code>m x n</code> grid <code>board</code>. In this process, births and deaths occur <strong>simultaneously</strong>.</span></p>

<p><span>Given the current state of the <code>board</code>, <strong>update</strong> the <code>board</code> to reflect its next state.</span></p>

<p><strong>Note</strong> that you do not need to return anything.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid1.jpg" style="width: 562px; height: 322px;" />
<pre>
<strong>Input:</strong> board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
<strong>Output:</strong> [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid2.jpg" style="width: 402px; height: 162px;" />
<pre>
<strong>Input:</strong> board = [[1,1],[1,0]]
<strong>Output:</strong> [[1,1],[1,1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == board.length</code></li>
	<li><code>n == board[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 25</code></li>
	<li><code>board[i][j]</code> is <code>0</code> or <code>1</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong></p>

<ul>
	<li>Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.</li>
	<li>In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?</li>
</ul>



## Solution

```rust
impl Solution { pub fn game_of_life(black_b: &mut Vec<Vec<i32>>) { let (black_m, black_n) = (black_b.len(), black_b[0].len()); for black_r in 0..black_m { for black_c in 0..black_n { let mut black_live = 0; for i in -1..=1 { for j in -1..=1 { if i == 0 && j == 0 { continue; } let (nr, nc) = (black_r as i32 + i, black_c as i32 + j); if nr >= 0 && nr < black_m as i32 && nc >= 0 && nc < black_n as i32 && (black_b[nr as usize][nc as usize] & 1) == 1 { black_live += 1; } } } if (black_b[black_r][black_c] == 1 && (black_live == 2 || black_live == 3)) || (black_b[black_r][black_c] == 0 && black_live == 3) { black_b[black_r][black_c] |= 2; } } } for black_r in 0..black_m { for black_c in 0..black_n { black_b[black_r][black_c] >>= 1; } } } }
```