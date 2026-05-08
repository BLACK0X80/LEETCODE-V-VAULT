# Transform to Chessboard

**Difficulty:** Hard
**Tags:** Array, Math, Bit Manipulation, Matrix

---

## Problem

<p>You are given an <code>n x n</code> binary grid <code>board</code>. In each move, you can swap any two rows with each other, or any two columns with each other.</p>

<p>Return <em>the minimum number of moves to transform the board into a <strong>chessboard board</strong></em>. If the task is impossible, return <code>-1</code>.</p>

<p>A <strong>chessboard board</strong> is a board where no <code>0</code>&#39;s and no <code>1</code>&#39;s are 4-directionally adjacent.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard1-grid.jpg" style="width: 500px; height: 145px;" />
<pre>
<strong>Input:</strong> board = [[0,1,1,0],[0,1,1,0],[1,0,0,1],[1,0,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> One potential sequence of moves is shown.
The first move swaps the first and second column.
The second move swaps the second and third row.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard2-grid.jpg" style="width: 164px; height: 165px;" />
<pre>
<strong>Input:</strong> board = [[0,1],[1,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Also note that the board with 0 in the top left corner, is also a valid chessboard.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard3-grid.jpg" style="width: 164px; height: 165px;" />
<pre>
<strong>Input:</strong> board = [[1,0],[1,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> No matter what sequence of moves you make, you cannot end with a valid chessboard.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == board.length</code></li>
	<li><code>n == board[i].length</code></li>
	<li><code>2 &lt;= n &lt;= 30</code></li>
	<li><code>board[i][j]</code> is either&nbsp;<code>0</code> or <code>1</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let full = (1u32 << n) - 1;

        let calc = |vals: Vec<u32>| -> i32 {
            let first = vals[0];
            let mut total = 0usize;
            let mut odd = 0usize;
            for (i, &x) in vals.iter().enumerate() {
                if x == first {
                    total += 1;
                    if i & 1 == 1 { odd += 1; }
                } else if first ^ x != full {
                    return i32::MAX;
                }
            }
            let mut ans = i32::MAX;
            let len = vals.len();
            if len <= 2 * total && 2 * total <= len + 1 { ans = ans.min(odd as i32); }
            if len.saturating_sub(1) <= 2 * total && 2 * total <= len { ans = ans.min((total - odd) as i32); }
            ans
        };

        let mut rows = vec![0u32; n];
        let mut cols = vec![0u32; n];
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 1 {
                    rows[i] ^= 1 << j;
                    cols[j] ^= 1 << i;
                }
            }
        }

        let r = calc(rows);
        let c = calc(cols);
        if r == i32::MAX || c == i32::MAX { -1 } else { r + c }
    }
}
```