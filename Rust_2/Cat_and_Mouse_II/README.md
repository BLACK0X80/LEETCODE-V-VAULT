# Cat and Mouse II

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Graph Theory, Topological Sort, Memoization, Matrix, Game Theory

---

## Problem

<p>A game is played by a cat and a mouse named Cat and Mouse.</p>

<p>The environment is represented by a <code>grid</code> of size <code>rows x cols</code>, where each element is a wall, floor, player (Cat, Mouse), or food.</p>

<ul>
	<li>Players are represented by the characters <code>&#39;C&#39;</code>(Cat)<code>,&#39;M&#39;</code>(Mouse).</li>
	<li>Floors are represented by the character <code>&#39;.&#39;</code> and can be walked on.</li>
	<li>Walls are represented by the character <code>&#39;#&#39;</code> and cannot be walked on.</li>
	<li>Food is represented by the character <code>&#39;F&#39;</code> and can be walked on.</li>
	<li>There is only one of each character <code>&#39;C&#39;</code>, <code>&#39;M&#39;</code>, and <code>&#39;F&#39;</code> in <code>grid</code>.</li>
</ul>

<p>Mouse and Cat play according to the following rules:</p>

<ul>
	<li>Mouse <strong>moves first</strong>, then they take turns to move.</li>
	<li>During each turn, Cat and Mouse can jump in one of the four directions (left, right, up, down). They cannot jump over the wall nor outside of the <code>grid</code>.</li>
	<li><code>catJump, mouseJump</code> are the maximum lengths Cat and Mouse can jump at a time, respectively. Cat and Mouse can jump less than the maximum length.</li>
	<li>Staying in the same position is allowed.</li>
	<li>Mouse can jump over Cat.</li>
</ul>

<p>The game can end in 4 ways:</p>

<ul>
	<li>If Cat occupies the same position as Mouse, Cat wins.</li>
	<li>If Cat reaches the food first, Cat wins.</li>
	<li>If Mouse reaches the food first, Mouse wins.</li>
	<li>If Mouse cannot get to the food within 1000 turns, Cat wins.</li>
</ul>

<p>Given a <code>rows x cols</code> matrix <code>grid</code> and two integers <code>catJump</code> and <code>mouseJump</code>, return <code>true</code><em> if Mouse can win the game if both Cat and Mouse play optimally, otherwise return </em><code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/12/sample_111_1955.png" style="width: 580px; height: 239px;" />
<pre>
<strong>Input:</strong> grid = [&quot;####F&quot;,&quot;#C...&quot;,&quot;M....&quot;], catJump = 1, mouseJump = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> Cat cannot catch Mouse on its turn nor can it get the food before Mouse.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/12/sample_2_1955.png" style="width: 580px; height: 175px;" />
<pre>
<strong>Input:</strong> grid = [&quot;M.C...F&quot;], catJump = 1, mouseJump = 4
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [&quot;M.C...F&quot;], catJump = 1, mouseJump = 3
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>rows == grid.length</code></li>
	<li><code>cols = grid[i].length</code></li>
	<li><code>1 &lt;= rows, cols &lt;= 8</code></li>
	<li><code>grid[i][j]</code> consist only of characters <code>&#39;C&#39;</code>, <code>&#39;M&#39;</code>, <code>&#39;F&#39;</code>, <code>&#39;.&#39;</code>, and <code>&#39;#&#39;</code>.</li>
	<li>There is only one of each character <code>&#39;C&#39;</code>, <code>&#39;M&#39;</code>, and <code>&#39;F&#39;</code> in <code>grid</code>.</li>
	<li><code>1 &lt;= catJump, mouseJump &lt;= 8</code></li>
</ul>


## Hints

1. Try working backward: consider all trivial states you know to be winning or losing, and work backward to determine which other states can be labeled as winning or losing.

## Solution

```rust
impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut mr = 0; let mut mc = 0;
        let mut cr = 0; let mut cc = 0;
        let mut fr = 0; let mut fc = 0;
        let g: Vec<Vec<u8>> = grid.iter().map(|s| s.bytes().collect()).collect();
        for r in 0..rows {
            for c in 0..cols {
                match g[r][c] {
                    b'M' => { mr = r; mc = c; }
                    b'C' => { cr = r; cc = c; }
                    b'F' => { fr = r; fc = c; }
                    _ => {}
                }
            }
        }
        let total = 1000 * rows * rows * cols * cols;
        let mut memo = vec![-1i8; total];
        fn solve(step: usize, mr: usize, mc: usize, cr: usize, cc: usize,
                 cj: i32, mj: i32, rows: usize, cols: usize,
                 g: &[Vec<u8>], fr: usize, fc: usize, memo: &mut [i8]) -> bool {
            if mr == fr && mc == fc { return true; }
            if cr == fr && cc == fc { return false; }
            if mr == cr && mc == cc { return false; }
            if step >= 1000 { return false; }
            let idx = (((step * rows + mr) * cols + mc) * rows + cr) * cols + cc;
            if memo[idx] != -1 { return memo[idx] == 1; }
            let is_mouse = step % 2 == 0;
            let jump = if is_mouse { mj } else { cj };
            let res = if is_mouse {
                let mut win = false;
                for &(dr, dc) in [(0,1), (0,-1), (1,0), (-1,0)].iter() {
                    for d in 0..=jump {
                        let nr = mr as i32 + dr * d;
                        let nc = mc as i32 + dc * d;
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 || g[nr as usize][nc as usize] == b'#' { break; }
                        if solve(step + 1, nr as usize, nc as usize, cr, cc, cj, mj, rows, cols, g, fr, fc, memo) { win = true; break; }
                    }
                    if win { break; }
                }
                win
            } else {
                let mut lose = true;
                for &(dr, dc) in [(0,1), (0,-1), (1,0), (-1,0)].iter() {
                    for d in 0..=jump {
                        let nr = cr as i32 + dr * d;
                        let nc = cc as i32 + dc * d;
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 || g[nr as usize][nc as usize] == b'#' { break; }
                        if !solve(step + 1, mr, mc, nr as usize, nc as usize, cj, mj, rows, cols, g, fr, fc, memo) { lose = false; break; }
                    }
                    if !lose { break; }
                }
                lose
            };
            memo[idx] = if res { 1 } else { 0 };
            res
        }
        solve(0, mr, mc, cr, cc, cat_jump, mouse_jump, rows, cols, &g, fr, fc, &mut memo)
    }
}
```