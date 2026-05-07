# Painting a Grid With Three Different Colors

**Difficulty:** Hard
**Tags:** Dynamic Programming

---

## Problem

<p>You are given two integers <code>m</code> and <code>n</code>. Consider an <code>m x n</code> grid where each cell is initially white. You can paint each cell <strong>red</strong>, <strong>green</strong>, or <strong>blue</strong>. All cells <strong>must</strong> be painted.</p>

<p>Return<em> the number of ways to color the grid with <strong>no two adjacent cells having the same color</strong></em>. Since the answer can be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/22/colorthegrid.png" style="width: 200px; height: 50px;" />
<pre>
<strong>Input:</strong> m = 1, n = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> The three possible colorings are shown in the image above.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/22/copy-of-colorthegrid.png" style="width: 321px; height: 121px;" />
<pre>
<strong>Input:</strong> m = 1, n = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> The six possible colorings are shown in the image above.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> m = 5, n = 5
<strong>Output:</strong> 580986
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m &lt;= 5</code></li>
	<li><code>1 &lt;= n &lt;= 1000</code></li>
</ul>


## Hints

1. Represent each colored column by a bitmask based on each cell color.
2. Use bitmasks DP with state (currentCell, prevColumn).

## Solution

```rust
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_m = m as usize;
        let mut black_states = Vec::new();

        fn black_gen(idx: usize, current: Vec<i32>, m: usize, res: &mut Vec<Vec<i32>>) {
            if idx == m { res.push(current); return; }
            for c in 0..3 {
                if idx > 0 && current[idx - 1] == c { continue; }
                let mut next = current.clone();
                next.push(c);
                black_gen(idx + 1, next, m, res);
            }
        }
        black_gen(0, vec![], black_m, &mut black_states);

        let black_sz = black_states.len();
        let mut black_adj = vec![vec![]; black_sz];
        for i in 0..black_sz {
            for j in 0..black_sz {
                let mut ok = true;
                for k in 0..black_m {
                    if black_states[i][k] == black_states[j][k] { ok = false; break; }
                }
                if ok { black_adj[i].push(j); }
            }
        }

        let mut black_dp = vec![1i64; black_sz];
        for _ in 1..n {
            let mut black_next = vec![0i64; black_sz];
            for i in 0..black_sz {
                for &next_idx in &black_adj[i] {
                    black_next[next_idx] = (black_next[next_idx] + black_dp[i]) % black_mod;
                }
            }
            black_dp = black_next;
        }

        (black_dp.iter().sum::<i64>() % black_mod) as i32
    }
}
```