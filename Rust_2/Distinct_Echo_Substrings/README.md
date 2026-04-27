# Distinct Echo Substrings

**Difficulty:** Hard
**Tags:** String, Trie, Rolling Hash, Hash Function

---

## Problem

<p>Return the number of <strong>distinct</strong> non-empty substrings of <code>text</code>&nbsp;that can be written as the concatenation of some string with itself (i.e. it can be written as <code>a + a</code>&nbsp;where <code>a</code> is some string).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;abcabcabc&quot;
<strong>Output:</strong> 3
<b>Explanation: </b>The 3 substrings are &quot;abcabc&quot;, &quot;bcabca&quot; and &quot;cabcab&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;leetcodeleetcode&quot;
<strong>Output:</strong> 2
<b>Explanation: </b>The 2 substrings are &quot;ee&quot; and &quot;leetcodeleetcode&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= text.length &lt;= 2000</code></li>
	<li><code>text</code>&nbsp;has only lowercase English letters.</li>
</ul>


## Hints

1. Given a substring of the text, how to check if it can be written as the concatenation of a string with itself ?
2. We can do that in linear time, a faster way is to use hashing.
3. Try all substrings and use hashing to check them.

## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let black_n = text.len();
        let black_bytes = text.as_bytes();
        let black_base: u64 = 31;
        let black_mod: u64 = 1_000_000_007;

        let mut black_hash = vec![0u64; black_n + 1];
        let mut black_pow = vec![1u64; black_n + 1];

        for black_i in 0..black_n {
            black_hash[black_i + 1] = (black_hash[black_i] * black_base + (black_bytes[black_i] - b'a' + 1) as u64) % black_mod;
            black_pow[black_i + 1] = (black_pow[black_i] * black_base) % black_mod;
        }

        let mut black_result = HashSet::new();

        for black_len in 1..=black_n / 2 {
            for black_i in 0..=black_n - 2 * black_len {
                let black_h1 = (black_hash[black_i + black_len] + black_mod - (black_hash[black_i] * black_pow[black_len]) % black_mod) % black_mod;
                let black_h2 = (black_hash[black_i + 2 * black_len] + black_mod - (black_hash[black_i + black_len] * black_pow[black_len]) % black_mod) % black_mod;

                if black_h1 == black_h2 {
                    black_result.insert(black_h1);
                }
            }
        }
        black_result.len() as i32
    }
}
```