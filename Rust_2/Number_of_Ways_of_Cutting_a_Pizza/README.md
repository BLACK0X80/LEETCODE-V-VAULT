# Number of Ways of Cutting a Pizza

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Memoization, Matrix, Prefix Sum

---

## Problem

<p>Given a rectangular pizza represented as a <code>rows x cols</code>&nbsp;matrix containing the following characters: <code>&#39;A&#39;</code> (an apple) and <code>&#39;.&#39;</code> (empty cell) and given the integer <code>k</code>. You have to cut the pizza into <code>k</code> pieces using <code>k-1</code> cuts.&nbsp;</p>

<p>For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.</p>

<p><em>Return the number of ways of cutting the pizza such that each piece contains <strong>at least</strong> one apple.&nbsp;</em>Since the answer can be a huge number, return this modulo 10^9 + 7.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png" style="width: 500px; height: 378px;" /></strong></p>

<pre>
<strong>Input:</strong> pizza = [&quot;A..&quot;,&quot;AAA&quot;,&quot;...&quot;], k = 3
<strong>Output:</strong> 3 
<strong>Explanation:</strong> The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> pizza = [&quot;A..&quot;,&quot;AA.&quot;,&quot;...&quot;], k = 3
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> pizza = [&quot;A..&quot;,&quot;A..&quot;,&quot;...&quot;], k = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rows, cols &lt;= 50</code></li>
	<li><code>rows ==&nbsp;pizza.length</code></li>
	<li><code>cols ==&nbsp;pizza[i].length</code></li>
	<li><code>1 &lt;= k &lt;= 10</code></li>
	<li><code>pizza</code> consists of characters <code>&#39;A&#39;</code>&nbsp;and <code>&#39;.&#39;</code> only.</li>
</ul>


## Hints

1. Note that after each cut the remaining piece of pizza always has the lower right coordinate at (rows-1,cols-1).
2. Use dynamic programming approach with states (row1, col1, c) which computes the number of ways of cutting the pizza using "c" cuts where the current piece of pizza has upper left coordinate at (row1,col1) and lower right coordinate at (rows-1,cols-1).
3. For the transitions try all vertical and horizontal cuts such that the piece of pizza you have to give a person must contain at least one apple. The base case is when c=k-1.
4. Additionally use a 2D dynamic programming to respond in O(1) if a piece of pizza contains at least one apple.

## Solution

```rust
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let rows = pizza.len();
        let cols = pizza[0].len();
        let mut pref = vec![vec![0; cols + 1]; rows + 1];
        for r in 0..rows {
            for c in 0..cols {
                pref[r+1][c+1] = pref[r][c+1] + pref[r+1][c] - pref[r][c] + (if pizza[r].as_bytes()[c] == b'A' { 1 } else { 0 });
            }
        }
        let apples = |r1: usize, c1: usize, r2: usize, c2: usize| {
            pref[r2+1][c2+1] - pref[r1][c2+1] - pref[r2+1][c1] + pref[r1][c1]
        };
        let mod_val = 1_000_000_007i32;
        let mut dp = vec![vec![0i32; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                dp[r][c] = if apples(r, c, rows-1, cols-1) > 0 { 1 } else { 0 };
            }
        }
        for _ in 2..=k {
            let mut new_dp = vec![vec![0i32; cols]; rows];
            for r in 0..rows {
                for c in 0..cols {
                    let mut ways = 0i32;
                    for i in r+1..rows {
                        if apples(r, c, i-1, cols-1) > 0 { ways = (ways + dp[i][c]) % mod_val; }
                    }
                    for j in c+1..cols {
                        if apples(r, c, rows-1, j-1) > 0 { ways = (ways + dp[r][j]) % mod_val; }
                    }
                    new_dp[r][c] = ways;
                }
            }
            dp = new_dp;
        }
        dp[0][0]
    }
}
```