# Shortest Palindrome

**Difficulty:** Hard
**Tags:** String, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a string <code>s</code>. You can convert <code>s</code> to a <span data-keyword="palindrome-string">palindrome</span> by adding characters in front of it.</p>

<p>Return <em>the shortest palindrome you can find by performing this transformation</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "aacecaaa"
<strong>Output:</strong> "aaacecaaa"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = "abcd"
<strong>Output:</strong> "dcbabcd"
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>s</code> consists of lowercase English letters only.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let rev: String = s.chars().rev().collect();
        let combined = format!("{}#{}", s, rev);
        let b = combined.as_bytes();
        let n = b.len();
        let mut fail = vec![0usize; n];
        for i in 1..n {
            let mut j = fail[i - 1];
            while j > 0 && b[i] != b[j] { j = fail[j - 1]; }
            if b[i] == b[j] { j += 1; }
            fail[i] = j;
        }
        let len = fail[n - 1];
        format!("{}{}", &rev[..s.len() - len], s)
    }
}
```