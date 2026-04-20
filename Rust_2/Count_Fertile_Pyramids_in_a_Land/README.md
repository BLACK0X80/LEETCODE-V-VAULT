# Count Fertile Pyramids in a Land

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>A farmer has a <strong>rectangular grid</strong> of land with <code>m</code> rows and <code>n</code> columns that can be divided into unit cells. Each cell is either <strong>fertile</strong> (represented by a <code>1</code>) or <strong>barren</strong> (represented by a <code>0</code>). All cells outside the grid are considered barren.</p>

<p>A <strong>pyramidal plot</strong> of land can be defined as a set of cells with the following criteria:</p>

<ol>
	<li>The number of cells in the set has to be <strong>greater than </strong><code>1</code> and all cells must be <strong>fertile</strong>.</li>
	<li>The <strong>apex</strong> of a pyramid is the <strong>topmost</strong> cell of the pyramid. The <strong>height</strong> of a pyramid is the number of rows it covers. Let <code>(r, c)</code> be the apex of the pyramid, and its height be <code>h</code>. Then, the plot comprises of cells <code>(i, j)</code> where <code>r &lt;= i &lt;= r + h - 1</code> <strong>and</strong> <code>c - (i - r) &lt;= j &lt;= c + (i - r)</code>.</li>
</ol>

<p>An <strong>inverse pyramidal plot</strong> of land can be defined as a set of cells with similar criteria:</p>

<ol>
	<li>The number of cells in the set has to be <strong>greater than </strong><code>1</code> and all cells must be <strong>fertile</strong>.</li>
	<li>The <strong>apex</strong> of an inverse pyramid is the <strong>bottommost</strong> cell of the inverse pyramid. The <strong>height</strong> of an inverse pyramid is the number of rows it covers. Let <code>(r, c)</code> be the apex of the pyramid, and its height be <code>h</code>. Then, the plot comprises of cells <code>(i, j)</code> where <code>r - h + 1 &lt;= i &lt;= r</code> <strong>and</strong> <code>c - (r - i) &lt;= j &lt;= c + (r - i)</code>.</li>
</ol>

<p>Some examples of valid and invalid pyramidal (and inverse pyramidal) plots are shown below. Black cells indicate fertile cells.</p>
<img src="https://assets.leetcode.com/uploads/2021/11/08/image.png" style="width: 700px; height: 156px;" />
<p>Given a <strong>0-indexed</strong> <code>m x n</code> binary matrix <code>grid</code> representing the farmland, return <em>the <strong>total number</strong> of pyramidal and inverse pyramidal plots that can be found in</em> <code>grid</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/22/1.JPG" style="width: 575px; height: 109px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[1,1,1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 2 possible pyramidal plots are shown in blue and red respectively.
There are no inverse pyramidal plots in this grid. 
Hence total number of pyramidal and inverse pyramidal plots is 2 + 0 = 2.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/22/2.JPG" style="width: 502px; height: 120px;" />
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The pyramidal plot is shown in blue, and the inverse pyramidal plot is shown in red. 
Hence the total number of plots is 1 + 1 = 2.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/22/3.JPG" style="width: 676px; height: 148px;" />
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,0],[1,1,1,1,1],[1,1,1,1,1],[0,1,0,0,1]]
<strong>Output:</strong> 13
<strong>Explanation:</strong> There are 7 pyramidal plots, 3 of which are shown in the 2nd and 3rd figures.
There are 6 inverse pyramidal plots, 2 of which are shown in the last figure.
The total number of plots is 7 + 6 = 13.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Think about how dynamic programming can help solve the problem.
2. For any fixed cell (r, c), can you calculate the maximum height of the pyramid for which it is the apex? Let us denote this value as dp[r][c].
3. How will the values at dp[r+1][c-1] and dp[r+1][c+1] help in determining the value at dp[r][c]?
4. For the cell (r, c), is there a relation between the number of pyramids for which it serves as the apex and dp[r][c]? How does it help in calculating the answer?

## Solution

```rust
impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m { for j in 0..n { dp[i][j] = grid[i][j]; } }
        for i in (0..m-1).rev() {
            for j in 1..n-1 {
                if dp[i][j] > 0 {
                    dp[i][j] = dp[i+1][j-1].min(dp[i+1][j]).min(dp[i+1][j+1]) + 1;
                    if dp[i][j] > 1 { ans += dp[i][j] - 1; }
                }
            }
        }
        for i in 0..m { for j in 0..n { dp[i][j] = grid[i][j]; } }
        for i in 1..m {
            for j in 1..n-1 {
                if dp[i][j] > 0 {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i-1][j+1]) + 1;
                    if dp[i][j] > 1 { ans += dp[i][j] - 1; }
                }
            }
        }
        ans
    }
}
```