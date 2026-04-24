# Minimum Moves to Spread Stones Over Grid

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Matrix, Bitmask

---

## Problem

<p>You are given a <strong>0-indexed</strong> 2D integer matrix <code>grid</code> of size <code>3 * 3</code>, representing the number of stones in each cell. The grid contains exactly <code>9</code> stones, and there can be <strong>multiple</strong> stones in a single cell.</p>

<p>In one move, you can move a single stone from its current cell to any other cell if the two cells share a side.</p>

<p>Return <em>the <strong>minimum number of moves</strong> required to place one stone in each cell</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/23/example1-3.svg" style="width: 401px; height: 281px;" />
<pre>
<strong>Input:</strong> grid = [[1,1,0],[1,1,1],[1,2,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One possible sequence of moves to place one stone in each cell is: 
1- Move one stone from cell (2,1) to cell (2,2).
2- Move one stone from cell (2,2) to cell (1,2).
3- Move one stone from cell (1,2) to cell (0,2).
In total, it takes 3 moves to place one stone in each cell of the grid.
It can be shown that 3 is the minimum number of moves required to place one stone in each cell.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/23/example2-2.svg" style="width: 401px; height: 281px;" />
<pre>
<strong>Input:</strong> grid = [[1,3,0],[1,0,0],[1,0,3]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One possible sequence of moves to place one stone in each cell is:
1- Move one stone from cell (0,1) to cell (0,2).
2- Move one stone from cell (0,1) to cell (1,1).
3- Move one stone from cell (2,2) to cell (1,2).
4- Move one stone from cell (2,2) to cell (2,1).
In total, it takes 4 moves to place one stone in each cell of the grid.
It can be shown that 4 is the minimum number of moves required to place one stone in each cell.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>grid.length == grid[i].length == 3</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 9</code></li>
	<li>Sum of <code>grid</code> is equal to <code>9</code>.</li>
</ul>


## Hints

1. There are at most <code>4</code> cells with more than one stone.
2. Let <code>a</code> be the number of cells containing more than one stone, and <code>b</code> be the number of cells containing no stones. <code></code>. <code>b^a ≤ 6561</code>. Use this fact to come up with a bruteforce.
3. For all empty cells, bruteforce over all possible cells from which a stone can come. Note that a stone will always come from a cell containing at least 2 stones.

## Solution

```rust
impl Solution {
    pub fn minimum_moves(black_grid: Vec<Vec<i32>>) -> i32 {
        let (mut black_src, mut black_dst) = (vec![], vec![]);
        for black_r in 0..3 { for black_c in 0..3 { if black_grid[black_r][black_c] == 0 { black_dst.push((black_r, black_c)); } else if black_grid[black_r][black_c] > 1 { for _ in 0..black_grid[black_r][black_c]-1 { black_src.push((black_r, black_c)); } } } }
        let mut black_min = i32::MAX; fn black_perm(mut b_s: Vec<(usize,usize)>, b_d: &Vec<(usize,usize)>, b_idx: usize, b_m: &mut i32) { if b_idx == b_s.len() { *b_m = (*b_m).min(b_s.iter().zip(b_d).map(|(s, d)| (s.0 as i32 - d.0 as i32).abs() + (s.1 as i32 - d.1 as i32).abs()).sum()); return; } for i in b_idx..b_s.len() { b_s.swap(b_idx, i); black_perm(b_s.clone(), b_d, b_idx + 1, b_m); b_s.swap(b_idx, i); } }
        black_perm(black_src, &black_dst, 0, &mut black_min); black_min
    }
}
```