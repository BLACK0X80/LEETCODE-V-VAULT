# Longest Happy Prefix

**Difficulty:** Hard
**Tags:** String, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>A string is called a <strong>happy prefix</strong> if is a <strong>non-empty</strong> prefix which is also a suffix (excluding itself).</p>

<p>Given a string <code>s</code>, return <em>the <strong>longest happy prefix</strong> of</em> <code>s</code>. Return an empty string <code>&quot;&quot;</code> if no such prefix exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;level&quot;
<strong>Output:</strong> &quot;l&quot;
<strong>Explanation:</strong> s contains 4 prefix excluding itself (&quot;l&quot;, &quot;le&quot;, &quot;lev&quot;, &quot;leve&quot;), and suffix (&quot;l&quot;, &quot;el&quot;, &quot;vel&quot;, &quot;evel&quot;). The largest prefix which is also suffix is given by &quot;l&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ababab&quot;
<strong>Output:</strong> &quot;abab&quot;
<strong>Explanation:</strong> &quot;abab&quot; is the largest prefix which is also suffix. They can overlap in the original string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
</ul>


## Hints

1. Use Longest Prefix Suffix (KMP-table) or String Hashing.

## Solution

```rust
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut fail = vec![0usize; n];
        for i in 1..n {
            let mut j = fail[i - 1];
            while j > 0 && b[i] != b[j] { j = fail[j - 1]; }
            if b[i] == b[j] { j += 1; }
            fail[i] = j;
        }
        let len = fail[n - 1];
        if len == 0 { return String::new(); }
        s[..len].to_string()
    }
}
```