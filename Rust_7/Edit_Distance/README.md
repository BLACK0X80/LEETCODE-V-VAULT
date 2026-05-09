# Edit Distance

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two strings <code>word1</code> and <code>word2</code>, return <em>the minimum number of operations required to convert <code>word1</code> to <code>word2</code></em>.</p>

<p>You have the following three operations permitted on a word:</p>

<ul>
	<li>Insert a character</li>
	<li>Delete a character</li>
	<li>Replace a character</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;horse&quot;, word2 = &quot;ros&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
horse -&gt; rorse (replace &#39;h&#39; with &#39;r&#39;)
rorse -&gt; rose (remove &#39;r&#39;)
rose -&gt; ros (remove &#39;e&#39;)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;intention&quot;, word2 = &quot;execution&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> 
intention -&gt; inention (remove &#39;t&#39;)
inention -&gt; enention (replace &#39;i&#39; with &#39;e&#39;)
enention -&gt; exention (replace &#39;n&#39; with &#39;x&#39;)
exention -&gt; exection (replace &#39;n&#39; with &#39;c&#39;)
exection -&gt; execution (insert &#39;u&#39;)
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= word1.length, word2.length &lt;= 500</code></li>
	<li><code>word1</code> and <code>word2</code> consist of lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (black_m, black_n) = (word1.len(), word2.len());
        let mut black_dp = vec![vec![0; black_n + 1]; black_m + 1];

        for black_i in 0..=black_m { black_dp[black_i][0] = black_i as i32; }
        for black_j in 0..=black_n { black_dp[0][black_j] = black_j as i32; }

        let black_w1 = word1.as_bytes();
        let black_w2 = word2.as_bytes();

        for black_i in 1..=black_m {
            for black_j in 1..=black_n {
                if black_w1[black_i - 1] == black_w2[black_j - 1] {
                    black_dp[black_i][black_j] = black_dp[black_i - 1][black_j - 1];
                } else {
                    black_dp[black_i][black_j] = 1 + std::cmp::min(
                        black_dp[black_i - 1][black_j - 1],
                        std::cmp::min(black_dp[black_i - 1][black_j], black_dp[black_i][black_j - 1])
                    );
                }
            }
        }
        black_dp[black_m][black_n]
    }
}
```