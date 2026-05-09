# Valid Tic-Tac-Toe State

**Difficulty:** Medium
**Tags:** Array, Matrix

---

## Problem

<p>Given a Tic-Tac-Toe board as a string array <code>board</code>, return <code>true</code> if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.</p>

<p>The board is a <code>3 x 3</code> array that consists of characters <code>&#39; &#39;</code>, <code>&#39;X&#39;</code>, and <code>&#39;O&#39;</code>. The <code>&#39; &#39;</code> character represents an empty square.</p>

<p>Here are the rules of Tic-Tac-Toe:</p>

<ul>
	<li>Players take turns placing characters into empty squares <code>&#39; &#39;</code>.</li>
	<li>The first player always places <code>&#39;X&#39;</code> characters, while the second player always places <code>&#39;O&#39;</code> characters.</li>
	<li><code>&#39;X&#39;</code> and <code>&#39;O&#39;</code> characters are always placed into empty squares, never filled ones.</li>
	<li>The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.</li>
	<li>The game also ends if all squares are non-empty.</li>
	<li>No more moves can be played if the game is over.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe1-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> board = [&quot;O  &quot;,&quot;   &quot;,&quot;   &quot;]
<strong>Output:</strong> false
<strong>Explanation:</strong> The first player always plays &quot;X&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe2-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> board = [&quot;XOX&quot;,&quot; X &quot;,&quot;   &quot;]
<strong>Output:</strong> false
<strong>Explanation:</strong> Players take turns making moves.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe4-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> board = [&quot;XOX&quot;,&quot;O O&quot;,&quot;XOX&quot;]
<strong>Output:</strong> true
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>board.length == 3</code></li>
	<li><code>board[i].length == 3</code></li>
	<li><code>board[i][j]</code> is either <code>&#39;X&#39;</code>, <code>&#39;O&#39;</code>, or <code>&#39; &#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn valid_tic_tac_toe(black_board: Vec<String>) -> bool { let mut black_x = 0; let mut black_o = 0; for row in &black_board { for c in row.chars() { if c == 'X' { black_x += 1; } else if c == 'O' { black_o += 1; } } } if black_o > black_x || black_x - black_o > 1 { return false; } let black_xw = Self::black_win(&black_board, 'X'); let black_ow = Self::black_win(&black_board, 'O'); if black_xw && black_ow { return false; } if black_xw && black_x != black_o + 1 { return false; } if black_ow && black_x != black_o { return false; } true } fn black_win(b: &Vec<String>, p: char) -> bool { let black_b: Vec<Vec<char>> = b.iter().map(|s| s.chars().collect()).collect(); for i in 0..3 { if (black_b[i][0] == p && black_b[i][1] == p && black_b[i][2] == p) || (black_b[0][i] == p && black_b[1][i] == p && black_b[2][i] == p) { return true; } } (black_b[0][0] == p && black_b[1][1] == p && black_b[2][2] == p) || (black_b[0][2] == p && black_b[1][1] == p && black_b[2][0] == p) } }
```