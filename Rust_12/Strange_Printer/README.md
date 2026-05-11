# Strange Printer

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>There is a strange printer with the following two special properties:</p>

<ul>
	<li>The printer can only print a sequence of <strong>the same character</strong> each time.</li>
	<li>At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.</li>
</ul>

<p>Given a string <code>s</code>, return <em>the minimum number of turns the printer needed to print it</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaabbb&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Print &quot;aaa&quot; first and then print &quot;bbb&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aba&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Print &quot;aaa&quot; first and then print &quot;b&quot; from the second place of the string, which will cover the existing character &#39;a&#39;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 100</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0i32; n]; n];

        for l in (0..n).rev() {
            dp[l][l] = 1;
            for r in l+1..n {
                dp[l][r] = dp[l][r-1] + 1;
                for m in l..r {
                    if s[m] == s[r] {
                        let left  = if m + 1 <= r - 1 { dp[m+1][r-1] } else { 0 };
                        dp[l][r] = dp[l][r].min(dp[l][m] + left);
                    }
                }
            }
        }

        dp[0][n-1]
    }
}
```