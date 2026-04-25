# Rank Transform of a Matrix

**Difficulty:** Hard
**Tags:** Array, Union-Find, Graph Theory, Topological Sort, Sorting, Matrix

---

## Problem

<p>Given an <code>m x n</code> <code>matrix</code>, return <em>a new matrix </em><code>answer</code><em> where </em><code>answer[row][col]</code><em> is the </em><em><strong>rank</strong> of </em><code>matrix[row][col]</code>.</p>

<p>The <strong>rank</strong> is an <strong>integer</strong> that represents how large an element is compared to other elements. It is calculated using the following rules:</p>

<ul>
	<li>The rank is an integer starting from <code>1</code>.</li>
	<li>If two elements <code>p</code> and <code>q</code> are in the <strong>same row or column</strong>, then:
	<ul>
		<li>If <code>p &lt; q</code> then <code>rank(p) &lt; rank(q)</code></li>
		<li>If <code>p == q</code> then <code>rank(p) == rank(q)</code></li>
		<li>If <code>p &gt; q</code> then <code>rank(p) &gt; rank(q)</code></li>
	</ul>
	</li>
	<li>The <strong>rank</strong> should be as <strong>small</strong> as possible.</li>
</ul>

<p>The test cases are generated so that <code>answer</code> is unique under the given rules.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank1.jpg" style="width: 442px; height: 162px;" />
<pre>
<strong>Input:</strong> matrix = [[1,2],[3,4]]
<strong>Output:</strong> [[1,2],[2,3]]
<strong>Explanation:</strong>
The rank of matrix[0][0] is 1 because it is the smallest integer in its row and column.
The rank of matrix[0][1] is 2 because matrix[0][1] &gt; matrix[0][0] and matrix[0][0] is rank 1.
The rank of matrix[1][0] is 2 because matrix[1][0] &gt; matrix[0][0] and matrix[0][0] is rank 1.
The rank of matrix[1][1] is 3 because matrix[1][1] &gt; matrix[0][1], matrix[1][1] &gt; matrix[1][0], and both matrix[0][1] and matrix[1][0] are rank 2.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank2.jpg" style="width: 442px; height: 162px;" />
<pre>
<strong>Input:</strong> matrix = [[7,7],[7,7]]
<strong>Output:</strong> [[1,1],[1,1]]
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank3.jpg" style="width: 601px; height: 322px;" />
<pre>
<strong>Input:</strong> matrix = [[20,-21,14],[-19,4,19],[22,-47,24],[-19,4,19]]
<strong>Output:</strong> [[4,2,3],[1,3,4],[5,1,6],[1,3,4]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 500</code></li>
	<li><code>-10<sup>9</sup> &lt;= matrix[row][col] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Sort the cells by value and process them in increasing order.
2. The rank of a cell is the maximum rank in its row and column plus one.
3. Handle the equal cells by treating them as components using a union-find data structure.

## Solution

```rust
impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut black_cells: Vec<(i32, usize, usize)> = vec![];
        for i in 0..m { for j in 0..n { black_cells.push((matrix[i][j], i, j)); } }
        black_cells.sort_unstable();
        let mut black_rank = vec![0i32; m + n];
        let mut black_ans = vec![vec![0i32; n]; m];

        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }

        let mut i = 0;
        while i < black_cells.len() {
            let val = black_cells[i].0;
            let mut j = i;
            while j < black_cells.len() && black_cells[j].0 == val { j += 1; }

            let mut black_dsu: Vec<usize> = (0..m+n).collect();
            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let (pr, pc) = (black_find(&mut black_dsu, r), black_find(&mut black_dsu, m+c));
                if pr != pc { black_dsu[pr] = pc; }
            }

            
            let mut black_max = vec![0i32; m+n];
            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let root = black_find(&mut black_dsu, r);
                black_max[root] = black_max[root].max(black_rank[r]).max(black_rank[m+c]);
            }

            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let root = black_find(&mut black_dsu, r);
                black_ans[r][c] = black_max[root] + 1;
                black_rank[r] = black_ans[r][c];
                black_rank[m+c] = black_ans[r][c];
            }
            i = j;
        }
        black_ans
    }
}
```