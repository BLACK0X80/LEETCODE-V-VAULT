# Minimum Number of Flips to Convert Binary Matrix to Zero Matrix

**Difficulty:** Hard
**Tags:** Array, Hash Table, Bit Manipulation, Breadth-First Search, Matrix

---

## Problem

<p>Given a <code>m x n</code> binary matrix <code>mat</code>. In one step, you can choose one cell and flip it and all the four neighbors of it if they exist (Flip is changing <code>1</code> to <code>0</code> and <code>0</code> to <code>1</code>). A pair of cells are called neighbors if they share one edge.</p>

<p>Return the <em>minimum number of steps</em> required to convert <code>mat</code> to a zero matrix or <code>-1</code> if you cannot.</p>

<p>A <strong>binary matrix</strong> is a matrix with all cells equal to <code>0</code> or <code>1</code> only.</p>

<p>A <strong>zero matrix</strong> is a matrix with all cells equal to <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/28/matrix.png" style="width: 409px; height: 86px;" />
<pre>
<strong>Input:</strong> mat = [[0,0],[0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One possible solution is to flip (1, 0) then (0, 1) and finally (1, 1) as shown.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> mat = [[0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Given matrix is a zero matrix. We do not need to change it.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,0,0],[1,0,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Given matrix cannot be a zero matrix.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n == mat[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 3</code></li>
	<li><code>mat[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Flipping same index two times is like not flipping it at all. Each index can be flipped one time. Try all possible combinations. O(2^(n*m)).

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let start: u32 = mat.iter().enumerate().flat_map(|(i,r)| r.iter().enumerate().map(move |(j,&v)| v as u32 * (1<<(i*n+j)))).sum();
        if start == 0 { return 0; }
        let mut vis = std::collections::HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((start, 0));
        vis.insert(start);
        while let Some((state, steps)) = q.pop_front() {
            for i in 0..m { for j in 0..n {
                let mut ns = state ^ (1<<(i*n+j));
                for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                    let nr=i as i32+dr; let nc=j as i32+dc;
                    if nr>=0&&nc>=0&&(nr as usize)<m&&(nc as usize)<n { ns^=1<<(nr as usize*n+nc as usize); }
                }
                if ns==0 { return steps+1; }
                if vis.insert(ns) { q.push_back((ns,steps+1)); }
            }}
        }
        -1
    }
}
```