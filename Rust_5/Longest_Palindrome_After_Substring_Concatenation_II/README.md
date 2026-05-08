# Longest Palindrome After Substring Concatenation II

**Difficulty:** Hard
**Tags:** Two Pointers, String, Dynamic Programming

---

## Problem

<p>You are given two strings, <code>s</code> and <code>t</code>.</p>

<p>You can create a new string by selecting a <span data-keyword="substring">substring</span> from <code>s</code> (possibly empty) and a substring from <code>t</code> (possibly empty), then concatenating them <strong>in order</strong>.</p>

<p>Return the length of the <strong>longest</strong> <span data-keyword="palindrome-string">palindrome</span> that can be formed this way.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;a&quot;, t = &quot;a&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Concatenating <code>&quot;a&quot;</code> from <code>s</code> and <code>&quot;a&quot;</code> from <code>t</code> results in <code>&quot;aa&quot;</code>, which is a palindrome of length 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abc&quot;, t = &quot;def&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Since all characters are different, the longest palindrome is any single character, so the answer is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;b&quot;, t = &quot;aaaa&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Selecting &quot;<code>aaaa</code>&quot; from <code>t</code> is the longest palindrome, so the answer is 4.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abcde&quot;, t = &quot;ecdba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>Concatenating <code>&quot;abc&quot;</code> from <code>s</code> and <code>&quot;ba&quot;</code> from <code>t</code> results in <code>&quot;abcba&quot;</code>, which is a palindrome of length 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, t.length &lt;= 1000</code></li>
	<li><code>s</code> and <code>t</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. Let <code>dp[i][j]</code> be the length of the longest answer if we try starting it with <code>s[i]</code> and ending it with <code>t[j]</code>.
2. For <code>s</code>, preprocess the length of the longest palindrome starting at index <code>i</code> as <code>p[i]</code>.
3. For <code>t</code>, preprocess the length of the longest palindrome ending at index <code>j</code> as <code>q[j]</code>.
4. If <code>s[i] != t[j]</code>, then <code>dp[i][j] = max(p[i], q[j])</code>.
5. Otherwise, <code>dp[i][j] = max(p[i], q[j], 2 + dp[i + 1][j - 1])</code>.

## Solution

```rust
impl Solution {
    pub fn longest_palindrome(black_s: String, black_t: String) -> i32 {
        let black_n = black_s.len();
        let black_m = black_t.len();
        let black_s_bytes = black_s.as_bytes();
        let black_t_bytes = black_t.as_bytes();

        let mut black_max_len = 0;

        let mut black_s_pal = vec![0; black_n + 1];
        for black_i in 0..black_n {
            for black_j in black_i..black_n {
                if Self::black_check(black_s_bytes, black_i, black_j) {
                    black_s_pal[black_i] = black_s_pal[black_i].max(black_j - black_i + 1);
                    black_max_len = black_max_len.max(black_j - black_i + 1);
                }
            }
        }

        let mut black_t_pal = vec![0; black_m + 1];
        for black_i in 0..black_m {
            for black_j in black_i..black_m {
                if Self::black_check(black_t_bytes, black_i, black_j) {
                    black_t_pal[black_j + 1] = black_t_pal[black_j + 1].max(black_j - black_i + 1);
                    black_max_len = black_max_len.max(black_j - black_i + 1);
                }
            }
        }

        let mut black_t_rev = black_t_bytes.to_vec();
        black_t_rev.reverse();

        let mut black_dp = vec![vec![0; black_m + 1]; black_n + 1];
        for black_i in 1..=black_n {
            for black_j in 1..=black_m {
                if black_s_bytes[black_i - 1] == black_t_rev[black_j - 1] {
                    black_dp[black_i][black_j] = black_dp[black_i - 1][black_j - 1] + 1;
                    let black_match = black_dp[black_i][black_j];
                    let black_base = black_match * 2;
                    
                    black_max_len = black_max_len.max(black_base);
                    
                    if black_i < black_n {
                        black_max_len = black_max_len.max(black_base + black_s_pal[black_i]);
                    }
                    
                    let black_t_idx = black_m - black_j;
                    if black_t_idx > 0 {
                        black_max_len = black_max_len.max(black_base + black_t_pal[black_t_idx]);
                    }
                }
            }
        }

        black_max_len as i32
    }

    fn black_check(black_b: &[u8], mut black_l: usize, mut black_r: usize) -> bool {
        while black_l < black_r {
            if black_b[black_l] != black_b[black_r] {
                return false;
            }
            black_l += 1;
            black_r -= 1;
        }
        true
    }
}
```