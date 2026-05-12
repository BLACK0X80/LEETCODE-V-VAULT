# Number of Paths with Max Score

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given a square <code>board</code>&nbsp;of characters. You can move on the board starting at the bottom right square marked with the character&nbsp;<code>&#39;S&#39;</code>.</p>

<p>You need&nbsp;to reach the top left square marked with the character <code>&#39;E&#39;</code>. The rest of the squares are labeled either with a numeric character&nbsp;<code>1, 2, ..., 9</code> or with an obstacle <code>&#39;X&#39;</code>. In one move you can go up, left or up-left (diagonally) only if there is no obstacle there.</p>

<p>Return a list of two integers: the first integer is the maximum sum of numeric characters you can collect, and the second is the number of such paths that you can take to get that maximum sum, <strong>taken modulo <code>10^9 + 7</code></strong>.</p>

<p>In case there is no path, return&nbsp;<code>[0, 0]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> board = ["E23","2X2","12S"]
<strong>Output:</strong> [7,1]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> board = ["E12","1X1","21S"]
<strong>Output:</strong> [4,2]
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> board = ["E11","XXX","11S"]
<strong>Output:</strong> [0,0]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= board.length == board[i].length &lt;= 100</code></li>
</ul>

## Hints

1. Use dynamic programming to find the path with the max score.
2. Use another dynamic programming array to count the number of paths with max score.

## Solution

```rust
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        let n = board.len();
        let g: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();
        let mut dp = vec![vec![(i64::MIN, 0i64); n]; n];
        dp[n-1][n-1] = (0, 1);
        for i in (0..n).rev() { for j in (0..n).rev() {
            if g[i][j] == b'X' || g[i][j] == b'S' && (i,j) != (n-1,n-1) { continue; }
            let val = if g[i][j].is_ascii_digit() { (g[i][j]-b'0') as i64 } else { 0 };
            if (i,j) == (n-1,n-1) { continue; }
            let mut best = (i64::MIN, 0i64);
            for (ni,nj) in [(i+1,j),(i,j+1),(i+1,j+1)] {
                if ni<n && nj<n && dp[ni][nj].1>0 {
                    let s = dp[ni][nj].0 + val;
                    if s > best.0 { best = (s, dp[ni][nj].1); }
                    else if s == best.0 { best.1 = (best.1 + dp[ni][nj].1) % MOD; }
                }
            }
            if best.1 > 0 { dp[i][j] = best; }
        }}
        if dp[0][0].1 == 0 { vec![0,0] } else { vec![dp[0][0].0 as i32, dp[0][0].1 as i32] }
    }
}
```