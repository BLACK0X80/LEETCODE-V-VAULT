# Minimum White Tiles After Covering With Carpets

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed binary</strong> string <code>floor</code>, which represents the colors of tiles on a floor:</p>

<ul>
	<li><code>floor[i] = &#39;0&#39;</code> denotes that the <code>i<sup>th</sup></code> tile of the floor is colored <strong>black</strong>.</li>
	<li>On the other hand, <code>floor[i] = &#39;1&#39;</code> denotes that the <code>i<sup>th</sup></code> tile of the floor is colored <strong>white</strong>.</li>
</ul>

<p>You are also given <code>numCarpets</code> and <code>carpetLen</code>. You have <code>numCarpets</code> <strong>black</strong> carpets, each of length <code>carpetLen</code> tiles. Cover the tiles with the given carpets such that the number of <strong>white</strong> tiles still visible is <strong>minimum</strong>. Carpets may overlap one another.</p>

<p>Return <em>the <strong>minimum</strong> number of white tiles still visible.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/02/10/ex1-1.png" style="width: 400px; height: 73px;" />
<pre>
<strong>Input:</strong> floor = &quot;10110101&quot;, numCarpets = 2, carpetLen = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
The figure above shows one way of covering the tiles with the carpets such that only 2 white tiles are visible.
No other way of covering the tiles with the carpets can leave less than 2 white tiles visible.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/02/10/ex2.png" style="width: 353px; height: 123px;" />
<pre>
<strong>Input:</strong> floor = &quot;11111&quot;, numCarpets = 2, carpetLen = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> 
The figure above shows one way of covering the tiles with the carpets such that no white tiles are visible.
Note that the carpets are able to overlap one another.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= carpetLen &lt;= floor.length &lt;= 1000</code></li>
	<li><code>floor[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
	<li><code>1 &lt;= numCarpets &lt;= 1000</code></li>
</ul>


## Hints

1. Can you think of a DP solution?
2. Let DP[i][j] denote the minimum number of white tiles still visible from indices i to floor.length-1 after covering with at most j carpets.
3. The transition will be whether to put down the carpet at position i (if possible), or not.

## Solution

```rust
impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let black_n = floor.len();
        let black_m = num_carpets as usize;
        let black_len = carpet_len as usize;
        let black_f = floor.as_bytes();
        let mut black_dp = vec![vec![0; black_n + 1]; black_m + 1];
        for black_j in 1..=black_n {
            black_dp[0][black_j] = black_dp[0][black_j - 1] + (if black_f[black_j - 1] == b'1' { 1 } else { 0 });
        }
        for black_i in 1..=black_m {
            for black_j in 1..=black_n {
                let black_skip = black_dp[black_i][black_j - 1] + (if black_f[black_j - 1] == b'1' { 1 } else { 0 });
                let black_cover = black_dp[black_i - 1][black_j.saturating_sub(black_len)];
                black_dp[black_i][black_j] = black_skip.min(black_cover);
            }
        }
        black_dp[black_m][black_n]
    }
}
```