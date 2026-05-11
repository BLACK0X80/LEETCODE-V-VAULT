# Number of Submatrices That Sum to Target

**Difficulty:** Hard
**Tags:** Array, Hash Table, Matrix, Prefix Sum

---

## Problem

<p>Given a <code>matrix</code>&nbsp;and a <code>target</code>, return the number of non-empty submatrices that sum to <font face="monospace">target</font>.</p>

<p>A submatrix <code>x1, y1, x2, y2</code> is the set of all cells <code>matrix[x][y]</code> with <code>x1 &lt;= x &lt;= x2</code> and <code>y1 &lt;= y &lt;= y2</code>.</p>

<p>Two submatrices <code>(x1, y1, x2, y2)</code> and <code>(x1&#39;, y1&#39;, x2&#39;, y2&#39;)</code> are different if they have some coordinate&nbsp;that is different: for example, if <code>x1 != x1&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/02/mate1.jpg" style="width: 242px; height: 242px;" />
<pre>
<strong>Input:</strong> matrix = [[0,1,0],[1,1,1],[0,1,0]], target = 0
<strong>Output:</strong> 4
<strong>Explanation:</strong> The four 1x1 submatrices that only contain 0.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[1,-1],[-1,1]], target = 0
<strong>Output:</strong> 5
<strong>Explanation:</strong> The two 1x2 submatrices, plus the two 2x1 submatrices, plus the 2x2 submatrix.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[904]], target = 0
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= matrix.length &lt;= 100</code></li>
	<li><code>1 &lt;= matrix[0].length &lt;= 100</code></li>
	<li><code>-1000 &lt;= matrix[i][j] &lt;= 1000</code></li>
	<li><code>-10^8 &lt;= target &lt;= 10^8</code></li>
</ul>


## Hints

1. Using a 2D prefix sum, we can query the sum of any submatrix in O(1) time.
Now for each (r1, r2), we can find the largest sum of a submatrix that uses every row in [r1, r2] in linear time using a sliding window.

## Solution

```rust
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans = 0;
        for r1 in 0..m {
            let mut col_sum = vec![0i32; n];
            for r2 in r1..m {
                for j in 0..n { col_sum[j] += matrix[r2][j]; }
                let mut map = HashMap::new();
                map.insert(0, 1);
                let mut prefix = 0;
                for j in 0..n {
                    prefix += col_sum[j];
                    ans += map.get(&(prefix - target)).copied().unwrap_or(0);
                    *map.entry(prefix).or_insert(0) += 1;
                }
            }
        }
        ans
    }
}
```