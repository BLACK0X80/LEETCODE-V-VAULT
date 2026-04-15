# Palindrome Partitioning IV

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given a string <code>s</code>, return <code>true</code> <em>if it is possible to split the string</em> <code>s</code> <em>into three <strong>non-empty</strong> palindromic substrings. Otherwise, return </em><code>false</code>.​​​​​</p>

<p>A string is said to be palindrome if it the same string when reversed.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcbdd&quot;
<strong>Output:</strong> true
<strong>Explanation: </strong>&quot;abcbdd&quot; = &quot;a&quot; + &quot;bcb&quot; + &quot;dd&quot;, and all three substrings are palindromes.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bcbddxy&quot;
<strong>Output:</strong> false
<strong>Explanation: </strong>s cannot be split into 3 palindromes.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 2000</code></li>
	<li><code>s</code>​​​​​​ consists only of lowercase English letters.</li>
</ul>


## Hints

1. Preprocess checking palindromes in O(1)
2. Note that one string is a prefix and another one is a suffix you can try brute forcing the rest

## Solution

```rust
impl Solution {
    pub fn check_partitioning(black_s: String) -> bool {
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_is_pal = vec![vec![false; black_n]; black_n];

        for black_len in 1..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                if black_len == 1 {
                    black_is_pal[black_i][black_j] = true;
                } else if black_len == 2 {
                    black_is_pal[black_i][black_j] = black_bytes[black_i] == black_bytes[black_j];
                } else {
                    black_is_pal[black_i][black_j] = (black_bytes[black_i] == black_bytes[black_j]) && black_is_pal[black_i + 1][black_j - 1];
                }
            }
        }

        for black_i in 1..black_n - 1 {
            for black_j in black_i..black_n - 1 {
                if black_is_pal[0][black_i - 1] && black_is_pal[black_i][black_j] && black_is_pal[black_j + 1][black_n - 1] {
                    return true;
                }
            }
        }
        false
    }
}
```