# Longest Palindromic Substring

**Difficulty:** Medium
**Tags:** Two Pointers, String, Dynamic Programming

---

## Problem

<p>Given a string <code>s</code>, return <em>the longest</em> <span data-keyword="palindromic-string"><em>palindromic</em></span> <span data-keyword="substring-nonempty"><em>substring</em></span> in <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;babad&quot;
<strong>Output:</strong> &quot;bab&quot;
<strong>Explanation:</strong> &quot;aba&quot; is also a valid answer.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cbbd&quot;
<strong>Output:</strong> &quot;bb&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consist of only digits and English letters.</li>
</ul>


## Hints

1. How can we reuse a previously computed palindrome to compute a larger palindrome?
2. If “aba” is a palindrome, is “xabax” a palindrome? Similarly is “xabay” a palindrome?
3. Complexity based hint:</br>
If we use brute-force and check whether for every start and end position a substring is a palindrome we have O(n^2) start - end pairs and O(n) palindromic checks. Can we reduce the time for palindromic checks to O(1) by reusing some previous computation.

## Solution

```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() { return s; }
        let black_bytes = s.as_bytes();
        let (mut black_start, mut black_end) = (0, 0);

        for black_i in 0..black_bytes.len() {
            let black_len1 = Self::black_expand(&black_bytes, black_i as i32, black_i as i32);
            let black_len2 = Self::black_expand(&black_bytes, black_i as i32, black_i as i32 + 1);
            let black_len = std::cmp::max(black_len1, black_len2);

            if black_len > (black_end - black_start) as i32 {
                black_start = black_i - (black_len as usize - 1) / 2;
                black_end = black_i + black_len as usize / 2;
            }
        }
        s[black_start..=black_end].to_string()
    }

    fn black_expand(black_s: &[u8], mut black_l: i32, mut black_r: i32) -> i32 {
        while black_l >= 0 && black_r < black_s.len() as i32 && black_s[black_l as usize] == black_s[black_r as usize] {
            black_l -= 1;
            black_r += 1;
        }
        black_r - black_l - 1
    }
}
```