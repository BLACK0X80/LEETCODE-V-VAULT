# N-Queens II

**Difficulty:** Hard
**Tags:** Backtracking

---

## Problem

<p>The <strong>n-queens</strong> puzzle is the problem of placing <code>n</code> queens on an <code>n x n</code> chessboard such that no two queens attack each other.</p>

<p>Given an integer <code>n</code>, return <em>the number of distinct solutions to the&nbsp;<strong>n-queens puzzle</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two distinct solutions to the 4-queens puzzle as shown.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 9</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];

        backtrack(0, n, &mut cols, &mut diag1, &mut diag2)
    }
}

fn backtrack(
    row: usize,
    n: usize,
    cols: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
) -> i32 {
    if row == n {
        return 1;
    }

    let mut count = 0;

    for col in 0..n {
        let d1 = row + col;
        let d2 = row + n - 1 - col;

        if cols[col] || diag1[d1] || diag2[d2] {
            continue;
        }

        cols[col] = true;
        diag1[d1] = true;
        diag2[d2] = true;

        count += backtrack(row + 1, n, cols, diag1, diag2);

        cols[col] = false;
        diag1[d1] = false;
        diag2[d2] = false;
    }

    count
}
```