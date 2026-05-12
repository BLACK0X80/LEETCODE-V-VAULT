# Spiral Matrix IV

**Difficulty:** Medium
**Tags:** Array, Linked List, Matrix, Simulation

---

## Problem

<p>You are given two integers <code>m</code> and <code>n</code>, which represent the dimensions of a matrix.</p>

<p>You are also given the <code>head</code> of a linked list of integers.</p>

<p>Generate an <code>m x n</code> matrix that contains the integers in the linked list presented in <strong>spiral</strong> order <strong>(clockwise)</strong>, starting from the <strong>top-left</strong> of the matrix. If there are remaining empty spaces, fill them with <code>-1</code>.</p>

<p>Return <em>the generated matrix</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/05/09/ex1new.jpg" style="width: 240px; height: 150px;" />
<pre>
<strong>Input:</strong> m = 3, n = 5, head = [3,0,2,6,8,1,7,9,4,2,5,5,0]
<strong>Output:</strong> [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]
<strong>Explanation:</strong> The diagram above shows how the values are printed in the matrix.
Note that the remaining spaces in the matrix are filled with -1.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/05/11/ex2.jpg" style="width: 221px; height: 60px;" />
<pre>
<strong>Input:</strong> m = 1, n = 4, head = [0,1,2]
<strong>Output:</strong> [[0,1,2,-1]]
<strong>Explanation:</strong> The diagram above shows how the values are printed from left to right in the matrix.
The last space in the matrix is set to -1.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li>The number of nodes in the list is in the range <code>[1, m * n]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 1000</code></li>
</ul>


## Hints

1. First, generate an m x n matrix filled with -1s.
2. Navigate within the matrix at (i, j) with the help of a direction vector ⟨di, dj⟩. At (i, j), you need to decide if you can keep going in the current direction.
3. If you cannot keep going, rotate the direction vector clockwise by 90 degrees.

## Solution

```rust
impl Solution { pub fn spiral_matrix(black_m: i32, black_n: i32, mut black_h: Option<Box<ListNode>>) -> Vec<Vec<i32>> { let (black_rows, black_cols) = (black_m as usize, black_n as usize); let mut black_res = vec![vec![-1; black_cols]; black_rows]; let (mut black_t, mut black_b, mut black_l, mut black_r) = (0, black_rows - 1, 0, black_cols - 1); while let Some(black_node) = black_h { let mut black_curr = Some(black_node); for j in black_l..=black_r { if let Some(n) = black_curr { black_res[black_t][j] = n.val; black_curr = n.next; } else { return black_res; } } black_t += 1; if black_t > black_b { return black_res; } for i in black_t..=black_b { if let Some(n) = black_curr { black_res[i][black_r] = n.val; black_curr = n.next; } else { return black_res; } } if black_r == 0 { return black_res; } black_r -= 1; for j in (black_l..=black_r).rev() { if let Some(n) = black_curr { black_res[black_b][j] = n.val; black_curr = n.next; } else { return black_res; } } if black_b == 0 { return black_res; } black_b -= 1; for i in (black_t..=black_b).rev() { if let Some(n) = black_curr { black_res[i][black_l] = n.val; black_curr = n.next; } else { return black_res; } } black_l += 1; black_h = black_curr; } black_res } }
```