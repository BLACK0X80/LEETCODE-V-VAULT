# Manhattan Distances of All Arrangements of Pieces

**Difficulty:** Hard
**Tags:** Math, Combinatorics

---

## Problem

<p>You are given three integers <code><font face="monospace">m</font></code>, <code><font face="monospace">n</font></code>, and <code>k</code>.</p>

<p>There is a rectangular grid of size <code>m &times; n</code> containing <code>k</code> identical pieces. Return the sum of Manhattan distances between every pair of pieces over all <strong>valid arrangements</strong> of pieces.</p>

<p>A <strong>valid arrangement</strong> is a placement of all <code>k</code> pieces on the grid with <strong>at most</strong> one piece per cell.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>The Manhattan Distance between two cells <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and <code>(x<sub>j</sub>, y<sub>j</sub>)</code> is <code>|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">m = 2, n = 2, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid arrangements of pieces on the board are:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/25/4040example1.drawio" /><img alt="" src="https://assets.leetcode.com/uploads/2024/12/25/untitled-diagramdrawio.png" style="width: 441px; height: 204px;" /></p>

<ul>
	<li>In the first 4 arrangements, the Manhattan distance between the two pieces is 1.</li>
	<li>In the last 2 arrangements, the Manhattan distance between the two pieces is 2.</li>
</ul>

<p>Thus, the total Manhattan distance across all valid arrangements is <code>1 + 1 + 1 + 1 + 2 + 2 = 8</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">m = 1, n = 4, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">20</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid arrangements of pieces on the board are:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/25/4040example2drawio.png" style="width: 762px; height: 41px;" /></p>

<ul>
	<li>The first and last arrangements have a total Manhattan distance of <code>1 + 1 + 2 = 4</code>.</li>
	<li>The middle two arrangements have a total Manhattan distance of <code>1 + 2 + 3 = 6</code>.</li>
</ul>

<p>The total Manhattan distance between all pairs of pieces across all arrangements is <code>4 + 6 + 6 + 4 = 20</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code><font face="monospace">2 &lt;= k &lt;= m * n</font></code></li>
</ul>


## Hints

1. Fix two pieces in two specific locations and find the number of boards where this can happen.
2. A particular pair of positions will be counted exactly <code>C(m * n - 2, k - 2)</code> times. Calculate the total distance for all pairs of positions and multiply it with <code>C(m * n - 2, k - 2)</code>.

## Solution

```rust
impl Solution {
    pub fn distance_sum(black_m: i32, black_n: i32, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_m = black_m as i64;
        let black_n = black_n as i64;

        let mut black_fact = vec![1i64; (black_m * black_n) as usize + 1];
        for i in 1..=(black_m * black_n) as usize {
            black_fact[i] = (black_fact[i - 1] * i as i64) % black_mod;
        }

        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }

        fn ncr(n: i64, r: i64, fact: &[i64], m: i64) -> i64 {
            if r < 0 || r > n { return 0; }
            let num = fact[n as usize];
            let den = (power(fact[r as usize], m - 2, m) * power(fact[(n - r) as usize], m - 2, m)) % m;
            (num * den) % m
        }

        let mut black_total_dist = 0i64;
        
        
        let black_sum_dist_h = black_n * (black_n * black_n - 1) / 6 % black_mod;
        let black_row_contrib = (black_sum_dist_h * black_m % black_mod) * black_m % black_mod;
        
        let black_sum_dist_v = black_m * (black_m * black_m - 1) / 6 % black_mod;
        let black_col_contrib = (black_sum_dist_v * black_n % black_mod) * black_n % black_mod;

        black_total_dist = (black_row_contrib + black_col_contrib) % black_mod;
        
        let black_arrangements = ncr(black_m * black_n - 2, black_k as i64 - 2, &black_fact, black_mod);
        (black_total_dist * black_arrangements % black_mod) as i32
    }
}
```