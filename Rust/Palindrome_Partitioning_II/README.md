# Palindrome Partitioning II

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given a string <code>s</code>, partition <code>s</code> such that every <span data-keyword="substring-nonempty">substring</span> of the partition is a <span data-keyword="palindrome-string">palindrome</span>.</p>

<p>Return <em>the <strong>minimum</strong> cuts needed for a palindrome partitioning of</em> <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aab&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> The palindrome partitioning [&quot;aa&quot;,&quot;b&quot;] could be produced using 1 cut.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;a&quot;
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ab&quot;
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 2000</code></li>
	<li><code>s</code> consists of lowercase English letters only.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut pal = vec![vec![false; n]; n];

        for c in 0..n {
            let mut l = c as i32;
            let mut r = c;
            while l >= 0 && r < n && s[l as usize] == s[r] { pal[l as usize][r] = true; l -= 1; r += 1; }
            l = c as i32; r = c + 1;
            while l >= 0 && r < n && s[l as usize] == s[r] { pal[l as usize][r] = true; l -= 1; r += 1; }
        }

        let mut dp = vec![0i32; n + 1];
        for i in 1..=n {
            dp[i] = i as i32 - 1;
            for j in 0..i {
                if pal[j][i - 1] {
                    dp[i] = dp[i].min(if j == 0 { 0 } else { dp[j] + 1 });
                }
            }
        }

        dp[n]
    }
}
```