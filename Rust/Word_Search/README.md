# Word Search

**Difficulty:** Medium
**Tags:** Array, String, Backtracking, Depth-First Search, Matrix

---

## Problem

<p>Given an <code>m x n</code> grid of characters <code>board</code> and a string <code>word</code>, return <code>true</code> <em>if</em> <code>word</code> <em>exists in the grid</em>.</p>

<p>The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" style="width: 322px; height: 242px;" />
<pre>
<strong>Input:</strong> board = [[&quot;A&quot;,&quot;B&quot;,&quot;C&quot;,&quot;E&quot;],[&quot;S&quot;,&quot;F&quot;,&quot;C&quot;,&quot;S&quot;],[&quot;A&quot;,&quot;D&quot;,&quot;E&quot;,&quot;E&quot;]], word = &quot;ABCCED&quot;
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg" style="width: 322px; height: 242px;" />
<pre>
<strong>Input:</strong> board = [[&quot;A&quot;,&quot;B&quot;,&quot;C&quot;,&quot;E&quot;],[&quot;S&quot;,&quot;F&quot;,&quot;C&quot;,&quot;S&quot;],[&quot;A&quot;,&quot;D&quot;,&quot;E&quot;,&quot;E&quot;]], word = &quot;SEE&quot;
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/word3.jpg" style="width: 322px; height: 242px;" />
<pre>
<strong>Input:</strong> board = [[&quot;A&quot;,&quot;B&quot;,&quot;C&quot;,&quot;E&quot;],[&quot;S&quot;,&quot;F&quot;,&quot;C&quot;,&quot;S&quot;],[&quot;A&quot;,&quot;D&quot;,&quot;E&quot;,&quot;E&quot;]], word = &quot;ABCB&quot;
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == board.length</code></li>
	<li><code>n = board[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 6</code></li>
	<li><code>1 &lt;= word.length &lt;= 15</code></li>
	<li><code>board</code> and <code>word</code> consists of only lowercase and uppercase English letters.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> Could you use search pruning to make your solution faster with a larger <code>board</code>?</p>



## Solution

```rust
impl Solution {
    pub fn exist(mut black_b: Vec<Vec<char>>, black_w: String) -> bool {
        fn black_f(black_r: usize, black_c: usize, black_k: usize, black_b: &mut Vec<Vec<char>>, black_s: &[u8]) -> bool {
            if black_k == black_s.len() { true } else if black_r >= black_b.len() || black_c >= black_b[0].len() || black_b[black_r][black_c] != black_s[black_k] as char { false } else {
                let black_v = black_b[black_r][black_c]; black_b[black_r][black_c] = '\0';
                let black_res = black_f(black_r + 1, black_c, black_k + 1, black_b, black_s) || black_f(black_r.wrapping_sub(1), black_c, black_k + 1, black_b, black_s) || black_f(black_r, black_c + 1, black_k + 1, black_b, black_s) || black_f(black_r, black_c.wrapping_sub(1), black_k + 1, black_b, black_s);
                black_b[black_r][black_c] = black_v; black_res
            }
        }
        (0..black_b.len()).any(|r| (0..black_b[0].len()).any(|c| black_f(r, c, 0, &mut black_b, black_w.as_bytes())))
    }
}
```