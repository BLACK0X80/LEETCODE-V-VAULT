# License Key Formatting

**Difficulty:** Easy
**Tags:** String

---

## Problem

<p>You are given a license key represented as a string <code>s</code> that consists of only alphanumeric characters and dashes. The string is separated into <code>n + 1</code> groups by <code>n</code> dashes. You are also given an integer <code>k</code>.</p>

<p>We want to reformat the string <code>s</code> such that each group contains exactly <code>k</code> characters, except for the first group, which could be shorter than <code>k</code> but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.</p>

<p>Return <em>the reformatted license key</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;5F3Z-2e-9-w&quot;, k = 4
<strong>Output:</strong> &quot;5F3Z-2E9W&quot;
<strong>Explanation:</strong> The string s has been split into two parts, each part has 4 characters.
Note that the two extra dashes are not needed and can be removed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;2-5g-3-J&quot;, k = 2
<strong>Output:</strong> &quot;2-5G-3J&quot;
<strong>Explanation:</strong> The string s has been split into three parts, each part has 2 characters except the first part as it could be shorter as mentioned above.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of English letters, digits, and dashes <code>&#39;-&#39;</code>.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let black: Vec<char> = s.chars().filter(|&c| c != '-').map(|c| c.to_ascii_uppercase()).collect();
        let k = k as usize;
        let first = black.len() % k;
        let mut res = vec![];
        for (i, &c) in black.iter().enumerate() {
            if i == first && i > 0 || i > first && (i - first) % k == 0 { res.push('-'); }
            res.push(c);
        }
        res.into_iter().collect()
    }
}
```