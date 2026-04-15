# Lexicographically Smallest String After Adjacent Removals

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a string <code>s</code> consisting of lowercase English letters.</p>

<p>You can perform the following operation any number of times (including zero):</p>

<ul>
	<li>Remove <strong>any</strong> pair of <strong>adjacent</strong> characters in the string that are <strong>consecutive</strong> in the alphabet, in either order (e.g., <code>&#39;a&#39;</code> and <code>&#39;b&#39;</code>, or <code>&#39;b&#39;</code> and <code>&#39;a&#39;</code>).</li>
	<li>Shift the remaining characters to the left to fill the gap.</li>
</ul>

<p>Return the <strong><span data-keyword="lexicographically-smaller-string">lexicographically smallest</span></strong> string that can be obtained after performing the operations optimally.</p>

<p><strong>Note:</strong> Consider the alphabet as circular, thus <code>&#39;a&#39;</code> and <code>&#39;z&#39;</code> are consecutive.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;a&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>&quot;bc&quot;</code> from the string, leaving <code>&quot;a&quot;</code> as the remaining string.</li>
	<li>No further operations are possible. Thus, the lexicographically smallest string after all possible removals is <code>&quot;a&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;bcda&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><strong>​​​​​​​</strong>Remove <code>&quot;cd&quot;</code> from the string, leaving <code>&quot;ba&quot;</code> as the remaining string.</li>
	<li>Remove <code>&quot;ba&quot;</code> from the string, leaving <code>&quot;&quot;</code> as the remaining string.</li>
	<li>No further operations are possible. Thus, the lexicographically smallest string after all possible removals is <code>&quot;&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;zdce&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;zdce&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>&quot;dc&quot;</code> from the string, leaving <code>&quot;ze&quot;</code> as the remaining string.</li>
	<li>No further operations are possible on <code>&quot;ze&quot;</code>.</li>
	<li>However, since <code>&quot;zdce&quot;</code> is lexicographically smaller than <code>&quot;ze&quot;</code>, the smallest string after all possible removals is <code>&quot;zdce&quot;</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 250</code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. As a result of the operation, some of the substrings can be removed
2. Find out using DP, which substrings can we remove
3. Now, try to build the ans using this DP
4. Define ans[i] = lex smallest string that can be made in [i, n - 1], then ans[i] = lex_smallest of { choose one char s[j] in [i, n - 1] + ans[j + 1] }

## Solution

```rust
impl Solution {
    pub fn lexicographically_smallest_string(black_s: String) -> String {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_dp = vec![vec![String::new(); black_n + 1]; black_n + 1];

        fn black_is_consec(a: u8, b: u8) -> bool {
            let black_diff = (a as i16 - b as i16).abs();
            black_diff == 1 || black_diff == 25
        }

        for black_len in 1..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len;
                
                let mut black_res = format!("{}{}", black_b[black_i] as char, black_dp[black_i + 1][black_j]);

                for black_k in black_i + 1..black_j {
                    if black_is_consec(black_b[black_i], black_b[black_k]) && black_dp[black_i + 1][black_k].is_empty() {
                        let black_candidate = black_dp[black_k + 1][black_j].clone();
                        if black_candidate < black_res {
                            black_res = black_candidate;
                        }
                    }
                }
                black_dp[black_i][black_j] = black_res;
            }
        }

        black_dp[0][black_n].clone()
    }
}
```