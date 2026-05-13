# Spiral Matrix II

**Difficulty:** Medium
**Tags:** Array, Matrix, Simulation

---

## Problem

<p>Given a positive integer <code>n</code>, generate an <code>n x n</code> <code>matrix</code> filled with elements from <code>1</code> to <code>n<sup>2</sup></code> in spiral order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [[1,2,3],[8,9,4],[7,6,5]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> [[1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 20</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn generate_matrix(black_n: i32) -> Vec<Vec<i32>> {
        let black_n_usize = black_n as usize;
        let mut black_res = vec![vec![0; black_n_usize]; black_n_usize];
        let (mut black_r, mut black_c, mut black_dr, mut black_dc) = (0i32, 0i32, 0i32, 1i32);
        
        for black_v in 1..=black_n * black_n {
            black_res[black_r as usize][black_c as usize] = black_v;
            
            let black_next_r = black_r + black_dr;
            let black_next_c = black_c + black_dc;
            
            if black_next_r < 0 || black_next_r >= black_n || 
               black_next_c < 0 || black_next_c >= black_n || 
               black_res[black_next_r as usize][black_next_c as usize] != 0 {
                let black_tmp = black_dr;
                black_dr = black_dc;
                black_dc = -black_tmp;
            }
            
            black_r += black_dr;
            black_c += black_dc;
        }
        black_res
    }
}
```