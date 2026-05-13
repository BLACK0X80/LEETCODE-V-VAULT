# Longest Common Prefix

**Difficulty:** Easy
**Tags:** Array, String, Trie

---

## Problem

<p>Write a function to find the longest common prefix string amongst an array of strings.</p>

<p>If there is no common prefix, return an empty string <code>&quot;&quot;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> strs = [&quot;flower&quot;,&quot;flow&quot;,&quot;flight&quot;]
<strong>Output:</strong> &quot;fl&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> strs = [&quot;dog&quot;,&quot;racecar&quot;,&quot;car&quot;]
<strong>Output:</strong> &quot;&quot;
<strong>Explanation:</strong> There is no common prefix among the input strings.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= strs.length &lt;= 200</code></li>
	<li><code>0 &lt;= strs[i].length &lt;= 200</code></li>
	<li><code>strs[i]</code> consists of only lowercase English letters if it is non-empty.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = strs[0].as_bytes();
        let mut len = first.len();
        for s in &strs[1..] {
            len = len.min(s.len());
            for (i, &b) in s.as_bytes()[..len].iter().enumerate() {
                if b != first[i] { len = i; break; }
            }
        }
        String::from_utf8(first[..len].to_vec()).unwrap()
    }
}
```