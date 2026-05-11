# Longest Substring Without Repeating Characters

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>Given a string <code>s</code>, find the length of the <strong>longest</strong> <span data-keyword="substring-nonempty"><strong>substring</strong></span> without duplicate characters.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcabcbb&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The answer is &quot;abc&quot;, with the length of 3. Note that <code>&quot;bca&quot;</code> and <code>&quot;cab&quot;</code> are also correct answers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bbbbb&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> The answer is &quot;b&quot;, with the length of 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;pwwkew&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The answer is &quot;wke&quot;, with the length of 3.
Notice that the answer must be a substring, &quot;pwke&quot; is a subsequence and not a substring.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>s</code> consists of English letters, digits, symbols and spaces.</li>
</ul>


## Hints

1. Since maximum string size is at most 26, generate and check all possible substrings with length at most 26.

## Solution

```rust
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut black_map = [0; 128];
        let mut black_result = 0;
        let mut black_start = 0;
        let black_bytes = s.as_bytes();

        for (black_end, &black_char) in black_bytes.iter().enumerate() {
            black_start = cmp::max(black_start, black_map[black_char as usize]);
            black_result = cmp::max(black_result, black_end - black_start + 1);
            black_map[black_char as usize] = black_end + 1;
        }

        black_result as i32
    }
}
```