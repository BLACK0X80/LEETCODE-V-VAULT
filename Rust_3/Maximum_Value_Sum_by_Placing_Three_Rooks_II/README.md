# Maximum Value Sum by Placing Three Rooks II

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix, Enumeration

---

## Problem

<p>You are given a <code>m x n</code> 2D array <code>board</code> representing a chessboard, where <code>board[i][j]</code> represents the <strong>value</strong> of the cell <code>(i, j)</code>.</p>

<p>Rooks in the <strong>same</strong> row or column <strong>attack</strong> each other. You need to place <em>three</em> rooks on the chessboard such that the rooks <strong>do not</strong> <strong>attack</strong> each other.</p>

<p>Return the <strong>maximum</strong> sum of the cell <strong>values</strong> on which the rooks are placed.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">board = </span>[[-3,1,1,1],[-3,1,-3,1],[-3,2,1,1]]</p>

<p><strong>Output:</strong> 4</p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/08/rooks2.png" style="width: 294px; height: 450px;" /></p>

<p>We can place the rooks in the cells <code>(0, 2)</code>, <code>(1, 3)</code>, and <code>(2, 1)</code> for a sum of <code>1 + 1 + 2 = 4</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">board = [[1,2,3],[4,5,6],[7,8,9]]</span></p>

<p><strong>Output:</strong> <span class="example-io">15</span></p>

<p><strong>Explanation:</strong></p>

<p>We can place the rooks in the cells <code>(0, 0)</code>, <code>(1, 1)</code>, and <code>(2, 2)</code> for a sum of <code>1 + 5 + 9 = 15</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">board = [[1,1,1],[1,1,1],[1,1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>We can place the rooks in the cells <code>(0, 2)</code>, <code>(1, 1)</code>, and <code>(2, 0)</code> for a sum of <code>1 + 1 + 1 = 3</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= m == board.length &lt;= 500</code></li>
	<li><code>3 &lt;= n == board[i].length &lt;= 500</code></li>
	<li><code>-10<sup>9</sup> &lt;= board[i][j] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Save the top 3 largest values in each row.
2. Select any row, and select any of the three values stored in it.
3. Get the top 4 values from all of the other 3 largest values of the other rows, which do not share the same column as the selected value.
4. Brute force the selection of 2 positions from the top 4 now.

## Solution

```rust
impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let black1 = board.len();
        let black2 = board[0].len();
        let mut black3 = vec![vec![]; black1];

        for black4 in 0..black1 {
            let mut black5: Vec<(i64, usize)> = board[black4].iter().enumerate()
                .map(|(i, &v)| (v as i64, i)).collect();
            black5.sort_by(|a, b| b.0.cmp(&a.0));
            black3[black4] = black5[..3].to_vec();
        }

        let mut black6 = i64::MIN;
        for black7 in 0..black1 {
            for black8 in black7 + 1..black1 {
                for black9 in black8 + 1..black1 {
                    for black10 in &black3[black7] {
                        for black11 in &black3[black8] {
                            if black11.1 == black10.1 { continue; }
                            for black12 in &black3[black9] {
                                if black12.1 == black10.1 || black12.1 == black11.1 { continue; }
                                black6 = black6.max(black10.0 + black11.0 + black12.0);
                            }
                        }
                    }
                }
            }
        }
        black6
    }
}
```