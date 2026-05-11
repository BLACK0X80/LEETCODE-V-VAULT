# Shortest Common Supersequence 

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two strings <code>str1</code> and <code>str2</code>, return <em>the shortest string that has both </em><code>str1</code><em> and </em><code>str2</code><em> as <strong>subsequences</strong></em>. If there are multiple valid strings, return <strong>any</strong> of them.</p>

<p>A string <code>s</code> is a <strong>subsequence</strong> of string <code>t</code> if deleting some number of characters from <code>t</code> (possibly <code>0</code>) results in the string <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> str1 = &quot;abac&quot;, str2 = &quot;cab&quot;
<strong>Output:</strong> &quot;cabac&quot;
<strong>Explanation:</strong> 
str1 = &quot;abac&quot; is a subsequence of &quot;cabac&quot; because we can delete the first &quot;c&quot;.
str2 = &quot;cab&quot; is a subsequence of &quot;cabac&quot; because we can delete the last &quot;ac&quot;.
The answer provided is the shortest such string that satisfies these properties.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> str1 = &quot;aaaaaaaa&quot;, str2 = &quot;aaaaaaaa&quot;
<strong>Output:</strong> &quot;aaaaaaaa&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= str1.length, str2.length &lt;= 1000</code></li>
	<li><code>str1</code> and <code>str2</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. We can find the length of the longest common subsequence between str1[i:] and str2[j:] (for all (i, j)) by using dynamic programming.
2. We can use this information to recover the shortest common supersequence.

## Solution

```rust
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (a, b) = (str1.as_bytes(), str2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0u16; n+1]; m+1];

        for i in 1..=m { dp[i][0] = i as u16; }
        for j in 1..=n { dp[0][j] = j as u16; }

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i-1] == b[j-1] {
                    dp[i-1][j-1] + 1
                } else {
                    dp[i-1][j].min(dp[i][j-1]) + 1
                };
            }
        }

        let mut res = Vec::new();
        let (mut i, mut j) = (m, n);
        while i > 0 && j > 0 {
            if a[i-1] == b[j-1] {
                res.push(a[i-1]);
                i -= 1; j -= 1;
            } else if dp[i-1][j] < dp[i][j-1] {
                res.push(a[i-1]);
                i -= 1;
            } else {
                res.push(b[j-1]);
                j -= 1;
            }
        }
        while i > 0 { res.push(a[i-1]); i -= 1; }
        while j > 0 { res.push(b[j-1]); j -= 1; }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}
```