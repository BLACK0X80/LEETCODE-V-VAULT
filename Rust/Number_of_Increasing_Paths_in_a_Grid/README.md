# Number of Increasing Paths in a Grid

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Depth-First Search, Breadth-First Search, Graph Theory, Topological Sort, Memoization, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>grid</code>, where you can move from a cell to any adjacent cell in all <code>4</code> directions.</p>

<p>Return <em>the number of <strong>strictly</strong> <strong>increasing</strong> paths in the grid such that you can start from <strong>any</strong> cell and end at <strong>any</strong> cell. </em>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Two paths are considered different if they do not have exactly the same sequence of visited cells.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/05/10/griddrawio-4.png" style="width: 181px; height: 121px;" />
<pre>
<strong>Input:</strong> grid = [[1,1],[3,4]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The strictly increasing paths are:
- Paths with length 1: [1], [1], [3], [4].
- Paths with length 2: [1 -&gt; 3], [1 -&gt; 4], [3 -&gt; 4].
- Paths with length 3: [1 -&gt; 3 -&gt; 4].
The total number of paths is 4 + 3 + 1 = 8.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1],[2]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The strictly increasing paths are:
- Paths with length 1: [1], [2].
- Paths with length 2: [1 -&gt; 2].
The total number of paths is 2 + 1 = 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. How can you calculate the number of increasing paths that start from a cell (i, j)? Think about dynamic programming.
2. Define f(i, j) as the number of increasing paths starting from cell (i, j). Try to find how f(i, j) is related to each of f(i, j+1), f(i, j-1), f(i+1, j) and f(i-1, j).

## Solution

```rust
impl Solution {
    pub fn count_paths(black1: Vec<Vec<i32>>) -> i32 {
        let black2 = black1.len();
        let black3 = black1[0].len();
        let black4 = 1_000_000_007i64;
        
        let mut black5 = Vec::with_capacity(black2 * black3);
        for black6 in 0..black2 {
            for black7 in 0..black3 {
                black5.push((black1[black6][black7], black6, black7));
            }
        }
        
        black5.sort_unstable_by_key(|&(black8, _, _)| black8);
        
        let mut black9 = vec![vec![1i64; black3]; black2];
        let black10: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for (_, black11, black12) in black5 {
            for (black13, black14) in black10 {
                let black15 = black11 as i32 + black13;
                let black16 = black12 as i32 + black14;
                
                if black15 >= 0 && black15 < black2 as i32 && 
                   black16 >= 0 && black16 < black3 as i32 {
                    let black17 = black15 as usize;
                    let black18 = black16 as usize;
                    
                    if black1[black17][black18] > black1[black11][black12] {
                        black9[black17][black18] = (black9[black17][black18] + black9[black11][black12]) % black4;
                    }
                }
            }
        }
        
        let mut black19 = 0i64;
        for black20 in 0..black2 {
            for black21 in 0..black3 {
                black19 = (black19 + black9[black20][black21]) % black4;
            }
        }
        
        black19 as i32
    }
}
```