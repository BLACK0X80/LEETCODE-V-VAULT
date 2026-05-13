#  Check if There Is a Valid Parentheses String Path

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>A parentheses string is a <strong>non-empty</strong> string consisting only of <code>&#39;(&#39;</code> and <code>&#39;)&#39;</code>. It is <strong>valid</strong> if <strong>any</strong> of the following conditions is <strong>true</strong>:</p>

<ul>
	<li>It is <code>()</code>.</li>
	<li>It can be written as <code>AB</code> (<code>A</code> concatenated with <code>B</code>), where <code>A</code> and <code>B</code> are valid parentheses strings.</li>
	<li>It can be written as <code>(A)</code>, where <code>A</code> is a valid parentheses string.</li>
</ul>

<p>You are given an <code>m x n</code> matrix of parentheses <code>grid</code>. A <strong>valid parentheses string path</strong> in the grid is a path satisfying <strong>all</strong> of the following conditions:</p>

<ul>
	<li>The path starts from the upper left cell <code>(0, 0)</code>.</li>
	<li>The path ends at the bottom-right cell <code>(m - 1, n - 1)</code>.</li>
	<li>The path only ever moves <strong>down</strong> or <strong>right</strong>.</li>
	<li>The resulting parentheses string formed by the path is <strong>valid</strong>.</li>
</ul>

<p>Return <code>true</code> <em>if there exists a <strong>valid parentheses string path</strong> in the grid.</em> Otherwise, return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/example1drawio.png" style="width: 521px; height: 300px;" />
<pre>
<strong>Input:</strong> grid = [[&quot;(&quot;,&quot;(&quot;,&quot;(&quot;],[&quot;)&quot;,&quot;(&quot;,&quot;)&quot;],[&quot;(&quot;,&quot;(&quot;,&quot;)&quot;],[&quot;(&quot;,&quot;(&quot;,&quot;)&quot;]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows two possible paths that form valid parentheses strings.
The first path shown results in the valid parentheses string &quot;()(())&quot;.
The second path shown results in the valid parentheses string &quot;((()))&quot;.
Note that there may be other valid parentheses string paths.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/example2drawio.png" style="width: 165px; height: 165px;" />
<pre>
<strong>Input:</strong> grid = [[&quot;)&quot;,&quot;)&quot;],[&quot;(&quot;,&quot;(&quot;]]
<strong>Output:</strong> false
<strong>Explanation:</strong> The two possible paths form the parentheses strings &quot;))(&quot; and &quot;)((&quot;. Since neither of them are valid parentheses strings, we return false.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 100</code></li>
	<li><code>grid[i][j]</code> is either <code>&#39;(&#39;</code> or <code>&#39;)&#39;</code>.</li>
</ul>


## Hints

1. What observations can you make about the number of open brackets and close brackets for any prefix of a valid bracket sequence?
2. The number of open brackets must always be greater than or equal to the number of close brackets.
3. Could you use dynamic programming?

## Solution

```rust
impl Solution {
    pub fn has_valid_path(black_grid: Vec<Vec<char>>) -> bool {
        let (black_m, black_n) = (black_grid.len(), black_grid[0].len());
        if (black_m + black_n - 1) % 2 != 0 || black_grid[0][0] == ')' { return false; }
        let black_max_bal = (black_m + black_n) / 2;
        let mut black_dp = vec![vec![vec![false; black_max_bal + 1]; black_n]; black_m];
        black_dp[0][0][1] = true;
        let bravexuneth = black_grid;
        for black_i in 0..black_m {
            for black_j in 0..black_n {
                for black_b in 0..=black_max_bal {
                    if !black_dp[black_i][black_j][black_b] { continue; }
                    for (black_di, black_dj) in [(0, 1), (1, 0)] {
                        let (black_ni, black_nj) = (black_i + black_di, black_j + black_dj);
                        if black_ni < black_m && black_nj < black_n {
                            let black_nb = if bravexuneth[black_ni][black_nj] == '(' { black_b + 1 } else { black_b.saturating_sub(1) };
                            if black_nb <= black_max_bal && (bravexuneth[black_ni][black_nj] == '(' || black_b > 0) {
                                black_dp[black_ni][black_nj][black_nb] = true;
                            }
                        }
                    }
                }
            }
        }
        black_dp[black_m - 1][black_n - 1][0]
    }
}
```