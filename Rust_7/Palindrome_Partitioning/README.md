# Palindrome Partitioning

**Difficulty:** Medium
**Tags:** String, Dynamic Programming, Backtracking

---

## Problem

<p>Given a string <code>s</code>, partition <code>s</code> such that every <span data-keyword="substring-nonempty">substring</span> of the partition is a <span data-keyword="palindrome-string"><strong>palindrome</strong></span>. Return <em>all possible palindrome partitioning of </em><code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "aab"
<strong>Output:</strong> [["a","a","b"],["aa","b"]]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = "a"
<strong>Output:</strong> [["a"]]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 16</code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut black_res = Vec::new();
        let mut black_path = Vec::new();
        Self::black_dfs(&s, 0, &mut black_path, &mut black_res);
        black_res
    }

    fn black_dfs(black_s: &str, black_start: usize, black_path: &mut Vec<String>, black_res: &mut Vec<Vec<String>>) {
        if black_start == black_s.len() {
            black_res.push(black_path.clone());
            return;
        }
        for black_end in black_start + 1..=black_s.len() {
            let black_sub = &black_s[black_start..black_end];
            if Self::black_is_pali(black_sub) {
                black_path.push(black_sub.to_string());
                Self::black_dfs(black_s, black_end, black_path, black_res);
                black_path.pop();
            }
        }
    }

    fn black_is_pali(black_sub: &str) -> bool {
        let black_b = black_sub.as_bytes();
        let (mut black_l, mut black_r) = (0, black_b.len() as i32 - 1);
        while black_l < black_r {
            if black_b[black_l as usize] != black_b[black_r as usize] { return false; }
            black_l += 1;
            black_r -= 1;
        }
        true
    }
}
```