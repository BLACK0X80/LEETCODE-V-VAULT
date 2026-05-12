# N-Queens

**Difficulty:** Hard
**Tags:** Array, Backtracking

---

## Problem

<p>The <strong>n-queens</strong> puzzle is the problem of placing <code>n</code> queens on an <code>n x n</code> chessboard such that no two queens attack each other.</p>

<p>Given an integer <code>n</code>, return <em>all distinct solutions to the <strong>n-queens puzzle</strong></em>. You may return the answer in <strong>any order</strong>.</p>

<p>Each solution contains a distinct board configuration of the n-queens&#39; placement, where <code>&#39;Q&#39;</code> and <code>&#39;.&#39;</code> both indicate a queen and an empty space, respectively.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> [[&quot;.Q..&quot;,&quot;...Q&quot;,&quot;Q...&quot;,&quot;..Q.&quot;],[&quot;..Q.&quot;,&quot;Q...&quot;,&quot;...Q&quot;,&quot;.Q..&quot;]]
<strong>Explanation:</strong> There exist two distinct solutions to the 4-queens puzzle as shown above
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> [[&quot;Q&quot;]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 9</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut queens = vec![0usize; n];
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];

        backtrack(0, n, &mut queens, &mut cols, &mut diag1, &mut diag2, &mut result);
        result
    }
}

fn backtrack(
    row: usize,
    n: usize,
    queens: &mut Vec<usize>,
    cols: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
    result: &mut Vec<Vec<String>>,
) {
    if row == n {
        result.push(build_board(queens, n));
        return;
    }

    for col in 0..n {
        let d1 = row + col;
        let d2 = row + n - 1 - col;

        if cols[col] || diag1[d1] || diag2[d2] {
            continue;
        }

        queens[row] = col;
        cols[col] = true;
        diag1[d1] = true;
        diag2[d2] = true;

        backtrack(row + 1, n, queens, cols, diag1, diag2, result);

        cols[col] = false;
        diag1[d1] = false;
        diag2[d2] = false;
    }
}

fn build_board(queens: &Vec<usize>, n: usize) -> Vec<String> {
    queens.iter().map(|&col| {
        let mut row = vec![b'.'; n];
        row[col] = b'Q';
        String::from_utf8(row).unwrap()
    }).collect()
}
```