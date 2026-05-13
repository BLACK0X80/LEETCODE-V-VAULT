# Maximum Number of Moves to Kill All Pawns

**Difficulty:** Hard
**Tags:** Array, Math, Bit Manipulation, Breadth-First Search, Game Theory, Bitmask

---

## Problem

<p>There is a <code>50 x 50</code> chessboard with <strong>one</strong> knight and some pawns on it. You are given two integers <code>kx</code> and <code>ky</code> where <code>(kx, ky)</code> denotes the position of the knight, and a 2D array <code>positions</code> where <code>positions[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> denotes the position of the pawns on the chessboard.</p>

<p>Alice and Bob play a <em>turn-based</em> game, where Alice goes first. In each player&#39;s turn:</p>

<ul>
	<li>The player <em>selects </em>a pawn that still exists on the board and captures it with the knight in the <strong>fewest</strong> possible <strong>moves</strong>. <strong>Note</strong> that the player can select <strong>any</strong> pawn, it <strong>might not</strong> be one that can be captured in the <strong>least</strong> number of moves.</li>
	<li><span>In the process of capturing the <em>selected</em> pawn, the knight <strong>may</strong> pass other pawns <strong>without</strong> capturing them</span>. <strong>Only</strong> the <em>selected</em> pawn can be captured in <em>this</em> turn.</li>
</ul>

<p>Alice is trying to <strong>maximize</strong> the <strong>sum</strong> of the number of moves made by <em>both</em> players until there are no more pawns on the board, whereas Bob tries to <strong>minimize</strong> them.</p>

<p>Return the <strong>maximum</strong> <em>total</em> number of moves made during the game that Alice can achieve, assuming both players play <strong>optimally</strong>.</p>

<p>Note that in one <strong>move, </strong>a chess knight has eight possible positions it can move to, as illustrated below. Each move is two cells in a cardinal direction, then one cell in an orthogonal direction.</p>

<p><img src="https://assets.leetcode.com/uploads/2024/08/01/chess_knight.jpg" style="width: 275px; height: 273px;" /></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">kx = 1, ky = 1, positions = [[0,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/16/gif3.gif" style="width: 275px; height: 275px;" /></p>

<p>The knight takes 4 moves to reach the pawn at <code>(0, 0)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">kx = 0, ky = 2, positions = [[1,1],[2,2],[3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/08/16/gif4.gif" style="width: 320px; height: 320px;" /></strong></p>

<ul>
	<li>Alice picks the pawn at <code>(2, 2)</code> and captures it in two moves: <code>(0, 2) -&gt; (1, 4) -&gt; (2, 2)</code>.</li>
	<li>Bob picks the pawn at <code>(3, 3)</code> and captures it in two moves: <code>(2, 2) -&gt; (4, 1) -&gt; (3, 3)</code>.</li>
	<li>Alice picks the pawn at <code>(1, 1)</code> and captures it in four moves: <code>(3, 3) -&gt; (4, 1) -&gt; (2, 2) -&gt; (0, 3) -&gt; (1, 1)</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">kx = 0, ky = 0, positions = [[1,2],[2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Alice picks the pawn at <code>(2, 4)</code> and captures it in two moves: <code>(0, 0) -&gt; (1, 2) -&gt; (2, 4)</code>. Note that the pawn at <code>(1, 2)</code> is not captured.</li>
	<li>Bob picks the pawn at <code>(1, 2)</code> and captures it in one move: <code>(2, 4) -&gt; (1, 2)</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= kx, ky &lt;= 49</code></li>
	<li><code>1 &lt;= positions.length &lt;= 15</code></li>
	<li><code>positions[i].length == 2</code></li>
	<li><code>0 &lt;= positions[i][0], positions[i][1] &lt;= 49</code></li>
	<li>All <code>positions[i]</code> are unique.</li>
	<li>The input is generated such that <code>positions[i] != [kx, ky]</code> for all <code>0 &lt;= i &lt; positions.length</code>.</li>
</ul>


## Hints

1. Use BFS to preprocess the minimum number of moves to reach one pawn from the other pawns.
2. Consider the knight’s original position as another pawn.
3. Use DP with a bitmask to store current pawns that have not been captured.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_moves(black_kx: i32, black_ky: i32, black_positions: Vec<Vec<i32>>) -> i32 {
        let black_n = black_positions.len();
        let mut black_pts = vec![vec![black_kx, black_ky]];
        for black_p in black_positions {
            black_pts.push(black_p);
        }

        let mut black_dist = vec![vec![0; black_n + 1]; black_n + 1];
        let black_dirs = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

        for black_i in 0..=black_n {
            let mut black_d = vec![vec![-1; 50]; 50];
            let mut black_q = VecDeque::from([(black_pts[black_i][0], black_pts[black_i][1])]);
            black_d[black_pts[black_i][0] as usize][black_pts[black_i][1] as usize] = 0;

            while let Some((black_x, black_y)) = black_q.pop_front() {
                for (black_dx, black_dy) in black_dirs {
                    let (black_nx, black_ny) = (black_x + black_dx, black_y + black_dy);
                    if black_nx >= 0 && black_nx < 50 && black_ny >= 0 && black_ny < 50 {
                        if black_d[black_nx as usize][black_ny as usize] == -1 {
                            black_d[black_nx as usize][black_ny as usize] = black_d[black_x as usize][black_y as usize] + 1;
                            black_q.push_back((black_nx, black_ny));
                        }
                    }
                }
            }
            for black_j in 0..=black_n {
                black_dist[black_i][black_j] = black_d[black_pts[black_j][0] as usize][black_pts[black_j][1] as usize];
            }
        }

        let mut black_memo = vec![vec![-1; 1 << black_n]; black_n + 1];
        Self::black_solve(0, (1 << black_n) - 1, black_n, &black_dist, &mut black_memo, true)
    }

    fn black_solve(black_curr: usize, black_mask: usize, black_n: usize, black_dist: &Vec<Vec<i32>>, black_memo: &mut Vec<Vec<i32>>, black_alice: bool) -> i32 {
        if black_mask == 0 { return 0; }
        if black_memo[black_curr][black_mask] != -1 { return black_memo[black_curr][black_mask]; }

        let mut black_res = if black_alice { 0 } else { i32::MAX };

        for black_i in 0..black_n {
            if (black_mask >> black_i) & 1 == 1 {
                let black_moves = black_dist[black_curr][black_i + 1] + 
                                  Self::black_solve(black_i + 1, black_mask ^ (1 << black_i), black_n, black_dist, black_memo, !black_alice);
                if black_alice {
                    black_res = black_res.max(black_moves);
                } else {
                    black_res = black_res.min(black_moves);
                }
            }
        }

        black_memo[black_curr][black_mask] = black_res;
        black_res
    }
}
```