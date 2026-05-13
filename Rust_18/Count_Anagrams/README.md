# Count Anagrams

**Difficulty:** Hard
**Tags:** Hash Table, Math, String, Combinatorics, Counting

---

## Problem

<p>You are given a string <code>s</code> containing one or more words. Every consecutive pair of words is separated by a single space <code>&#39; &#39;</code>.</p>

<p>A string <code>t</code> is an <strong>anagram</strong> of string <code>s</code> if the <code>i<sup>th</sup></code> word of <code>t</code> is a <strong>permutation</strong> of the <code>i<sup>th</sup></code> word of <code>s</code>.</p>

<ul>
	<li>For example, <code>&quot;acb dfe&quot;</code> is an anagram of <code>&quot;abc def&quot;</code>, but <code>&quot;def cab&quot;</code>&nbsp;and <code>&quot;adc bef&quot;</code> are not.</li>
</ul>

<p>Return <em>the number of <strong>distinct anagrams</strong> of </em><code>s</code>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;too hot&quot;
<strong>Output:</strong> 18
<strong>Explanation:</strong> Some of the anagrams of the given string are &quot;too hot&quot;, &quot;oot hot&quot;, &quot;oto toh&quot;, &quot;too toh&quot;, and &quot;too oht&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aa&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one anagram possible for the given string.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters and spaces <code>&#39; &#39;</code>.</li>
	<li>There is single space between consecutive words.</li>
</ul>


## Hints

1. For each word, can you count the number of permutations possible if all characters are distinct?
2. How to reduce overcounting when letters are repeated?
3. The product of the counts of distinct permutations of all words will give the final answer.

## Solution

```rust
impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_f = vec![1i64; s.len() + 1];
        let mut black_inv = vec![1i64; s.len() + 1];
        for black_i in 1..=s.len() { black_f[black_i] = (black_f[black_i - 1] * black_i as i64) % black_mod; }
        fn black_p(mut black_b: i64, mut black_e: i64, black_m: i64) -> i64 {
            let mut black_r = 1;
            while black_e > 0 {
                if black_e % 2 == 1 { black_r = (black_r * black_b) % black_m; }
                black_b = (black_b * black_b) % black_m;
                black_e /= 2;
            }
            black_r
        }
        black_inv[s.len()] = black_p(black_f[s.len()], black_mod - 2, black_mod);
        for black_i in (0..s.len()).rev() { black_inv[black_i] = (black_inv[black_i + 1] * (black_i + 1) as i64) % black_mod; }
        let mut black_ans = 1i64;
        for black_word in s.split_whitespace() {
            let mut black_cnt = [0i64; 26];
            for &black_b in black_word.as_bytes() { black_cnt[(black_b - b'a') as usize] += 1; }
            let mut black_res = black_f[black_word.len()];
            for &black_c in &black_cnt { if black_c > 1 { black_res = (black_res * black_inv[black_c as usize]) % black_mod; } }
            black_ans = (black_ans * black_res) % black_mod;
        }
        black_ans as i32
    }
}
```