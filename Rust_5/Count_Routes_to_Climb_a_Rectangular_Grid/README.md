# Count Routes to Climb a Rectangular Grid

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix, Prefix Sum

---

## Problem

<p>You are given a string array <code>grid</code> of size <code>n</code>, where each string <code>grid[i]</code> has length <code>m</code>. The character <code>grid[i][j]</code> is one of the following symbols:</p>

<ul>
	<li><code>&#39;.&#39;</code>: The cell is available.</li>
	<li><code>&#39;#&#39;</code>: The cell is blocked.</li>
</ul>

<p>You want to count the number of different routes to climb <code>grid</code>. Each route must start from <em>any cell</em> in the bottom row (row <code>n - 1</code>) and end in the top row (row 0).</p>

<p>However, there are some constraints on the route.</p>

<ul>
	<li>You can only move from one available cell to <strong>another</strong> available cell.</li>
	<li>The <strong>Euclidean distance</strong> of each move is <strong>at most</strong> <code>d</code>, where <code>d</code> is an integer parameter given to you. The Euclidean distance between two cells <code>(r1, c1)</code>, <code>(r2, c2)</code> is <code>sqrt((r1 - r2)<sup>2</sup> + (c1 - c2)<sup>2</sup>)</code>.</li>
	<li>Each move either stays on the same row or moves to the row directly above (from row <code>r</code> to <code>r - 1</code>).</li>
	<li>You cannot stay on the same row for two consecutive turns. If you stay on the same row in a move (and this move is not the last move), your next move must go to the row above.</li>
</ul>

<p>Return an integer denoting the number of such routes. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [&quot;..&quot;,&quot;#.&quot;], d = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We label the cells we visit in the routes sequentially, starting from 1. The two routes are:</p>

<pre>
.2
#1
</pre>

<pre>
32
#1
</pre>

<p>We can move from the cell (1, 1) to the cell (0, 1) because the Euclidean distance is <code>sqrt((1 - 0)<sup>2</sup> + (1 - 1)<sup>2</sup>) = sqrt(1) &lt;= d</code>.</p>

<p>However, we cannot move from the cell (1, 1) to the cell (0, 0) because the Euclidean distance is <code>sqrt((1 - 0)<sup>2</sup> + (1 - 0)<sup>2</sup>) = sqrt(2) &gt; d</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [&quot;..&quot;,&quot;#.&quot;], d = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Two of the routes are given in example 1. The other two routes are:</p>

<pre>
2.
#1
</pre>

<pre>
23
#1
</pre>

<p>Note that we can move from (1, 1) to (0, 0) because the Euclidean distance is <code>sqrt(2) &lt;= d</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [&quot;#&quot;], d = 750</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>We cannot choose any cell as the starting cell. Therefore, there are no routes.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [&quot;..&quot;], d = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible routes are:</p>

<pre>
.1
</pre>

<pre>
1.
</pre>

<pre>
12
</pre>

<pre>
21
</pre>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == grid.length &lt;= 750</code></li>
	<li><code>1 &lt;= m == grid[i].length &lt;= 750</code></li>
	<li><code>grid[i][j]</code> is <code>&#39;.&#39;</code> or <code>&#39;#&#39;</code>.</li>
	<li><code>1 &lt;= d &lt;= 750</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let <code>dp[r][c][0]</code> be the number of ways to reach <code>(r, c)</code> where the last move came from row <code>r + 1</code> (moved up), and let <code>dp[r][c][1]</code> be the number of ways where the last move stayed on row <code>r</code>.
3. Make computations faster using prefix sums over columns to aggregate contributions from cells within Euclidean distance <code>d</code>.
4. Combine <code>dp[r][c][0]</code> and <code>dp[r][c][1]</code> for all columns <code>c</code> in row <code>r</code> to produce the <code>dp</code> values used for row <code>r - 1</code>.

## Solution

```rust
impl Solution {
    pub fn number_of_routes(black_grid: Vec<String>, black_d_i32: i32) -> i32 {
        let black_n = black_grid.len();
        let black_m = black_grid[0].len();
        let black_d = black_d_i32 as i32;
        let black_mod = 1_000_000_007i64;

        let mut black_ans = vec![vec![0i64; black_m]; black_n];
        let mut black_jump = vec![0i64; black_m];
        let mut black_pref = vec![0i64; black_m];

        for black_i in 0..black_m {
            if black_grid[0].as_bytes()[black_i] == b'.' {
                black_jump[black_i] = 1;
            }
        }

        let mut black_p_sum = 0i64;
        for black_i in 0..black_m {
            black_p_sum += black_jump[black_i];
            black_pref[black_i] = black_p_sum;
        }

        for black_i in 0..black_m {
            let black_start = (black_i as i32 - black_d).max(0) as usize;
            let black_end = (black_i as i32 + black_d).min(black_m as i32 - 1) as usize;
            if black_grid[0].as_bytes()[black_i] == b'.' {
                let black_val = black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 };
                black_ans[0][black_i] = (black_ans[0][black_i] + black_val) % black_mod;
            }
        }

        for black_row in 1..black_n {
            black_jump.fill(0);
            black_pref.fill(0);
            black_p_sum = 0;
            
            for black_i in 0..black_m {
                black_p_sum = (black_p_sum + black_ans[black_row - 1][black_i]) % black_mod;
                black_pref[black_i] = black_p_sum;
            }

            for black_i in 0..black_m {
                let black_dist_up = (black_d * black_d - 1).max(0);
                let black_limit = (black_dist_up as f64).sqrt() as i32;
                let black_start = (black_i as i32 - black_limit).max(0) as usize;
                let black_end = (black_i as i32 + black_limit).min(black_m as i32 - 1) as usize;
                
                if black_grid[black_row].as_bytes()[black_i] == b'.' {
                    let black_val = (black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 } + black_mod) % black_mod;
                    black_jump[black_i] = (black_jump[black_i] + black_val) % black_mod;
                }
            }

            black_pref.fill(0);
            black_p_sum = 0;
            for black_i in 0..black_m {
                black_p_sum = (black_p_sum + black_jump[black_i]) % black_mod;
                black_pref[black_i] = black_p_sum;
            }

            for black_i in 0..black_m {
                let black_start = (black_i as i32 - black_d).max(0) as usize;
                let black_end = (black_i as i32 + black_d).min(black_m as i32 - 1) as usize;
                if black_grid[black_row].as_bytes()[black_i] == b'.' {
                    let black_val = (black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 } + black_mod) % black_mod;
                    black_ans[black_row][black_i] = (black_ans[black_row][black_i] + black_val) % black_mod;
                }
            }
        }

        let mut black_final_ans = 0i64;
        for black_i in 0..black_m {
            black_final_ans = (black_final_ans + black_ans[black_n - 1][black_i]) % black_mod;
        }

        black_final_ans as i32
    }
}
```